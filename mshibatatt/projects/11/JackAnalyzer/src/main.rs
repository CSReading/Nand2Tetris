#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;
mod JackTokenizer;
mod CompilationEngine;
mod SymbolTable;
mod VMWriter;
use JackTokenizer::JackTokenizer as Tokenizer;
use CompilationEngine::CompilationEngine as Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = Path::new(&args[1]);
    let mut outfilename = String::from("../test/");

    assert!(filename.extension() == Some(OsStr::new("jack")));
    let name = filename.file_name().unwrap().to_str().unwrap();
    outfilename += name.trim_end_matches(".jack");
    outfilename += ".vm";

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let tokenizer = Tokenizer::new(contents);
    let mut compilation_engine = Compiler::new(tokenizer, &outfilename);
    compilation_engine.compile_class();
    // compilation_engine.token_test();
    
    compilation_engine.writer.close();
}
