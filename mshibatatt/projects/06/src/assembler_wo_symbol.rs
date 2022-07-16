use std::env;
use std::fs::File;
use std::io::prelude::*;
mod parser;
mod code;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    let infilename = &args[1];
    let outfilename = &args[2];
    
    let mut f = File::open(infilename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut instructions: String = String::from("");
    let mut command: parser::Command = parser::Command::init(&mut contents);
    command.advance();
    while command.has_more_commands() {
        let command_type: parser::CommandType = command.command_type();
        let code = match command_type {
            parser::CommandType::ACOMMAND => {
                "0".to_owned() + 
                &format!("{:0>15b}", command.symbol().to_string().parse::<u16>().unwrap())
            },
            parser::CommandType::CCOMMAND => {
                "111".to_owned() + 
                code::comp(command.comp()) + 
                code::dest(command.dest()) + 
                code::jump(command.jump())
            },
            parser::CommandType::LCOMMAND => command.symbol().to_string(),
        };
        command.advance();
        instructions.push_str(&code);
        instructions.push_str("\n");
    }

    let mut outfile = match File::create(outfilename) {
        Err(why) => panic!("couldn't create {}: {}", outfilename, why),
        Ok(outfile) => outfile,
    }; 

    match outfile.write_all(instructions.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", outfilename, why),
        Ok(_) =>println!("successfully write to {}", outfilename),
    }
}