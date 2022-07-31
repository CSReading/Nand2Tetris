// CodeWriter module
use std::fs::File;
use std::io::prelude::*;
use::parser::CommandType;

pub struct CodeWriter<'a> {
    sp: usize,
    infilename: &'a str, 
    outfilename: &'a str, 
    out_code: String,

}

pub fn init(outfilename: &str) -> CodeWriter {    
    CodeWriter {
        sp: 0,
        infilename: "",
        outfilename: outfilename,
        out_code: String::from(""),
    }
}

impl CodeWriter<'_> {
    pub fn set_file_name(&mut self, file_name: &'static str) {
        self.infilename = file_name;
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        let mut output = String::from("");
        match command {
            "add" => output += "// add\n@SP\nD=M\nD=D+M\nM=D\n",
            "sub" => unimplemented!(),
            "neg" => unimplemented!(),
            "eq" => unimplemented!(),
            "gt" => unimplemented!(),
            "lt" => unimplemented!(),
            "and" => unimplemented!(),
            "or" => unimplemented!(),
            "not" => unimplemented!(),
            _ => panic!("invalid arg {}!", command),
        }
        self.out_code += &output;
    }

    pub fn write_push_pop(
        &mut self,
        command: CommandType,
        segment: &str,
        index: usize
    ) {
        let mut output = String::from("");
        match command {
            CommandType::CPUSH => {
                if segment == "constant" {
                    output += "// push constant ";
                    output += &(index.to_string() + "\n");
                    output += "@SP\nM=";
                    output += &(index.to_string() + "\n");
                }
            },
            CommandType::CPOP => unimplemented!(),
            _ => panic!("Command neither push or pop!"),
        }
        self.out_code += &output;
    }

    pub fn close(&self) {
        let mut outfile = match File::create(self.outfilename) {
            Err(why) => panic!("couldn't create {}: {}", self.outfilename, why),
            Ok(outfile) => outfile,
        }; 

        match outfile.write_all(self.out_code.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", self.outfilename, why),
            Ok(_) =>println!("successfully write to {}", self.outfilename),
        };
    }
}