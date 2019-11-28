use super::tigerabs::{Exp, _Exp};
use super::position::{Pos, WithPos};
use super::lexer::Lexer;
use std::fs::{read_dir, read_to_string};
use lalrpop_util::lalrpop_mod;

#[macro_use]
lalrpop_mod!(pub parser);

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Pos),
}


pub fn parse(source : String) -> Result<Exp, ParseError> {
    let str_src: &str = &*source;
    let lexed = Lexer::new(str_src.lines());
    //let box_exp = parser::ExprParser::new().parse(lexed).unwrap();
    let box_exp = parser::ExprParser::new().parse(lexed).unwrap();

    println!("{:?}", box_exp);

    return Ok(*box_exp);
}

#[test]
fn basic_full_tokens() {
    let good_path = "./tiger_sources/lexer/simplest.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    parse(contents);
    return;
}

#[test]
fn basic_compound_tokens() {
    let good_path = "./tiger_sources/lexer/compound_tokens.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    println!("Parser output:\n\n");
    parse(contents);
    return;
}

#[test]
fn complex_compound_tokens() {
    let good_path = "./tiger_sources/lexer/complex_compound.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    println!("Parser output:\n\n");
    parse(contents);
    return;
}

#[test]
fn simple_line_comments() {
    let good_path = "./tiger_sources/lexer/simple_line_comments.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    println!("Parser output:\n\n");
    parse(contents);
    return;
}

#[test]
fn simple_block_comments() {
    let good_path = "./tiger_sources/lexer/simple_block_comments.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    println!("Parser output:\n\n");
    parse(contents);
    return;
}

#[test]
fn simple_strings() {
    let good_path = "./tiger_sources/lexer/simple_strings.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    println!("Parser output:\n\n");
    parse(contents);
    return;
}

#[test]
fn spaced_strings() {
    let good_path = "./tiger_sources/lexer/spaced_strings.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    println!("Parser output:\n\n");
    parse(contents);
    return;
}

#[test]
fn escaped_strings() {
    let good_path = "./tiger_sources/lexer/escaped_strings.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    println!("Parser output:\n\n");
    parse(contents);
    return;
}

#[test]
fn multiline_strings() {
    let good_path = "./tiger_sources/lexer/multiline_strings.tig";
    let contents: String = read_to_string(&good_path).unwrap();
    let lexed = Lexer::new(contents.lines());

    for tok in lexed {
        if let Ok(token) = tok {
            println!("{:?}", token);
        } else {
            panic!("Lex error!");
        }
    }

    println!("Parser output:\n\n");
    parse(contents);
    return;
}
