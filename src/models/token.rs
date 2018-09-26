use chrono::NaiveDateTime;
use diesel::{insert_into, delete};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use infra::mysql::DbConn;
use schema::users;
use schema::tokens;
use schema::tokens::dsl::*;
use models::user::User;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[table_name = "tokens"]
#[belongs_to(User)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Insertable, Associations)]
#[table_name = "tokens"]
#[belongs_to(User)]
pub struct NewToken {
    pub user_id: i32,
    pub token: String,
}

pub trait TokenRepository {
    fn resolve(&self, token: String) -> Result<Token, DieselError>;
    fn create(&self, newToken: NewToken) -> Result<usize, DieselError>;
    fn delete(&self, targetId: i32) -> Result<usize, DieselError>;
    fn delete_all_by_user(&self, user: User) -> Result<usize, DieselError>;
    fn list_by_user(&self, user: User) -> Result<Vec<Token>, DieselError>;
}

pub struct MySqlTokenRepository<'a> {
    conn: &'a MysqlConnection,
}

impl<'a> MySqlTokenRepository<'a> {
    pub fn new(conn: &'a MysqlConnection) -> MySqlTokenRepository {
        MySqlTokenRepository { conn: conn }
    }
}

impl<'a> TokenRepository for MySqlTokenRepository<'a> {
    fn resolve(&self, input_token: String) -> Result<Token, DieselError> {
        tokens.select((
            tokens::id,
            tokens::user_id,
            tokens::token,
            tokens::updated_at,
            tokens::created_at,
        )).filter(tokens::token.eq(input_token)).first::<Token>(self.conn)
    }
    fn create(&self, new_token: NewToken) -> Result<usize, DieselError> {
        insert_into(tokens).values(new_token).execute(self.conn)
    }
    fn delete(&self, target_id: i32) -> Result<usize, DieselError> {
        delete(tokens).filter(tokens::id.eq(target_id)).execute(self.conn)
    }
    fn delete_all_by_user(&self, user: User) -> Result<usize, DieselError> {
        delete(tokens).filter(tokens::user_id.eq(user.id)).execute(self.conn)
    }
    fn list_by_user(&self, user: User) -> Result<Vec<Token>, DieselError> {
        tokens
            .select((
                tokens::id,
                tokens::user_id,
                tokens::token,
                tokens::updated_at,
                tokens::created_at
            ))
            .filter(tokens::user_id.eq(user.id)).get_results::<Token>(self.conn)
    }
}
