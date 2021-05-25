#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(dead_code, unused_assignments, unused_variables)]

use std::env;


pub mod parser;
pub mod base;

fn main() {
    parser::parse(env::args().collect::<Vec<String>>().get(1).expect("No file provided"));
}
