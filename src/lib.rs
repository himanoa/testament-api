#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate diesel;
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate rocket_contrib;
// pub mod api;
pub mod error_handlers;
pub mod helpers;
pub mod models;
pub mod schema;

use helpers::mysql::init_mysql_pool;

pub fn create_rocket(database_url: &str) -> rocket::Rocket {
    rocket::ignite()
        .manage(init_mysql_pool(database_url))
        .catch(catchers![
            error_handlers::bad_request,
            error_handlers::unauthorized,
            error_handlers::payment_required,
            error_handlers::forbidden,
            error_handlers::not_found,
            error_handlers::method_not_allowed,
            error_handlers::not_acceptable,
            error_handlers::proxy_authentication_required,
            error_handlers::request_timeout,
            error_handlers::confrict,
            error_handlers::gone,
            error_handlers::length_required,
            error_handlers::procondition_failed,
            error_handlers::payload_too_large,
            error_handlers::internal_error,
            error_handlers::not_implemented,
            error_handlers::bad_gateway,
            error_handlers::service_unavailable,
            error_handlers::gateway_timeout
        ])
}
