extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What is the word");
    let mut word = String::new();
    stdin().read_line(&mut word).unwrap();
    let word = &word[..(word.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", word, EOF);
    let mut gender = String::new();
    stdin().read_to_string(&mut gender).unwrap();
    let gender =  &gender[..(gender.len() - 1)];
    let word = create_word(&connection, word, &gender);
    println!("\nSaved draft {} with id {}", word.word,word.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
