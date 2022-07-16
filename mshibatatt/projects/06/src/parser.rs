// Parser module
 
#[derive(Debug)]
#[derive(PartialEq)]
pub enum CommandType {
    ACOMMAND,
    CCOMMAND,
    LCOMMAND,
}

pub struct Command<'a> {
    vec: Vec<&'a str>,
    current_command: &'a str,
    pub next_line: usize,
    contents_size: usize,
}

impl Command<'_> {

    pub fn init(contents: &mut String) -> Command {
        let vec: Vec<&str> = contents.split('\n').collect::<Vec<&str>>().iter().map(|s| s.trim()).collect();
        let size: usize = vec.len(); 
        Command {
            vec: vec.clone(),
            current_command: "",
            next_line: 0,
            contents_size: size,
        }
    }

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
            _ => &self.current_command[..comment.unwrap()].trim(),
        };

        if self.current_command == "" {
            self.advance();
        }
    }

    pub fn command_type(&self) -> CommandType {
        if self.current_command.chars().nth(0) == Some('@') {
            CommandType::ACOMMAND
        } else if self.current_command.chars().nth(0) == Some('(') {
            CommandType::LCOMMAND
        } else {
            CommandType::CCOMMAND
        }
    }

    pub fn symbol(&self) -> &str {
        let command_type: CommandType = self.command_type();
        assert_ne!(command_type, CommandType::CCOMMAND);
        if command_type == CommandType::ACOMMAND {
            &self.current_command[1..]
        } else {
            let symbol: String = self.current_command.to_string();
            &self.current_command[1..symbol.len()-1]
        } 
    }

    pub fn dest(&self) -> &str {
        assert_eq!(self.command_type(), CommandType::CCOMMAND);
        let position = self.current_command.find('=');
        match position {
            None => "null",
            _ => &self.current_command[0..position.unwrap()],
        }
    }

    pub fn comp(&self) -> &str {
        assert_eq!(self.command_type(), CommandType::CCOMMAND);
        let position_start = self.current_command.find('=');
        let position_end = self.current_command.find(';');
        let i_start = match position_start {
            None => 0,
            _ => position_start.unwrap()+1,
        };
        let i_end = match position_end {
            None => self.current_command.len(),
            _ => position_end.unwrap(),
        };

        &self.current_command[i_start..i_end]
    }

    pub fn jump(&self) -> &str {
        assert_eq!(self.command_type(), CommandType::CCOMMAND);
        let position = self.current_command.find(';');
        match position {
            None => "null",
            _ => &self.current_command[position.unwrap()+1..],
        }
    }
}
