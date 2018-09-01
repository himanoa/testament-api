extern crate dotenv;
extern crate testament;

use std::env::var;
use std::process::exit;

use dotenv::dotenv;

use testament::create_rocket;

fn main() {
    dotenv().ok();
    let database_url = match var("DATABASE_URL") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    create_rocket(&database_url).launch();
}
