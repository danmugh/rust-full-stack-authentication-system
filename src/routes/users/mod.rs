use actix_web::web::service;

mod register;
mod confirm_registration;
mod login;
mod logout;
mod update_user;
mod generate_new_token;

pub fn auth_routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_web::web::scope("/users")
    .service(register::register_user))
    .service(confirm_registration::confirm)
    .service(login::login_user)
    .service(update_user::update_users_details)
    .service(generate_new_token::regenerate_token)
    .service(logout::log_out)
}