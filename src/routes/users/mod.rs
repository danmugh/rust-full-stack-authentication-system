use actix_web::web::service;

mod register;
mod confirm_registration;
mod login;

pub fn auth_routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_web::web::scope("/users")
    .service(register::register_user))
    .service(confirm_registration::activate_new_user)
    .service(login::get_user_who_is_active)
}