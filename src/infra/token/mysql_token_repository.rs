use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{delete, insert_into};
use models::token::{NewToken, Token, TokenRepository};
use models::user::User;
use schema::tokens;
use schema::tokens::dsl::*;

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
        tokens
            .select((
                tokens::id,
                tokens::user_id,
                tokens::token,
                tokens::updated_at,
                tokens::created_at,
            )).filter(tokens::token.eq(input_token))
            .first::<Token>(self.conn)
    }
    fn create(&self, new_token: &NewToken) -> Result<usize, DieselError> {
        insert_into(tokens).values(new_token).execute(self.conn)
    }
    fn delete(&self, target_id: i32) -> Result<usize, DieselError> {
        delete(tokens)
            .filter(tokens::id.eq(target_id))
            .execute(self.conn)
    }
    // TODO: テストを書く
    fn delete_all_by_user(&self, user: &User) -> Result<usize, DieselError> {
        delete(tokens)
            .filter(tokens::user_id.eq(user.id))
            .execute(self.conn)
    }
    // TODO: テストを書く
    fn list_by_user(&self, user: &User) -> Result<Vec<Token>, DieselError> {
        tokens
            .select((
                tokens::id,
                tokens::user_id,
                tokens::token,
                tokens::updated_at,
                tokens::created_at,
            )).filter(tokens::user_id.eq(user.id))
            .get_results::<Token>(self.conn)
    }
}
