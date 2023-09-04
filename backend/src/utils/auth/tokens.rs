use argon2::password_hash::rand_core::{OsRng, RngCore};
use core::convert::TryFrom;
use deadpool_redis::redis::AsyncCommands;
use hex;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::{local, version4::V4, Local};

const SESSION_KEY_PREFIX: &str = "valid_session_key_for_{}";

#[tracing::instrument(name = "Issue pasetors token", skip(redis_connection))]
pub async fn issue_confirmation_token_pasetors(
    user_id: uuid::Uuid,
    redis_connection: &mut deadpool_redis::redis::aio::Connection,
    is_for_password_change: Option<bool>,
) -> Result<String, deadpool_redis::redis::RedisError> {
    let session_key: String = {
        let mut buff = [0_u8; 128];
        OsRng.fill_bytes(&mut buff);
        hex::encode(buff)
    };

    let redis_key = {
        if is_for_password_change.is_some() {
            format!(
                "{}{}is_for_password_change",
                SESSION_KEY_PREFIX, session_key
            )
        } else {
            format!("{}{}", SESSION_KEY_PREFIX, session_key)
        }
    };

    redis_connection
        .set(
            redis_key.clone(),
            String::new(),
        )
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "RedisError (set): {}", e);
            e
        })?;

    let settings = crate::settings::get_settings().expect("Cannot load settings.");
    let current_date_time = chrono::Local::now();
    let dt = {
        if is_for_password_change.is_some() {
            current_date_time + chrono::Duration::hours(1)
        } else {
            current_date_time + chrono::Duration::minutes(settings.secret.token_expiration)
        }
    };

    let time_to_live = {
        if is_for_password_change.is_some() {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::minutes(settings.secret.token_expiration)
        }
    };

    redis_connection
        .expire(
            redis_key.clone(),
            time_to_live.num_seconds().try_into().unwrap(),
        )
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "RedisError (expiry): {}", e);
            e
        })?;

    let mut claims = Claims::new().unwrap();
    claims.expiration(&dt.to_rfc3339()).unwrap();
    claims
        .add_additional("user_id", serde_json::json!(user_id))
        .unwrap();
    claims
        .add_additional("session_key", serde_json::json!(session_key))
        .unwrap();

    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();
    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )
    .unwrap())
}

#[tracing::instrument(name = "Verify pasetors token", skip(token, redis_connection))]
pub async fn verify_confirmation_token_pasetor(
    token: String,
    redis_connection: &mut deadpool_redis::redis::aio::Connection,
    is_password: Option<bool>,
) -> Result<crate::types::ConfirmationToken, String> {
    let settings = crate::settings::get_settings().expect("Cannot load settings.");
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&token)
        .map_err(|e| format!("TokenValiation: {}", e))?;
    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )
    .map_err(|e| format!("Pasetor: {}", e))?;
    let claims = trusted_token.payload_claims().unwrap();

    let uid = serde_json::to_value(claims.get_claim("user_id").unwrap()).unwrap();

    match serde_json::from_value::<String>(uid) {
        Ok(uuid_string) => match uuid::Uuid::parse_str(&uuid_string) {
            Ok(user_uuid) => {
                let sss_key =
                    serde_json::to_value(claims.get_claim("session_key").unwrap()).unwrap();
                let session_key = match serde_json::from_value::<String>(sss_key) {
                    Ok(session_key) => session_key,
                    Err(e) => return Err(format!("{}", e)),
                };

                let redis_key = {
                    if is_password.is_some() {
                        format!(
                            "{}{}is_for_password_change",
                            SESSION_KEY_PREFIX, session_key
                        )
                    } else {
                        format!("{}{}", SESSION_KEY_PREFIX, session_key)
                    }
                };

                if redis_connection
                    .get::<_, Option<String>>(redis_key.clone())
                    .await
                    .map_err(|e| format!("{}", e))?
                    .is_none()
                {
                    return Err("Token has been used or expired.".to_string());
                }
                redis_connection
                    .del(redis_key.clone())
                    .await
                    .map_err(|e| format!("{}", e))?;
                Ok(crate::types::ConfirmationToken { user_id: user_uuid })
            }
            Err(e) => Err(format!("{}", e)),
        },

        Err(e) => Err(format!("{}", e)),
    }
}