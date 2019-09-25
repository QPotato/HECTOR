use super::super::ast::tigerabs::*;
use super::tigerseman::*;

pub fn typecheck<'a>(exp: &Exp<'a>, type_env: &'a TypeEnviroment<'a>, value_env: &ValueEnviroment<'a>) ->  Result<Tipo<'a>, TypeError> {
    use std::collections::HashMap;
    let tipar_fields = |args: &Vec<(Symbol, Box<Exp<'a>>)>| -> HashMap<Symbol, Result<Tipo<'a>, TypeError>> {
        args.iter().map(|arg| (arg.0.clone(), type_exp(&*arg.1, type_env, value_env))).rev().collect()
    };
    match exp { Exp {node: _Exp::RecordExp{fields, typ: record_type_string, ..}, pos} => {
        let mut field_types = tipar_fields(fields);

        let record_type = match type_env.get(record_type_string) {
            Some(tipo) => tipo_real(tipo.clone(), type_env),
            None => return Err(TypeError::UndeclaredType(*pos))
        };
        match record_type {
            Tipo::TRecord(formals, type_id) => {
                for formal in formals.clone() {
                    match field_types.get(&*formal.0) {
                        Some(Ok(field_type)) => if *field_type == *formal.1 {
                            field_types.remove(&*formal.0);
                        }
                        else {
                            return Err(TypeError::TypeMismatch(*pos));
                        },
                        Some(Err(type_error)) => return Err((*type_error).clone()),
                        None =>  return Err(TypeError::MissingRecordField(*pos)),
                    }
                }
                if field_types.is_empty() {
                    Ok(Tipo::TRecord(formals, type_id))
                } else {
                    Err(TypeError::TooManyArguments(*pos))
                }
            },
            Tipo::TUnit | Tipo::TNil | Tipo::TInt(..) | Tipo::TString | Tipo::TArray(..) | Tipo::TipoInterno(..) => Err(TypeError::NotRecordType(*pos)),
        }
    }
    _ => panic!("delegation panic on recordexp::tipar")
    }
}

pub fn translate(_exp: Exp) -> ExpInterm {
    ExpInterm::CONST(0)
}