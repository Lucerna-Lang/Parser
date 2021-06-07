use crate::parser::parse_with_env;
use std::rc::Rc;
use structures::structs::{DefaultTypes, DynFunc, Env, Function, Table};

// We store this function as a function pointer in a Vec, so the type signatures must line up
#[allow(clippy::needless_pass_by_value)]
pub fn print(e: &mut Env, v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let x = v.get(0).expect("Bad arguments");
    if let DefaultTypes::Str(s) = x {
        print!("{}", s);
    } else {
        dbg!(&x);
        println!("Bad arguments");
        e.exit();
    }
    v
}

// We store this function as a function pointer in a Vec, so the type signatures must line up
#[allow(clippy::needless_pass_by_value)]
pub fn with(e: &mut Env, v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    match &v[0] {
        DefaultTypes::Str(s) => {
            parse_with_env(e, s, false);
        }
        _ => {
            println!("With argument should be a string");
        }
    }
    v
}

// We store this function as a function pointer in a Vec, so the type signatures must line up
#[allow(clippy::needless_pass_by_value)]
pub fn table(e: &mut Env, v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let x = vec![DefaultTypes::Table(Table::new())];
    x
}

// We store this function as a function pointer in a Vec, so the type signatures must line up
#[allow(clippy::needless_pass_by_value)]
pub fn get(e: &mut Env, v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let mut x = vec![];
    let tab = v.get(0).expect("Bad Arguments");
    let key = v.get(1).expect("Bad Arguments");
    if let DefaultTypes::Table(t) = tab {
        if let DefaultTypes::Str(s) = key {
            x.push(t.raw_get(s).expect("Value didn't exist"));
        }
    } else {
        println!("Invalid type");
    }
    x
}

// We store this function as a function pointer in a Vec, so the type signatures must line up
#[allow(clippy::needless_pass_by_value)]
pub fn return_def(e: &mut Env, v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    e.return_f(v.clone());
    v
}

pub fn loop_def(e: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let func = v.remove(0);
    if let DefaultTypes::Function(f) = func {
        loop {
            let res = f.call(e, v.clone());
            if !res.is_empty() {
                if let DefaultTypes::Str(s) = &res[0] {
                    if s == "STOPLOOP" {
                        break;
                    }
                }
            }
        }
    }
    v
}

pub fn add(e: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let int1 = v.remove(0);
    let int2 = v.remove(0);
    match (int1, int2) {
        (DefaultTypes::Int(i1), DefaultTypes::Int(i2)) => {
            vec![DefaultTypes::Int(i1 + i2)]
        },
        (DefaultTypes::Str(string), DefaultTypes::Str(other_string)) => {
           vec![DefaultTypes::Str(format!("{}{}", &string, &other_string))] 
        }
        (_, _) => v,
    }
}

pub fn eq(e: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let int1 = v.remove(0);
    let int2 = v.remove(0);
    match (int1, int2) {
        (DefaultTypes::Int(i1), DefaultTypes::Int(i2)) => {
            vec![DefaultTypes::Bool(i1 == i2)]
        },
        (_, _) => {
            println!("Attempting to call eq on different types");
            v
        }
    }
}

pub fn noteq(e: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let int1 = v.remove(0);
    let int2 = v.remove(0);
    match (int1, int2) {
        (DefaultTypes::Int(i1), DefaultTypes::Int(i2)) => {
            vec![DefaultTypes::Bool(i1 != i2)]
        }
        (_, _) => {
            println!("Attempting to call eq on different types");
            v
        }
    }
}

pub fn break_def(e: &mut Env, v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    e.return_f(vec![DefaultTypes::Str("STOPLOOP".to_string())]);
    v
}

pub fn set(e: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let mut x = vec![];
    let tab = v.remove(0);
    let key = v.remove(0);
    let value = v.remove(0);
    if let DefaultTypes::Table(mut t) = tab {
        if let DefaultTypes::Str(s) = key {
            t.set(s, value);
            x.push(DefaultTypes::Table(t));
        }
    } else {
        println!("Invalid type");
        e.exit();
    }
    x
}

pub fn rm(e: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let x = vec![];
    let key = v.remove(0);
    if let DefaultTypes::Str(s) = key {
        e.remove(&s);
    }
    x
}

pub fn load_module(e: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let x = vec![];
    let tab = v.remove(0);
    if let DefaultTypes::Table(t) = tab {
        for (k, v) in t.iter_data() {
            e.add_variable(&k, v.clone());
        }
    } else {
        println!("Not a module");
        e.exit();
    }
    x
}

pub fn if_def(e: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let condition = v.remove(0);
    let func = v.remove(0);
    if let DefaultTypes::Bool(b) = condition {
        if b {
            if let DefaultTypes::Function(f) = func {
                f.call(e, v.clone());
            } else {
                println!("If statement did not provide a function");
            }
        }
    } else {
        println!("If statement did not provide a boolean");
    }
    v
}

fn create_func(name: &'static str, f: &'static DynFunc, e: &mut Env) {
    e.add_variable(name, DefaultTypes::Function(Function::new(Rc::new(f))));
}

pub fn load(e: &mut Env) {
    // Most important functions
    create_func("if", &if_def, e);
    create_func("return", &return_def, e);
    create_func("with", &with, e);
    create_func("print", &print, e);
    create_func("table", &table, e);
    create_func("get", &get, e);
    create_func("add", &add, e);
    create_func("loop", &loop_def, e);
    create_func("set", &set, e);
    create_func("load", &load_module, e);
    create_func("eq", &eq, e);
    create_func("noteq", &noteq, e);
    create_func("remove", &rm, e);
    create_func("break", &break_def, e);
}
