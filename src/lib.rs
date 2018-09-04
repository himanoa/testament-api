#![feature(try_trait)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate url;
#[macro_use]
extern crate failure;
extern crate reqwest;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate rocket_contrib;
extern crate base64;
extern crate jsonwebtoken;
extern crate oauth2;
extern crate time;
extern crate uuid;

pub mod api;
pub mod responses;
pub mod infra;
pub mod models;
pub mod services;

pub fn create_rocket(database_url: &str) -> rocket::Rocket {
    rocket::ignite()
        .mount("/auth", routes![api::auth::login])
        .catch(catchers![
            responses::bad_request,
            responses::unauthorized,
            responses::payment_required,
            responses::forbidden,
            responses::not_found,
            responses::method_not_allowed,
            responses::not_acceptable,
            responses::proxy_authentication_required,
            responses::request_timeout,
            responses::confrict,
            responses::gone,
            responses::length_required,
            responses::procondition_failed,
            responses::payload_too_large,
            responses::internal_error,
            responses::not_implemented,
            responses::bad_gateway,
            responses::service_unavailable,
            responses::gateway_timeout
        ])
}
