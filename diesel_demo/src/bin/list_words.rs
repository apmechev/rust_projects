extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_demo::schema::words::dsl::*;

    let connection = establish_connection();
    let results = words.limit(5)
        .load::<Word>(&connection)
        .expect("Error loading words");

    println!("Displaying {} words", results.len());
    for wordd in results {
        println!("{} is {}", wordd.word, wordd.Gender);
        println!("----------\n");
    
    }
}
