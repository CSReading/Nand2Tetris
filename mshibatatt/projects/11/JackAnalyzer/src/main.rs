#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs;
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


fn compile(infilename: &Path, outfilename: String) {
    let mut f = File::open(infilename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let tokenizer = Tokenizer::new(contents);
    let mut compilation_engine = Compiler::new(tokenizer, &outfilename);
    compilation_engine.compile_class();
    
    compilation_engine.writer.close();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = Path::new(&args[1]);
    let mut outfilename = String::from("../test/");

    if filename.extension() == Some(OsStr::new("jack")) {
        let name = filename.file_name().unwrap().to_str().unwrap();
        outfilename += name.trim_end_matches(".jack");
        outfilename += ".vm";
        compile(&filename, outfilename);
    } else {
        let paths = fs::read_dir(filename).unwrap();
        for path in paths {
            if !path.as_ref().unwrap().path().to_str().unwrap().ends_with(".jack") {
                continue;
            }
            outfilename = String::from("../test/");
            let name = path.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap().to_owned();
            outfilename += name.trim_end_matches(".jack");
            outfilename += ".vm";
            compile(&path.as_ref().unwrap().path(), outfilename);
        }
    };
}
