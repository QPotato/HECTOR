extern crate hector;
use std::fs::read_to_string;


fn main() {
    let contents = read_to_string("./tiger_sources/good/test01.tig").expect("read_to_string");
    println!("{:#?}", hector::run_compile(&contents).wasm)
}