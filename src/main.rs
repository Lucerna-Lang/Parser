#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(dead_code, unused_assignments, unused_variables)]

use std::{env, panic};

pub mod base;
pub mod parser;

fn main() {
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
    parser::parse(
        env::args()
            .collect::<Vec<String>>()
            .get(1)
            .expect("No file provided"),
    );
}
