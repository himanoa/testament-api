#![feature(use_extern_macros, proc_macro_gen)]

extern crate diesel;
extern crate parking_lot;
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

    it "should be returned 404 response" {
        let res = client.post("/asdiasdfhasuidfh")
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(res.status(), Status::NotFound);
    }
}
