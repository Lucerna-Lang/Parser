use super::{Statement, Statements, io};

pub(super) fn split_w(s: &str) -> Option<Vec<String>> {
    let mut s1 = String::new();
    let mut is_in_literal = false;
    for x in s.chars() {
        if !is_in_literal || x != ' ' {
            s1.push(x);
        } else {
            s1.push_str("|_");
        }
        if x == '"' && is_in_literal {
            is_in_literal = false;
        } else if x == '"' && !is_in_literal {
            is_in_literal = true;
        }
    }

    let new_s = String::from(s1
        .replace(";", " ;")
        .replace("{", " { ;")
        .replace("}", " } ;")
        .replace(",", " ")
        .replace("(", "( ")
        .replace(")", " )")
        .split("//")
        .next()
        .unwrap());
    if new_s == *"" {
        None
    } else {
        let v = new_s.split_whitespace().collect::<Vec<&str>>();
        let mut r = Vec::new();
        for t in v {
            r.push(String::from(t));
        }
        Some(r)
    }
}

pub(super) fn organize_s<T>(lines: T) -> Statements
    where T: Iterator<Item = Result<String, io::Error>>, {
    let mut statements = Vec::new();
    let mut i: u32 = 1;
    for line in lines {
        if let Some(dat) = split_w(&line.unwrap()) {
            statements.push(Statement::new(dat, i));
        }
        i += 1;
    }
    statements
}


pub(super) fn fuse_statements(s: Statements) -> Statements {
    let mut new: Statements = Vec::new();
    let raw: Vec<String> = vec!();
    let mut unfinished_statement = Statement::new(raw, 1);
    for mut statement in s {
        let mut slice: Vec<String> = statement.mut_raw()[0..].to_vec();
        let l = statement.line();
        if !unfinished_statement.is_finished() {
            let x = statement.mut_raw();
            let unf = unfinished_statement.mut_raw();
            let len = unf.len();
            if len > 0 {
                unf[len-1].push_str(&x[0]);
                slice = statement.mut_raw()[1..].to_vec();
            }
        }
        unfinished_statement.mut_raw().append(&mut slice);
        if unfinished_statement.is_finished() {
            new.push(unfinished_statement);
            unfinished_statement = Statement::new(vec!(), l+1);
        }
    }
    new
}