extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;
use std::io::{stdin, Read};

use std::env;
use std::fs;

fn main() {
    println!("What is the filename?");
    let mut filename = String::new();

    stdin().read_line(&mut filename).unwrap();
    let filename = &filename[..(filename.len() - 1)];

    println!("What is the gender?");
    let mut gender = String::new();
    stdin().read_line(&mut gender).unwrap();
    let gender = &gender[..(gender.len() - 1)];

    load_from_file(&filename.to_string(), &gender.to_string());
}

fn load_from_file(filename: &String, gender: &String) {
    let connection = establish_connection();

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents.split("\n");
    for line in lines {
        let word: Vec<&str> = line.split('\t').collect();
        let word_name = word[1];
        let freq:f32 = word[17].parse().unwrap();
//        println!("{} {}", word_name,freq);
        let word = create_word(&connection, word_name, &gender, &freq);
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
