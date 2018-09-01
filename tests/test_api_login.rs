
#![feature(use_extern_macros, proc_macro_gen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;
extern crate serde_json;
#[cfg(test)]
extern crate speculate;
extern crate testament;

mod common;

use rocket::http::{ContentType, Status};
use rocket::local::Client;
use speculate::speculate;

use testament::create_rocket;

use common::setup;
use std::env::var;

speculate! {
    before {
        setup();
        let rocket = create_rocket(var("TEST_DATABASE_URL").unwrap().as_ref());
        let client = Client::new(rocket).unwrap();
    }

    describe "/login" {
        it "should be http status is SeeOther" {
            let res = client.get("/auth/login")
                .header(ContentType::JSON)
                .dispatch();
            assert_eq!(res.status(), Status::SeeOther);
        }
    }
}
