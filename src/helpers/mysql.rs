use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

pub struct DbConn(pub PooledConnection<ConnectionManager<MysqlConnection>>);
type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn init_mysql_pool(database_url: &str) -> MysqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::new(manager).expect("プールの取得に失敗しました")
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
