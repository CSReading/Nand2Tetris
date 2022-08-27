use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;
mod parser; 
use parser::Parser;
use parser::CommandType;
mod codewriter; 

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = Path::new(&args[1]);
    let outfilename;
    if filename.extension() == Some(OsStr::new("vm")) {
        let name = filename.to_str().unwrap();
        outfilename = name.trim_end_matches(".vm").to_owned() + ".asm";
    } else {
        let name = filename.join(filename.file_stem().unwrap().to_str().unwrap());
        outfilename = name.to_str().unwrap().to_owned() + ".asm";
    }

    let mut writer = codewriter::init(&outfilename);
    let mut vmfiles: Vec<Parser> = Vec::new();
    let mut basefilenames: Vec<String> = Vec::new();
    let mut contents: String;
    if filename.extension() == Some(OsStr::new("vm")) {
        let mut f = File::open(filename).expect("file not found");
        contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        vmfiles.push(parser::init(contents));
        basefilenames.push(filename.file_stem().unwrap().to_str().unwrap().to_owned());
    } else {
        let paths = fs::read_dir(filename).unwrap();
        for path in paths {
            if !path.as_ref().unwrap().path().to_str().unwrap().ends_with(".vm") {
                    continue;
                }
            let mut f = File::open(path.as_ref().unwrap().path()).expect("file not found");
            contents = String::new();
            f.read_to_string(&mut contents).expect("something went wrong reading the file");
            vmfiles.push(parser::init(contents));
            basefilenames.push(path.as_ref().unwrap().path().file_stem().unwrap().to_str().unwrap().to_owned());
        }
        writer.write_init();
    };
       
    for (vm, f) in vmfiles.iter_mut().zip(basefilenames.into_iter()) {
        writer.set_file_name(f);
        vm.advance();
        // update for vmtranslator
        while vm.has_more_commands() {
            match vm.command_type() {
                CommandType::CPUSH | CommandType::CPOP => 
                    writer.write_push_pop(
                        vm.command_type(),
                        &vm.arg1(), // &str
                        vm.arg2() // usize
                    ),
                CommandType::CARITHMETIC => writer.write_arithmetic(&vm.arg1()),
                CommandType::CLABEL => writer.write_label(&vm.arg1()),
                CommandType::CGOTO => writer.write_goto(&vm.arg1()),
                CommandType::CIF => writer.write_if(&vm.arg1()),
                CommandType::CFUNCTION => writer.write_function(&vm.arg1(), vm.arg2()),
                CommandType::CRETURN => writer.write_return(),
                CommandType::CCALL => writer.write_call(&vm.arg1(), vm.arg2()),
            }
            vm.advance();
        }
    }
  

    writer.close();
}