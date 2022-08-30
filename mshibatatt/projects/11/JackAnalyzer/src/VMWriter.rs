use std::fs::File;
use std::io::prelude::*;

pub enum Segment {
    CONST,
    ARG,
    LOCAL,
    STATIC,
    THIS,
    THAT,
    POINTER,
    TEMP,
}

pub enum Command {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}

pub struct VMWriter {
    vm: String,
    outfilename: String,
}

impl VMWriter {
    pub fn new(outfilename: String) -> Self {
        Self {
            vm: String::from(""),
            outfilename: outfilename,
        }
    }

    pub fn write_push(&mut self, segment: Segment, index: usize) {
        match segment {
            Segment::CONST => {
                self.vm += "push constant ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::ARG => {
                self.vm += "push argument ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::LOCAL => {
                self.vm += "push local ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::STATIC => {
                self.vm += "push static ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::THIS => {
                self.vm += "push this ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::THAT => {
                self.vm += "push that ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::POINTER => {
                self.vm += "push pointer ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::TEMP => {
                self.vm += "push temp ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
        }
    }

    pub fn write_pop(&mut self, segment: Segment, index: usize) {
        match segment {
            Segment::CONST => {
                self.vm += "pop constant ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::ARG => {
                self.vm += "pop argument ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::LOCAL => {
                self.vm += "pop local ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::STATIC => {
                self.vm += "pop static ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::THIS => {
                self.vm += "pop this ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::THAT => {
                self.vm += "pop that ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::POINTER => {
                self.vm += "pop pointer ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
            Segment::TEMP => {
                self.vm += "pop temp ";
                self.vm += &index.to_string();
                self.vm += "\n";
            },
        }
    }

    pub fn write_arithmetic(&mut self, command: Command) {
        match command {
            Command::ADD => self.vm += "add\n",
            Command::SUB => self.vm += "sub\n",
            Command::NEG => self.vm += "neg\n",
            Command::EQ => self.vm += "eq\n",
            Command::GT => self.vm += "gt\n",
            Command::LT => self.vm += "lt\n",
            Command::AND => self.vm += "and\n",
            Command::OR => self.vm += "or\n",
            Command::NOT => self.vm += "not\n",
        }
    }

    pub fn write_label(&mut self, label: &str) {
        self.vm += "label ";
        self.vm += label;
        self.vm += "\n"
    }
    
    pub fn write_goto(&mut self, label: &str) {
        self.vm += "goto ";
        self.vm += label;
        self.vm += "\n"        
    }
    
    pub fn write_if(&mut self, label: &str) {
        self.vm += "if-goto ";
        self.vm += label;
        self.vm += "\n"        
    }
    
    pub fn write_call(&mut self, name: &str, n_args: usize) {
        self.vm += "call ";
        self.vm += name;
        self.vm += " ";
        self.vm += &n_args.to_string();
        self.vm += "\n"         
    }
    
    pub fn write_function(&mut self, name: &str, n_locals: usize) {
        self.vm += "function ";
        self.vm += name;
        self.vm += " "; 
        self.vm += &n_locals.to_string();
        self.vm += "\n"          
    }
    
    pub fn write_return(&mut self) {
        self.vm += "return\n"
    }
    
    pub fn close(&mut self) {
        let mut outfile = match File::create(&self.outfilename) {
            Err(why) => panic!("couldn't create {}: {}", self.outfilename, why),
            Ok(outfile) => outfile,
        }; 

        match outfile.write_all(self.vm.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", self.outfilename, why),
            Ok(_) =>println!("successfully write to {}", self.outfilename),
        };
    }
}