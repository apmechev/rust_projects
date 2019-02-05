extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use std::io::{stdin, Read};

use std::env;
use std::fs;


fn main() {
    let connection = establish_connection();

    println!("What is the filename?");
    let mut filename = String::new();

    stdin().read_line(&mut filename).unwrap();
    let filename = &filename[..(filename.len() -1)];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    
    let mut lines = contents.split("\n");

    println!("What is the gender?");
    let mut gender = String::new();
    stdin().read_line(&mut gender).unwrap();
    let gender =  &gender[..(gender.len() - 1)];


    for line in lines {
        let word :  Vec<&str> = line.split('\t').collect();
        let word = word[1];
        let word = create_word(&connection, word, &gender);
        println!("\nSaved word {} with id {}", word.word,word.id);
}
 
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
