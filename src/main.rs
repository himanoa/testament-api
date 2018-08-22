extern crate testament;

use testament::create_rocket;
use std::env::var;
use std::process::exit;

fn main() {
    let database_url = match var("DATABASE_URL") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    create_rocket(&database_url).launch();
}
