use super::*;

/// Inefficient lookup of field types.
///
/// fiels should actually be a HashMap
fn find_field_type(fields: &[(String, RecordFieldType, i32)], symbol: &str, type_env: &TypeEnviroment) -> Option<Arc<TigerType>> {
    for field in fields {
        if field.0 == symbol {
            match &field.1 {
                RecordFieldType::Type(t) => return Some(Arc::clone(&t)),
                RecordFieldType::Record(field_type_id) => return Some(type_env
                    .iter()
                    .find(|(_, typ)| -> bool {
                        if let TigerType::TRecord(_, table_type_id) = ***typ {
                            table_type_id == *field_type_id
                        } else { false }
                    })?
                    .1
                    .clone()
                )
            }
        }
    }
    None
}

/// Rebuild a `Var` with the correct types given the context in the enviroments or return a `TypeError`
pub fn typecheck_var(
    Var {kind, pos, ..}: Var,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment
) -> Result<Var, TypeError> {
    match kind {
        VarKind::Simple(var_symbol) => match value_env.get(&var_symbol) {
            // A simple var, as in:
            // a := foo
            Some(EnvEntry::Var { ty: var_type, .. }) => Ok(Var {
                kind: VarKind::Simple(var_symbol),
                typ: Arc::clone(&var_type),
                pos
            }),
            Some(..) => Err(TypeError::NotSimpleVar(pos)),
            None => Err(TypeError::UndeclaredSimpleVar(pos)),
        },
        VarKind::Subscript(subscript_var, index) => {
            // A subscript var, as in:
            // a := foo[3]
            let typed_subscript_var = typecheck_var(*subscript_var, type_env, value_env)?;
            let array_of = if let TigerType::TArray(array_of, ..) = &*typed_subscript_var.typ {
                Arc::clone(&array_of)
            } else {
                return Err(TypeError::NotArrayType(pos))
            };
            let typed_index = type_exp(*index, type_env, value_env)?;
            if *typed_index.typ != TigerType::TInt(R::RW) {
                return Err(TypeError::NonIntegerSubscript(pos))
            };
            Ok(Var{
                kind: VarKind::Subscript(Box::new(typed_subscript_var), Box::new(typed_index)),
                typ: array_of,
                pos
            })
        }
        VarKind::Field(field_var, field_symbol) => {
            // A field var as in:
            // a := foo.bar
            let typed_field_var = typecheck_var(*field_var, type_env, value_env)?;
            let record_fields = if let TigerType::TRecord(fields, ..) = &*typed_field_var.typ {
                fields
            } else {
                return Err(TypeError::NotRecordType(pos))
            };
            let field_type = if let Some(ty) = find_field_type(&record_fields, &field_symbol, type_env) {
                ty
            } else {
                return Err(TypeError::UndeclaredField(pos))
            };
            Ok(Var{
                kind: VarKind::Field(Box::new(typed_field_var), field_symbol),
                typ: field_type,
                pos
            })
        },
    }
}

