// CodeWriter module
use std::fs::File;
use std::io::prelude::*;
use::parser::CommandType;

pub struct CodeWriter<'a> {
    counter: usize,
    infilename: &'a str, 
    outfilename: &'a str, 
    out_code: String,

}

pub fn init(outfilename: &str) -> CodeWriter {    
    CodeWriter {
        counter: 0,
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
            "add" => output += "// add\n@SP\nD=M\nD=M+D\nM=D\n",
            "sub" => output += "// sub\n@SP\nD=M\nD=M-D\nM=D\n",
            "neg" => output += "// not\n@SP\nM=-M\n",
            "eq" => {
                let true_label = "TRUEEQ".to_owned() + &self.counter.to_string();
                let false_label = "ENDEQ".to_owned() + &self.counter.to_string();
                output += "// eq\n@SP\nD=M\nD=M-D\n@";
                output += &true_label;
                output += "D;JEQ\n@SP\nM=0\n@";
                output += &false_label;
                output += "D;JMP\n(";
                output += &true_label;
                output += ")\n@SP\nM=-1\n(";
                output += &false_label;
                output += ")\n";
                self.counter += 1;
            },
            "gt" => {
                let true_label = "TRUEGT".to_owned() + &self.counter.to_string();
                let false_label = "ENDGT".to_owned() + &self.counter.to_string();
                output += "// eq\n@SP\nD=M\nD=M-D\n@";
                output += &true_label;
                output += "D;JGT\n@SP\nM=0\n@";
                output += &false_label;
                output += "D;JMP\n(";
                output += &true_label;
                output += ")\n@SP\nM=-1\n(";
                output += &false_label;
                output += ")\n";
                self.counter += 1;
            },
            "lt" =>{
                let true_label = "TRUELT".to_owned() + &self.counter.to_string();
                let false_label = "ENDLT".to_owned() + &self.counter.to_string();
                output += "// eq\n@SP\nD=M\nD=M-D\n@";
                output += &true_label;
                output += "D;JLT\n@SP\nM=0\n@";
                output += &false_label;
                output += "D;JMP\n(";
                output += &true_label;
                output += ")\n@SP\nM=-1\n(";
                output += &false_label;
                output += ")\n";
                self.counter += 1;
            },
            "and" => output += "// and\n@SP\nD=M\nD=M&D\nM=D\n",
            "or" => output += "// or\n@SP\nD=M\nD=M|D\nM=D\n",
            "not" => output += "// not\n@SP\nM=!M\n",
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
            CommandType::CPOP => {
                if segment == "constant" {
                    output += "// pop constant ";
                    output += &(index.to_string() + "\n");
                    output += "@SP\nM=";
                    output += &(index.to_string() + "\n");
                }
            },
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