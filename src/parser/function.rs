use structures::structs::{DefaultTypes, Env, Function, Statements, Statement};

pub fn parse_functions(ss: Statements, e: &mut Env) -> Statements {
    let mut c_func = Function::from_raw(vec![]);
    let mut is_function_scope = false;
    let mut tr: Statements = Vec::new();
    for s in ss {
        if s.is_function_decl() {
            is_function_scope = true;
            c_func = Function::from_raw(vec![]);
            c_func.set_name((&s.first()).clone());
        }
        if s.is_function_end() {
            is_function_scope = false;
            let mut f = c_func.clone();
            f.parse_func();
            tr.push(
                Statement::new(
                    vec![c_func.name().to_string(), "->".to_string(), "SEGMENT".to_string(), ";".to_string()],
                    s.line(),
                ).with_setter(DefaultTypes::Function(f.clone())),
            );
        } else if is_function_scope && !s.is_function_decl() {
            c_func.push_raw((&s).clone());
        }
        if !s.is_function_end() && !s.is_function_decl() && !is_function_scope {
            tr.push((&s).clone());
        }
    }
    tr
}
