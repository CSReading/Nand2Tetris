use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
mod parser; 
use parser::Parser;
use parser::CommandType;
mod codewriter; 

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let outfilename;
    if filename.ends_with(".vm") {
        outfilename = filename.trim_end_matches(".vm").to_owned() + ".asm";
    } else {
        outfilename = filename.to_owned() + ".asm";
    }

    
    let mut vmfiles: Vec<Parser> = Vec::new();
    let mut contents: String;
    if filename.ends_with(".vm") {
        let mut f = File::open(filename).expect("file not found");
        contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        vmfiles.push(parser::init(&mut contents));
    } else {
        unimplemented!();
        // match fs::read_dir(filename) {
        //     Err(why) => println!("! {:?}", why.kind()),
        //     Ok(paths) => for path in paths {
        //         let mut f = File::open(path.unwrap().path()).expect("file not found");
        //         contents = String::new();
        //         f.read_to_string(&mut contents).expect("something went wrong reading the file");
        //         vmfiles.push(parser::init(&mut contents));
        //     }
        // };
    }
    
    let mut writer = codewriter::init(&outfilename);
    
    vmfiles[0].advance();
    // update for vmtranslator
    while vmfiles[0].has_more_commands() {
        match vmfiles[0].command_type() {
            CommandType::CPUSH | CommandType::CPOP => 
                writer.write_push_pop(
                    vmfiles[0].command_type(),
                    &vmfiles[0].arg1(), // &str
                    vmfiles[0].arg2() // usize
                ),
            CommandType::CARITHMETIC => writer.write_arithmetic(&vmfiles[0].arg1()),
            _ => unimplemented!(),
        }
        vmfiles[0].advance();

    }    

    writer.close();
}