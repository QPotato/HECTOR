use super::super::ast::*;
use super::super::ast::position::*;
use super::super::tree::escape::*;


fn possed_exp(exp: _Exp) -> Exp {
    Exp {node: exp, pos: Pos {line: 0, column: 0}}
}

fn boxed_exp(exp: _Exp) -> Box<Exp> {
    Box::new(Exp {node: exp, pos: Pos {line: 0, column: 0}})
}


#[test]
fn escaped_arguments() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::Let {
                        decs: vec![
                            Dec::FunctionDec(vec![(
                                _FunctionDec::new(
                                    Symbol::from("fun2"),
                                    vec![Field {
                                        name: Symbol::from("arg2"),
                                        typ: Ty::Name(Symbol::from("int")),
                                        escape: false,
                                    }],
                                    Some(Symbol::from("int")),
                                    boxed_exp(_Exp::Op {
                                        left: boxed_exp(_Exp::Var(Var::Simple(Symbol::from("arg1")))),
                                        right: boxed_exp(_Exp::Var(Var::Simple(Symbol::from("arg2")))),
                                        oper: Oper::PlusOp
                                    }),
                                ),
                                Pos{line: 0, column: 0}
                            )]),
                        ],
                        body: boxed_exp(_Exp::Call {
                            func: Symbol::from("baaz"),
                            args: vec![possed_exp(_Exp::Int(2))]
                        })
                    }),
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::Call {
            func: Symbol::from("fun1"),
            args: vec![possed_exp(_Exp::Int(2))]
        })
    });
    if let Exp {node: _Exp::Let {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::FunctionDec(funs), ..)) = decs.split_first() {
            if let Some(((_FunctionDec{params, ..}, ..), ..)) = funs.split_first() {
                if let Some((Field {escape, ..}, ..)) = params.split_first() {
                    if *escape {
                        return () // PASS
                    } else {
                        panic!("wrong escape")
                    }
                }
            }
        }
    }
    panic!("wrong structure")
}

#[test]
fn not_escaped_arguments() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::Let {
                        decs: vec![
                            Dec::FunctionDec(vec![(
                                _FunctionDec::new(
                                    Symbol::from("fun2"),
                                    vec![Field {
                                        name: Symbol::from("arg2"),
                                        typ: Ty::Name(Symbol::from("int")),
                                        escape: false,
                                    }],
                                    Some(Symbol::from("int")),
                                    boxed_exp(_Exp::Op {
                                        left: boxed_exp(_Exp::Var(Var::Simple(Symbol::from("arg2")))),
                                        right: boxed_exp(_Exp::Var(Var::Simple(Symbol::from("arg2")))),
                                        oper: Oper::PlusOp
                                    }),
                                ),
                                Pos{line: 0, column: 0}
                            )]),
                        ],
                        body: boxed_exp(_Exp::Call {
                            func: Symbol::from("baaz"),
                            args: vec![possed_exp(_Exp::Int(2))]
                        })
                    }),
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::Call {
            func: Symbol::from("fun1"),
            args: vec![possed_exp(_Exp::Int(2))]
        })
    });
    if let Exp {node: _Exp::Let {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::FunctionDec(funs), ..)) = decs.split_first() {
            if let Some(((_FunctionDec{params, ..}, ..), ..)) = funs.split_first() {
                if let Some((Field {escape, ..}, ..)) = params.split_first() {
                    if !escape {
                        return () // PASS
                    } else {
                        panic!("wrong escape")
                    }
                }
            }
        }
    }
    panic!("wrong structure")
}

