// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

pub mod garden;
mod chapter2_guessing_game;
mod chapter3_concepts;
mod chapter4_ownership;
mod chapter5_structs;
mod chapter6_enums_pattern_match;
mod chapter7_packaging;
mod chapter8_collections;
mod chapter9_errors;
mod chapter10_generics_traits_lifetimes;
mod chapter12_io_project;

fn main() {
    //chapter2::guessgame();
    // chapter8::convert_to_pig_latin("hello world wonderful amazing world");

    // won't compile because it doesn't return Result<T, E>
    //let greeting_file = File::open("hello.txt")?;


    chapter12_io_project::io_project();
}