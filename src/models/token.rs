use chrono::NaiveDateTime;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use infra::mysql::DbConn;
use schema::users;
use schema::tokens;
use schema::tokens::dsl::*;

#[derive(Debgu, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "tokens"]
#[belongs_to(User)]
pub struct Tokens {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debgu, Serialize, Deserialize, Insertable)]
#[table_name = "tokens"]
#[belongs_to(User)]
pub struct NewTokens {
    pub user_id: i32,
    pub token: String,
}

