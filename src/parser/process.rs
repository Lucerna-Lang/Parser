use crate::base::load as load_base;
use crate::parser::function::parse_functions;
use crate::parser::parse_with_env;
use liber::loader;
use std::env;
use structures::structs::{Env, Function, Statements};

const STDFILE: &str = "std.sic";

/// # Panics
pub fn caller(sv: Statements, e: &mut Env, is_std: bool) -> &Env {
    // Loads the environment with default libs
    load_base(e);
    loader(e);
    if !is_std {
        load_std(e);
    }
    let ss = parse_functions(sv, e);
    let mut k = Function::from_raw(ss);
    k.parse_func();
    k.call(e, vec![]); // TODO: Replace the empty vec by the command line arguments
    e
}

fn load_std(e: &mut Env) {
    let mut std = env::current_exe().unwrap();
    std.pop();
    std.push(STDFILE);
    parse_with_env(e, std, true);
}