#[test]
fn escaped_var() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::VarDec(
                _VarDec{name: Symbol::from("var1"), escape: false, init: boxed_exp(_Exp::Int(1)), typ: None}, // var defined here
                Pos{line: 0, column: 0}
            ),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"),
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::Var(Var::Simple(Symbol::from("var1")))), // and used here
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::Call {
            func: Symbol::from("fun1"),
            args: vec![possed_exp(_Exp::Int(2))]
        })
    });
    if let Exp {node: _Exp::Let {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::VarDec(_VarDec{escape, ..}, ..), ..)) = decs.split_first() {
            if *escape {
                return () // PASS
            } else {
                panic!("wrong escape")
            }
        }
    }
    panic!("wrong structure")
}
#[test]
fn not_escaped_var() {
    let exp = possed_exp(_Exp::Let {
        decs: vec![
            Dec::VarDec(
                _VarDec{name: Symbol::from("var1"), escape: false, init: boxed_exp(_Exp::Int(1)), typ: None}, // var defined, never used
                Pos{line: 0, column: 0}
            ),
            Dec::FunctionDec(vec![(
                _FunctionDec::new(
                    Symbol::from("fun1"),
                    vec![Field {
                        name: Symbol::from("arg1"), // arg defined here
                        typ: Ty::Name(Symbol::from("int")),
                        escape: false,
                    }],
                    Some(Symbol::from("int")),
                    boxed_exp(_Exp::Var(Var::Simple(Symbol::from("arg1")))),  // and used here
                ),
                Pos{line: 0, column: 0}
            )]),
        ],
        body: boxed_exp(_Exp::Call {
            func: Symbol::from("fun1"),
            args: vec![possed_exp(_Exp::Int(2))]
        })
    });
    if let Exp {node: _Exp::Let {decs, ..}, ..} = find_escapes(exp) {
        if let Some((Dec::VarDec(_VarDec{escape, ..}, ..), ..)) = decs.split_first() {
            if !*escape {
                return () // PASS
            } else {
                panic!("wrong escape")
            }
        }
    }
    panic!("wrong structure")
}

#[test]
fn escaped_for() {
    // TODO
    let exp = possed_exp(_Exp::For {
        var: Symbol::from("i"), // iterator defined here
        lo: boxed_exp(_Exp::Int(1)),
        hi: boxed_exp(_Exp::Int(1)),
        body: boxed_exp(_Exp::Let {
            decs: vec![
                Dec::FunctionDec(vec![(
                    _FunctionDec::new(
                        Symbol::from("fun1"),
                        vec![Field {
                            name: Symbol::from("arg1"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        }],
                        Some(Symbol::from("int")),
                        boxed_exp(_Exp::Var(Var::Simple(Symbol::from("i")))), // and used here
                    ),
                    Pos{line: 0, column: 0}
                )]),
            ],
            body: boxed_exp(_Exp::Call {
                func: Symbol::from("fun1"),
                args: vec![possed_exp(_Exp::Int(2))]
            })
        }),
        escape: false
    });
    if let Exp {node: _Exp::For {escape, ..}, ..} = find_escapes(exp) {
        assert!(escape)
    }
}
#[test]
fn not_escaped_for() {
    // TODO
    let exp = possed_exp(_Exp::For {
        var: Symbol::from("i"), // iterator defined here
        lo: boxed_exp(_Exp::Int(1)),
        hi: boxed_exp(_Exp::Int(1)),
        body: boxed_exp(_Exp::Let {
            decs: vec![
                Dec::FunctionDec(vec![(
                    _FunctionDec::new(
                        Symbol::from("fun1"),
                        vec![Field {
                            name: Symbol::from("arg1"),
                            typ: Ty::Name(Symbol::from("int")),
                            escape: false,
                        }],
                        Some(Symbol::from("int")),
                        boxed_exp(_Exp::Var(Var::Simple(Symbol::from("arg1")))), // but not used
                    ),
                    Pos{line: 0, column: 0}
                )]),
            ],
            body: boxed_exp(_Exp::Call {
                func: Symbol::from("fun1"),
                args: vec![possed_exp(_Exp::Int(2))]
            })
        }),
        escape: false
    });
    if let Exp {node: _Exp::For {escape, ..}, ..} = find_escapes(exp) {
        assert!(!escape)
    }
}