#![feature(use_extern_macros, proc_macro_gen)]

use common::setup;
use speculate::speculate;
use std::env::var;
use testament::infra::mysql::init_mysql_pool;
use testament::infra::user::mysql_user_repository::MySqlUserRepository;
use testament::models::user::{NewUser, User, UserRepository};
use testament::schema::users::dsl::*;

use diesel::insert_into;
use diesel::prelude::*;

speculate! {
    before {
        setup();
        let conn = init_mysql_pool(var("TEST_DATABASE_URL").unwrap().as_str()).get().unwrap();
        let target_repository = MySqlUserRepository::new(&conn);
    }

    describe "resolve" {
        it "should be fetched user" {
            let expected_user_name = "himanoa".to_string();
            insert_into(users).values(NewUser { name: "himanoa".to_string(), uid: "foobar".to_string() }).execute(&*conn);
            let actual = target_repository.resolve(1);
            assert!(actual.is_ok());
            assert_eq!(actual.unwrap().name, expected_user_name);
        }
    }

    describe "find_by_uid" {
        it "should be find by user uid" {
            let expected = "happy";
            insert_into(users).values(NewUser { name: "himanoa".to_string(), uid: expected.to_string() }).execute(&*conn);

            let actual = target_repository.find_by_uid(&expected);
            assert!(actual.is_ok());
            assert_eq!(actual.unwrap().uid, expected);
        }
    }
}
