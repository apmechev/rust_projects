extern crate diesel_demo;
extern crate diesel;
extern crate rand;

use self::diesel_demo::*;
use self::diesel::prelude::*;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};

use std::io;

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
    println!("How many words do you want to test?");
    let mut guess = String::new();
    io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    let guess: i64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!"); 
                0
            }
        };

    let numwords  = get_number_words();
    let mut counter = 0;
    while counter < guess {
        counter += 1;
        get_random_n_word(10000)
    
    };

}

fn get_random_n_word(word_limit:i64) {

    use diesel_demo::schema::words::dsl::*;

    let connection = establish_connection();
    let mut rng = rand::thread_rng();
    let word_id: i64 = rng.gen_range(1, word_limit);
//    let word_id =  word_id as i32; // Need to cast it to i32 for it to work with diesel::Expressin id.eq()
    println!("{}", word_id);
    let word1: (String, String) =  words
        .order(frequency.desc())
        .limit(word_limit)
        .offset(word_id)
        .select((word, gender))
//        .get_result(&connection)
        .first(&connection)
        .expect("Error getting word");
    println!("word {} is {}",word1.0, word1.1);
}

