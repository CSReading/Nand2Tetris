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
use JackTokenizer::JackTokenizer as Tokenizer;
use CompilationEngine::CompilationEngine as Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = Path::new(&args[1]);
    let outfilename;
    if filename.extension() == Some(OsStr::new("jack")) {
        let name = filename.file_name().unwrap().to_str().unwrap();
        let folder = filename.parent().unwrap().join("out");
        fs::create_dir_all(folder.to_str().unwrap());
        outfilename = folder.join(name.trim_end_matches(".jack").to_owned() + ".xml").to_str().unwrap().to_owned();
    } else {
        let name = filename.join(filename.file_stem().unwrap().to_str().unwrap());
        outfilename = name.to_str().unwrap().to_owned() + ".xml";
    }

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let tokenizer = Tokenizer::new(contents);
    let mut compilation_engine = Compiler::new(tokenizer);
    compilation_engine.compile_class();
    // compilation_engine.token_test();
    
    let mut outfile = match File::create(outfilename) {
        Err(why) => panic!("couldn't create new file: {}", why),
        Ok(outfile) => outfile,
    }; 

    match outfile.write_all(compilation_engine.xml.as_bytes()) {
        Err(why) => panic!("couldn't write to new file: {}", why),
        Ok(_) =>println!("successfully write new file"),
    };
}
