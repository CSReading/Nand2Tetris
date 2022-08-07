// CodeWriter module
use std::fs::File;
use std::io::prelude::*;
use::parser::CommandType;

pub struct CodeWriter<'a> {
    counter: usize,
    infilename: String, 
    outfilename: &'a str, 
    out_code: String,

}

pub fn init(outfilename: &str) -> CodeWriter {    
    CodeWriter {
        counter: 0,
        infilename: String::from(""),
        outfilename: outfilename,
        out_code: String::from(""),
    }
}

impl<'a>  CodeWriter<'a> {
    pub fn set_file_name(&mut self, file_name: String) {
        self.infilename = file_name;
    }

    pub fn write_init(&mut self) {
        self.out_code += "@256\nD=A\n@SP\nM=D\n";
        self.write_call("Sys.init", 0);
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        let mut output = String::from("");
        match command {
            "add" => output += "// add\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M+D\n",
            "sub" => output += "// sub\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n",
            "neg" => output += "// not\n@SP\nAM=M-1\nM=-M\n@SP\nM=M+1\n",
            "eq" => {
                let true_label = "__TRUEEQ".to_owned() + &self.counter.to_string();
                let false_label = "__ENDEQ".to_owned() + &self.counter.to_string();
                output += "// eq\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@";
                output += &true_label;
                output += "\nD;JEQ\n@SP\nAM=M-1\nM=0\n@";
                output += &false_label;
                output += "\nD;JMP\n(";
                output += &true_label;
                output += ")\n@SP\nAM=M-1\nM=-1\n(";
                output += &false_label;
                output += ")\n@SP\nM=M+1\n";
                self.counter += 1;
            },
            "gt" => {
                let true_label = "__TRUEGT".to_owned() + &self.counter.to_string();
                let false_label = "__ENDGT".to_owned() + &self.counter.to_string();
                output += "// gt\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@";
                output += &true_label;
                output += "\nD;JGT\n@SP\nAM=M-1\nM=0\n@";
                output += &false_label;
                output += "\nD;JMP\n(";
                output += &true_label;
                output += ")\n@SP\nAM=M-1\nM=-1\n(";
                output += &false_label;
                output += ")\n@SP\nM=M+1\n";
                self.counter += 1;
            },
            "lt" =>{
                let true_label = "__TRUELT".to_owned() + &self.counter.to_string();
                let false_label = "__ENDLT".to_owned() + &self.counter.to_string();
                output += "// lt\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@";
                output += &true_label;
                output += "\nD;JLT\n@SP\nAM=M-1\nM=0\n@";
                output += &false_label;
                output += "\nD;JMP\n(";
                output += &true_label;
                output += ")\n@SP\nAM=M-1\nM=-1\n(";
                output += &false_label;
                output += ")\n@SP\nM=M+1\n";
                self.counter += 1;
            },
            "and" => output += "// and\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M&D\n",
            "or" => output += "// or\n@SP\nAM=M-1\nD=M\nA=A-1\nM=M|D\n",
            "not" => output += "// not\n@SP\nAM=M-1\nM=!M\n@SP\nM=M+1\n",
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
                    output += "@LCL\nD=M\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
                },
                "argument" => {
                    output += "// push argument[";
                    output += &(index.to_string() + "]\n");
                    output += "@ARG\nD=M\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
                },
                "this" => {
                    output += "// push this[";
                    output += &(index.to_string() + "]\n");
                    output += "@THIS\nD=M\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
                },
                "that" => {
                    output += "// push that[";
                    output += &(index.to_string() + "]\n");
                    output += "@THAT\nD=M\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
                },
                "pointer" => {
                    output += "// push pointer[";
                    output += &(index.to_string() + "]\n");
                    output += "@3\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
                },
                "temp" => {
                    output += "// push temp[";
                    output += &(index.to_string() + "]\n");
                    output += "@5\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "A=D+A\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
                },
                "static" => {
                    output += "// push static.";
                    output += &(index.to_string() + "]\n@");
                    output += &self.infilename;
                    output += ".";
                    output += &(index.to_string() + "\nD=M\n");
                    output += "@SP\nA=M\nM=D\n@SP\nM=M+1\n";
                },
                _ => panic!("Unknown push segment {}!!", segment),
            },
            CommandType::CPOP => match segment{
                "local" => {
                    output += "// pop local[";
                    output += &(index.to_string() + "]\n");
                    output += "@LCL\nD=M\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
                },
                "argument" => {
                    output += "// pop argument[";
                    output += &(index.to_string() + "]\n");
                    output += "@ARG\nD=M\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
                },
                "this" => {
                    output += "// pop this[";
                    output += &(index.to_string() + "]\n");
                    output += "@THIS\nD=M\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
                },
                "that" => {
                    output += "// pop that[";
                    output += &(index.to_string() + "]\n");
                    output += "@THAT\nD=M\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
                },
                "pointer" => {
                    output += "// pop pointer[";
                    output += &(index.to_string() + "]\n");
                    output += "@3\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
                },
                "temp" => {
                    output += "// pop temp[";
                    output += &(index.to_string() + "]\n");
                    output += "@5\nD=A\n@";
                    output += &(index.to_string() + "\n");
                    output += "D=D+A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
                },
                "static" => {
                    output += "// pop static.";
                    output += &(index.to_string() + "\n@SP\nAM=M-1\nD=M\n@");
                    output += &self.infilename;
                    output += ".";
                    output += &(index.to_string() + "\nM=D\n");
                },
                _ => panic!("Unknown pop segment {}!!", segment),
            },
            _ => panic!("Command neither push or pop!"),
        }
        self.out_code += &output;
    }

    pub fn write_label(&mut self, label: &str) {
        let mut output = String::from("");
        output += "// label \n(";
        output += label;
        output += ")\n";
        self.out_code += &output;
    }

    pub fn write_goto(&mut self, label: &str) {
        let mut output = String::from("");
        output += "// goto\n@";
        output += label;
        output += "\nD;JMP\n";
        self.out_code += &output;
    }  
    
    pub fn write_if(&mut self, label: &str) {
        let mut output = String::from("");
        output += "// if-goto\n@SP\nAM=M-1\nD=M\n@";
        output += label;
        output += "\nD;JNE\n";
        self.out_code += &output;
    }
    
    pub fn write_call(&mut self, function_name: &str, num_args: usize) {
        let mut output = String::from("");
        output += "// call ";
        output += function_name;
        output += "\n@return-address-"; 
        output += &(self.counter.to_string() + "\n");
        output += "D=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"; // push return-address
        output += "@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"; // push LCL
        output += "@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"; // push ARG
        output += "@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"; // push THIS
        output += "@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"; // push THAT
        output += "@SP\nD=M\n@";
        output += &((num_args + 5).to_string() + "\nD=D-A\n@ARG\nM=D\n"); // ARG = SP-n-5
        output += "@SP\nD=M\n@LCL\nM=D\n"; // LCL = SP 
        output += "@";
        output += function_name;
        output += "\nD;JMP\n"; // goto f
        output += "(return-address-"; 
        output += &(self.counter.to_string() + ")\n");
        self.counter += 1;
        self.out_code += &output;
    }

    pub fn write_return(&mut self) {
        let mut output = String::from("");
        output += "// return\n";
        output += "@LCL\nD=M\n@R13\nM=D\n"; // R13 = LCL
        output += "@5\nA=D-A\nD=M\n@R14\nM=D\n"; // R14 = return-address
        output += "@ARG\nD=M\n@R15\nM=D\n@SP\nA=M-1\nD=M\n@R15\nA=M\nM=D\n"; // *ARG = pop()
        output += "@ARG\nD=M\n@SP\nM=D+1\n"; // SP = ARG+1
        output += "@R13\nAM=M-1\nD=M\n@THAT\nM=D\n"; // R13--; THAT = *R13;
        output += "@R13\nAM=M-1\nD=M\n@THIS\nM=D\n"; // R13--; THIS = *R13;
        output += "@R13\nAM=M-1\nD=M\n@ARG\nM=D\n"; // R13--; ARG = *R13;
        output += "@R13\nAM=M-1\nD=M\n@LCL\nM=D\n"; // R13--; LCL = *R13;
        output += "@R14\nA=M\nD;JMP\n"; // goto return address
        self.out_code += &output;
    }

    pub fn write_function(&mut self, function_name: &str, num_args: usize) {
        let mut output = String::from("");
        let push_0 = "@0\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
        output += "// define function\n(";
        output += function_name;
        output += ")\n";
        for _ in 0..num_args {
            output += push_0;
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