/// Rebuild an `Exp::Var` with the correct types given the context in the enviroments or return a `TypeError`
pub fn typecheck(
    AST{node, pos, ..}: AST,
    type_env: &TypeEnviroment,
    value_env: &ValueEnviroment,
) -> Result<AST, TypeError> {
    // A Var literal
    match node {
        Exp::Var(var) => {
            let typed_var = typecheck_var(var, type_env, value_env)?;
            let typ = Arc::clone(&typed_var.typ);
            Ok(AST {
                node: Exp::Var(typed_var),
                pos,
                typ
            })
        },
        _ => panic!("Delegation error varexp::typecheck"),
    }
}

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use super::*;
    pub fn boxed_var(kind: VarKind) -> Box<Var> {
        Box::new(Var {kind, pos: Pos {line: 0, column: 0}, typ: Arc::new(TigerType::Untyped)})
    }

    #[test]
    #[wasm_bindgen_test]
    fn varexp_simplevar_ok() {
        let ast =  make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))));
        let type_env = initial_type_env();
        let mut value_env = initial_value_env();
        value_env.insert(Symbol::from("foo"), EnvEntry::Var{ty: Arc::new(TigerType::TInt(R::RW)),});
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn varexp_simplevar_not_declared() {
        let ast =  make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("foo")))));
        let type_env = initial_type_env();
        let value_env = initial_value_env();
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::UndeclaredSimpleVar(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(ast) => panic!("Should error, returns: {:?}", ast)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn varexp_simplevar_not_simple() {
        let ast =  make_ast(Exp::Var(make_var(VarKind::Simple(Symbol::from("f")))));
        let type_env = initial_type_env();
        let mut value_env = initial_value_env();
        value_env.insert(Symbol::from("f"), EnvEntry::Func {
            formals: vec![],
            result: Arc::new(TigerType::TUnit),
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NotSimpleVar(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn varexp_fieldvar_ok() {
        let ast = make_ast(Exp::Var(
            make_var(VarKind::Field(
                boxed_var(VarKind::Simple(Symbol::from("foo"))),
                Symbol::from("bar")))
        ));
        let mut type_env = initial_type_env();
        let mut value_env = initial_value_env();
        let field_type = Arc::new(TigerType::TInt(R::RW));
        let foo_type = Arc::new(TigerType::TRecord(
                vec![(String::from("bar"),
                    RecordFieldType::Type(field_type),
                    0)], TypeId::new()));
        type_env.insert(Symbol::from("FooType"), Arc::clone(&foo_type));
        value_env.insert(Symbol::from("foo"), EnvEntry::Var{
            ty: foo_type,
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn varexp_fieldvar_field_does_not_exist() {
        let ast = make_ast(Exp::Var(
            make_var(VarKind::Field(
                boxed_var(VarKind::Simple(Symbol::from("foo"))),
                Symbol::from("perro")))
        ));
        let mut type_env = initial_type_env();
        let mut value_env = initial_value_env();
        let foo_type = Arc::new(TigerType::TRecord(
                vec![(String::from("bar"),
                    RecordFieldType::Type(Arc::new(TigerType::TInt(R::RW))),
                    0)], TypeId::new()));
        type_env.insert(Symbol::from("FooType"), Arc::clone(&foo_type));
        value_env.insert(Symbol::from("foo"), EnvEntry::Var{
            ty: foo_type,
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::UndeclaredField(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn varexp_fieldvar_not_record() {
        let ast = make_ast(Exp::Var(
            make_var(VarKind::Field(
                boxed_var(VarKind::Simple(Symbol::from("foo"))),
                Symbol::from("bar")))
        ));
        let mut type_env = initial_type_env();
        let mut value_env = initial_value_env();
        let foo_type = Arc::new(TigerType::TInt(R::RW));
        type_env.insert(Symbol::from("FooType"), Arc::clone(&foo_type));
        value_env.insert(Symbol::from("foo"), EnvEntry::Var{
            ty: foo_type,
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NotRecordType(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn varexp_subscriptvar_ok() {
        let ast = make_ast(Exp::Var(
            make_var(VarKind::Subscript(boxed_var(VarKind::Simple(Symbol::from("foo"))),
            boxed_ast(Exp::Int(0))),
        )));
        let mut type_env = initial_type_env();
        let mut value_env = initial_value_env();
        let foo_type = Arc::new(TigerType::TArray(
            Arc::new(TigerType::TInt(R::RW)),
            TypeId::new(),
        ));
        type_env.insert(Symbol::from("FooType"), Arc::clone(&foo_type));
        value_env.insert(Symbol::from("foo"), EnvEntry::Var{
            ty: foo_type,
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Ok(AST{typ, ..}) if *typ == TigerType::TInt(R::RW) => (),
            Ok(AST{typ, ..}) => panic!("wrong type: {:?}", typ),
            Err(type_error) => panic!("type error: {:?}", type_error)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn varexp_subscriptvar_non_integer_index() {
        let ast = make_ast(Exp::Var(
            make_var(VarKind::Subscript(
                boxed_var(VarKind::Simple(Symbol::from("foo"))),
                boxed_ast(Exp::String(String::from("una string de indice :o"))),
            ))
        ));
        let mut type_env = initial_type_env();
        let mut value_env = initial_value_env();
        let foo_type = Arc::new(TigerType::TArray(
            Arc::new(TigerType::TInt(R::RW)),
            TypeId::new(),
        ));
        type_env.insert(Symbol::from("FooType"), Arc::clone(&foo_type));
        value_env.insert(Symbol::from("foo"), EnvEntry::Var{
            ty: foo_type,
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NonIntegerSubscript(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn varexp_subscriptvar_not_array() {
        let ast = make_ast(Exp::Var(
            make_var(VarKind::Subscript(boxed_var(VarKind::Simple(Symbol::from("foo"))),
            boxed_ast(Exp::Int(0))),
        )));
        let mut type_env = initial_type_env();
        let mut value_env = initial_value_env();
        let foo_type = Arc::new(TigerType::TInt(R::RW));
        type_env.insert(Symbol::from("FooType"), Arc::clone(&foo_type));
        value_env.insert(Symbol::from("foo"), EnvEntry::Var{
            ty: foo_type,
        });
        let res = type_exp(ast, &type_env, &value_env);
        match res {
            Err(TypeError::NotArrayType(_)) => (),
            Err(type_error) => panic!("Wrong type error: {:?}", type_error),
            Ok(tiger_type) => panic!("Should error, returns: {:?}", tiger_type)
        }
    }
}