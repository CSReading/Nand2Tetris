use std::env;
use std::fs::File;
use std::io::prelude::*;
mod parser; 
mod code; 
mod symbol_table;

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

    // first path
    let mut address_number: u16 = 0;
    let mut symbol_table: symbol_table::SymbolTable = symbol_table::SymbolTable::init();

    command.advance();
    while command.has_more_commands() {
        let command_type: parser::CommandType = command.command_type();
        match command_type {
            parser::CommandType::ACOMMAND => address_number += 1,
            parser::CommandType::CCOMMAND => address_number += 1,
            parser::CommandType::LCOMMAND => {
                let symbol = command.symbol().to_string();
                symbol_table.add_entry(&symbol, address_number)
            },
        };
        command.advance();
    }    

    // second path
    command.next_line = 0;
    let mut next_ram: u16 = 16;
    command.advance();
    while command.has_more_commands() {
        let command_type: parser::CommandType = command.command_type();
        let code = match command_type {
            parser::CommandType::ACOMMAND => {
                let symbol = command.symbol().to_string();
                let number = match symbol.parse::<u16>() {
                    Ok(_) => symbol.parse::<u16>().unwrap(),
                    Err(_) => {
                        if !symbol_table.contains(&symbol) {
                            symbol_table.add_entry(&symbol, next_ram);
                            next_ram += 1;
                        }
                        symbol_table.get_address(&symbol)
                    }
                };
                "0".to_owned() + &format!("{:0>15b}", number)
            },
            parser::CommandType::CCOMMAND => {
                "111".to_owned() + 
                code::comp(command.comp()) + 
                code::dest(command.dest()) + 
                code::jump(command.jump())
            },
            parser::CommandType::LCOMMAND => {
                command.advance();        
                continue;
            },
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