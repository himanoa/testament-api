#![feature(use_extern_macros, proc_macro_gen)]

use common::setup;
use speculate::speculate;
use std::env::var;
use testament::infra::mysql::init_mysql_pool;
use testament::models::token::{NewToken, Token, TokenRepository};
use testament::infra::token::mysql_token_repository::{MySqlTokenRepository};
use testament::schema::tokens;
use testament::schema::tokens::dsl::*;

use diesel::insert_into;
use diesel::prelude::*;

speculate! {
    before {
        setup();
        let conn = init_mysql_pool(var("TEST_DATABASE_URL").unwrap().as_str()).get().unwrap();
        let target_repository = MySqlTokenRepository::new(&conn);
    }

    describe "resolve" {
        it "should be fetched token" {
            let expected_token = "hogefugafoobar".to_string();
            insert_into(tokens).values(NewToken { user_id: 1, token: expected_token.clone()}).execute(&*conn);
            let actual = target_repository.resolve(expected_token.clone());
            assert!(actual.is_ok());
            assert_eq!(actual.unwrap().token, expected_token);
        }
    }

    describe "create" {
        it "should be write to db" {
            let expected_new_token = NewToken { user_id: 2, token: "poepoe".to_string() };
            let actual = target_repository.create(&expected_new_token);
            assert!(actual.is_ok());
            assert_eq!(actual.unwrap(), 1);
            assert_eq!(target_repository.resolve(expected_new_token.token).unwrap().user_id, expected_new_token.user_id);
        }
    }

    describe "delete" {
        it "should be remove token" {
            let expected_token = "hogefugafoobar".to_string();
            insert_into(tokens).values(NewToken { user_id: 1, token: expected_token.clone()}).execute(&*conn);
            let actual = target_repository.resolve(expected_token.clone());
            let result = target_repository.delete(actual.unwrap().id);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1);
        }
    }
}
