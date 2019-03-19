#[macro_use]
extern crate diesel;
extern crate dotenv;


pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Word, NewWord};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_word<'a>(conn: &PgConnection, word: &'a str, gender:  &'a str, frequency: &'a f32) -> Word {
    use schema::words;
    println!("{}", frequency);

    let new_word = NewWord {
        word: word,
        gender: gender,
        frequency: frequency, 
    };

    diesel::insert_into(words::table)
        .values(&new_word)
        .get_result(conn)
        .expect("Error saving word")
}

/*

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/
