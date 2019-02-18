extern crate diesel_demo;
extern crate diesel;
extern crate rand;

use self::diesel_demo::*;
use self::diesel::prelude::*;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};



fn get_number_words() -> i64 {
    use diesel_demo::schema::words::dsl::*;

    let connection = establish_connection();
    let results: i64 = words
        .count()
        .get_result(&connection)
        .expect("Error conting words");
    println!("{} words found", results);
    results
    
}


fn main() {
    use diesel_demo::schema::words::dsl::*;

    println!("Counting all words:");
    let numwords = get_number_words();
    let connection = establish_connection();
    
    let mut rng = rand::thread_rng();
    let word_id: i64 = rng.gen_range(1, numwords);

    let word_id =  word_id as i32; // Need to cast it to i32 for it to work with diesel::Expressin id.eq()

    let word1: (String, String) =  words
        .filter(id.eq(word_id))
        .select((word,gender))
        .first(&connection)
        .expect("Error conting words");
    println!("word {} is {}",word1.0, word1.1)
}
