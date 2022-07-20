// Parser module

pub enum CommandType {
    CARITHMETIC,
    CPUSH,
    CPOP,
    CLABEL,
    CGOTO,
    CIF,
    CFUNCTION,
    CRETURN,
    CCALL,
}

pub fn init(contents: &mut String) -> Parser {
    let vec: Vec<&str> = contents.split('\n').collect::<Vec<&str>>().iter().map(|s| s.trim()).collect();
    let size: usize = vec.len(); 
    Command {
        vec: vec.clone(),
        current_command: "",
        next_line: 0,
        contents_size: size,
    }
}

pub struct Parser<'a> {
    vec: Vec<&'a str>,
    current_command: &'a str,
    pub next_line: usize,
    contents_size: usize,
}

impl Parser {
    pub fn has_more_commands(&self) -> bool {
        if self.next_line > self.contents_size {
            false
        } else {
            true
        }
    }

    pub fn advance(&mut self) {
        if !self.has_more_commands(){
            return;
        }

        self.current_command = self.vec[self.next_line];
        self.next_line += 1;

        let comment = self.current_command.find("//");
        self.current_command = match comment {
            None => self.current_command,
            _ => &self.current_command[..comment.unwrap()].trim()
        };

        if self.current_command == "" {
            self.advance();
        }
    }

    pub fn command_type(&self) -> CommandType {
        match self.current_command.to_owned()
            .split(" ").collect::<Vec<&str>>()[0] {
            "add" | "sub" | "neg" | "eq" | "gt" | 
                "lt" | "and" | "or" | "not" => CARITHMETIC,
            "push" => CPUSH,
            "pop" => CPOP,
            "label" => CLABEL,
            "goto" => CGOTO,
            "if-goto" => CIF,
            "function" => CFUNCTION,
            "call" => CCALL,
            "return" => CRETURN,
        }
    }

    pub fn arg1(&self) -> String {
        let parsed_command: Vec<&str> = 
            self.current_command.to_owned().split(" ").collect();
        match self.command_type() {
            CRETURN => panic!("trying parse arg1 in return command!!"),
            CARITHMETIC => parsed_command[0].to_owned(),
            _ => parsed_command[1].to_owned()
        }
    }

    pub fn arg2() -> usize {
        let parsed_command: Vec<&str> = 
            self.current_command.to_owned().split(" ").collect();
        match self.command_type() {
            CPUSH | CPOP | CFUNCTION | CCALL => parsed_command[1].parse::<usize>().unwrap(),
            _ => panic!("No second argument!!")
        }
    }
}