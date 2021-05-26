#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(dead_code, unused_assignments, unused_variables)]

use std::env;

pub mod base;
pub mod parser;

fn main() {
    parser::parse(
        env::args()
            .collect::<Vec<String>>()
            .get(1)
            .expect("No file provided"),
    );
}
