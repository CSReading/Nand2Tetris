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

pub fn init(contents: &mut String) -> Parser{
    let vec: Vec<&str> = contents.split('\n').collect::<Vec<&str>>().iter().map(|s| s.trim()).collect();
    let size: usize = vec.len(); 
    Parser {
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

impl Parser<'_> {
    pub fn has_more_commands(&self) -> bool {
        if self.next_line >= self.contents_size {
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
                "lt" | "and" | "or" | "not" => CommandType::CARITHMETIC,
            "push" => CommandType::CPUSH,
            "pop" => CommandType::CPOP,
            "label" => CommandType::CLABEL,
            "goto" => CommandType::CGOTO,
            "if-goto" => CommandType::CIF,
            "function" => CommandType::CFUNCTION,
            "call" => CommandType::CCALL,
            "return" => CommandType::CRETURN,
            _ => panic!("invalid argument {} !", self.current_command.to_owned()),
        }
    }

    pub fn arg1(&self) -> String {
        let current_command = self.current_command.to_owned();
        let parsed_command: Vec<&str> = current_command.split(" ").collect();
        match self.command_type() {
            CommandType::CRETURN => panic!("trying parse arg1 in return command!!"),
            CommandType::CARITHMETIC => parsed_command[0].to_owned(),
            _ => parsed_command[1].to_owned()
        }
    }

    pub fn arg2(&self) -> usize {
        let current_command = self.current_command.to_owned();
        let parsed_command: Vec<&str> = current_command.split(" ").collect();
        match self.command_type() {
            CommandType::CPUSH |
            CommandType::CPOP |
            CommandType::CFUNCTION | 
            CommandType::CCALL => 
                parsed_command[2].parse::<usize>().unwrap(),
            _ => panic!("No second argument!!")
        }
    }
}