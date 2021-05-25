mod raw;
pub mod process;
mod function;

// STD
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// Crate
use structures::structs::{Env, Statement, Statements};
// Super
use raw::{organize_s, fuse_statements};

pub fn parse<T>(filename: T)
where T: AsRef<Path>, {
    let file = File::open(filename).expect("Error opening file");
    let stats = pre_parse(file);
    let mut env = Env::new();
    process::caller((&stats).clone(), &mut env, false);
}

pub fn parse_with_env<T>(e: &mut Env, filename: T, is_std: bool) -> &Env
where T: AsRef<Path>, {
    let file = File::open(filename).expect("Could not find required files");
    let stats = pre_parse(file);
    process::caller((&stats).clone(), e, is_std)
}

fn pre_parse(f: File) -> Statements {
    fuse_statements(organize_s(io::BufReader::new(f).lines()))
}

