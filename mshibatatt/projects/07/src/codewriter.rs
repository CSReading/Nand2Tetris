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
            "add" => output += "// add\n@SP\nM=M-1\nA=M\nD=M\nA=A-1\nM=D+M\n",
            "sub" => output += "// sub\n@SP\nM=M-1\nA=M\nD=M\nA=A-1\nM=D-M\n",
            "neg" => output += "// not\n@SP\nM=M-1\nA=M\nM=-M\n",
            "eq" => {
                let true_label = "TRUEEQ".to_owned() + &self.counter.to_string();
                let false_label = "ENDEQ".to_owned() + &self.counter.to_string();
                output += "// eq\n@SP\nM=M-1\nA=M\nD=M\nA=A-1\nD=D-M\n@";
                output += &true_label;
                output += "\nD;JEQ\n@SP\nA=M\nM=1\n@";
                output += &false_label;
                output += "\nD;JMP\n(";
                output += &true_label;
                output += ")\n@SP\nA=M\nM=-1\n(";
                output += &false_label;
                output += ")\n";
                self.counter += 1;
            },
            "gt" => {
                let true_label = "TRUEGT".to_owned() + &self.counter.to_string();
                let false_label = "ENDGT".to_owned() + &self.counter.to_string();
                output += "// gt\n@SP\nM=M-1\nA=M\nD=M\nA=A-1\nD=D-M\n@";
                output += &true_label;
                output += "\nD;JGT\n@SP\nA=M\nM=1\n@";
                output += &false_label;
                output += "\nD;JMP\n(";
                output += &true_label;
                output += ")\n@SP\nA=M\nM=-1\n(";
                output += &false_label;
                output += ")\n";
                self.counter += 1;
            },
            "lt" =>{
                let true_label = "TRUELT".to_owned() + &self.counter.to_string();
                let false_label = "ENDLT".to_owned() + &self.counter.to_string();
                output += "// lt\n@SP\nM=M-1\nA=M\nD=M\nA=A-1\nD=D-M\n@";
                output += &true_label;
                output += "\nD;JLT\n@SP\nA=M\nM=1\n@";
                output += &false_label;
                output += "\nD;JMP\n(";
                output += &true_label;
                output += ")\n@SP\nA=M\nM=-1\n(";
                output += &false_label;
                output += ")\n";
                self.counter += 1;
            },
            "and" => output += "// and\n@SP\nM=M-1\nA=M\nD=M\nA=A-1\nM=D&M\n",
            "or" => output += "// or\n@SP\nM=M-1\nA=M\nD=M\nA=A-1\nM=D|M\n",
            "not" => output += "// not\n@SP\nM=M-1\nA=M\nM=!M\n",
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
            CommandType::CPUSH => match segment {
                "constant" => {
                    output += "// push constant ";
                    output += &(index.to_string() + "\n");
                    output += "@";
                    output += &(index.to_string() + "\n");
                    output += "D=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
                },
                "local" => {
                    output += "// push local[";
                    output += &(index.to_string() + "]\n");
                    output += "@LCL\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nM=D\n";
                },
                "argument" => {
                    output += "// push argument[";
                    output += &(index.to_string() + "]\n");
                    output += "@ARG\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nM=D\n";
                },
                "this" => {
                    output += "// push this[";
                    output += &(index.to_string() + "]\n");
                    output += "@THIS\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nM=D\n";
                },
                "that" => {
                    output += "// push that[";
                    output += &(index.to_string() + "]\n");
                    output += "@THAT\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nM=D\n";
                },
                "pointer" => {
                    output += "// push pointer[";
                    output += &(index.to_string() + "]\n");
                    output += "@3\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\n\nD=M\n@SP\nM=D\n";
                },
                "temp" => {
                    output += "// push temp[";
                    output += &(index.to_string() + "]\n");
                    output += "@5\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nM=D\n";
                },
                "static" => {
                    output += "// push static.";
                    output += &(index.to_string() + "]\n@");
                    output += self.infilename;
                    output += ".";
                    output += &(index.to_string() + "\nD=M\n");
                    output += "@SP\nM=D\n";
                },
                _ => panic!("Unknown push segment {}!!", segment),
            },
            CommandType::CPOP => match segment{
                "local" => {
                    output += "// pop local[";
                    output += &(index.to_string() + "]\n");
                    output += "@LCL\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nD=M\n@R13\nA=M\nM=D\n";
                },
                "argument" => {
                    output += "// pop argument[";
                    output += &(index.to_string() + "]\n");
                    output += "@ARG\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nD=M\n@R13\nA=M\nM=D\n";
                },
                "this" => {
                    output += "// pop this[";
                    output += &(index.to_string() + "]\n");
                    output += "@THIS\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nD=M\n@R13\nA=M\nM=D\n";
                },
                "that" => {
                    output += "// pop that[";
                    output += &(index.to_string() + "]\n");
                    output += "@THAT\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nD=M\n@R13\nA=M\nM=D\n";
                },
                "pointer" => {
                    output += "// pop pointer[";
                    output += &(index.to_string() + "]\n");
                    output += "@3\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nD=M\n@R13\nA=M\nM=D\n";
                },
                "temp" => {
                    output += "// pop temp[";
                    output += &(index.to_string() + "]\n");
                    output += "@5\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nD=M\n@R13\nA=M\nM=D\n";
                },
                "static" => {
                    output += "// pop static.";
                    output += "@SP\nD=M\n";
                    output += &(index.to_string() + "]\n@");
                    output += self.infilename;
                    output += ".";
                    output += &(index.to_string() + "\nM=D\n");
                },
                _ => panic!("Unknown pop segment {}!!", segment),
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