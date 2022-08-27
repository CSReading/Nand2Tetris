use std::collections::HashSet;
use std::cmp::min;

#[derive(PartialEq)]
pub enum TokenType {
    KEYWORD,
    SYMBOL,
    IDENTIFIER,
    INTCONST,
    STRINGCONST,
}

lazy_static! {
    static ref SYMBOLS: HashSet<&'static str> = {
        vec![
            "{", "}", "(", ")", "[", "]", ".", ",", ";", "+",
            "-", "*", "/", "&", "|", "<", ">", "=", "~"
        ].into_iter().collect()
    };
}


lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        vec![
            "class", "constructor", "function", "method",
            "field", "static", "var", "int", "char",
            "boolean", "void", "true", "false", "null", "this",
            "let", "do", "if", "else", "while", "return"
        ].into_iter().collect()
    };
}


pub struct JackTokenizer {
    pub file: String,
    pub token: String,
    pub size: usize,
    pub idx: usize,
    pub next_idx: usize,
}

impl JackTokenizer {
    pub fn new(input: String) -> Self {
        let n = input.len();
        Self {
            file: input,
            token: String::new(),
            size: n,
            idx: 0,
            next_idx: 0,
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        self.next_idx < self.size
    }

    pub fn advance(&mut self) {
        assert!(self.has_more_tokens());
        self.idx = self.next_idx;
        self.skip_comment();

        if self.file.chars().nth(self.idx) == Some('"') {
            // find " if string value 
            loop { 
                if self.next_idx >= self.size { break; }
                if self.file.chars().nth(self.next_idx) == Some('"') { 
                    self.next_idx += 1;
                    break; 
                }
            
                self.next_idx += 1;
            }
        } else {
            loop { 
                if self.next_idx >= self.size { break; }
                if SYMBOLS.contains(self.file.chars().nth(self.idx).unwrap().to_string().as_str()) { break; }
                if SYMBOLS.contains(self.file.chars().nth(self.next_idx).unwrap().to_string().as_str()) { break; }
                if self.file.chars().nth(self.next_idx).unwrap().to_string().contains(char::is_whitespace) { break; }
            
                self.next_idx += 1;
            }
        }
        self.token = self.file[self.idx..self.next_idx].to_owned();
    }

    fn skip_comment(&mut self) {
        // skip comment out and blank
        // self.idx is replaced by self.idx if it's not comment, othereise index after the comment
        
        // skip whitespace
        while self.file.chars().nth(self.idx).unwrap().to_string().contains(char::is_whitespace) {
            self.idx += 1;
            if self.idx >= self.size - 1 {
                self.next_idx = self.idx;
                return;
            }
        }  

        // skip comment out
        let mut line_comment = false;
        self.idx = match &self.file[self.idx..self.idx+2] {
            "//" => {
                line_comment = true;
                match self.file[self.idx..].find('\n') {
                    Some(i) => i + 1 + self.idx,
                    None => self.size-1,
                }
            },
            "/*" => {
                line_comment = true;
                self.file[self.idx..].find("*/").unwrap() + 2 + self.idx
            },
            _ => self.idx,
        };

        self.next_idx = self.idx + 1;
        if line_comment {
            self.skip_comment();
        }
    }

    pub fn token_type(&self) -> TokenType {
        let output: TokenType;
        
        if KEYWORDS.contains(self.token.as_str()) {
            output = TokenType::KEYWORD;
        } else if SYMBOLS.contains(self.token.as_str()) {
            output = TokenType::SYMBOL;
        } else if self.token.parse::<u32>().is_ok() {
            output = TokenType::INTCONST;
        } else if self.token.chars().nth(0) == Some('\"') {
            output = TokenType::STRINGCONST;
        } else {
            output = TokenType::IDENTIFIER;
        } 

        output
    }

    pub fn key_word(&self) -> &str {
        // assert_eq!(self.token_type(), TokenType::KEYWORD);
        &self.token
    }

    pub fn symbol(&self) -> char {
        // assert_eq!(self.token_type(), TokenType::SYMBOL);
        self.token.chars().nth(0).unwrap()
    }

    pub fn identifier(&self) -> &str {
        // assert_eq!(self.token_type(), TokenType::IDENTIFIER);
        &self.token
    }

    pub fn int_val(&self) -> u32 {
        // assert_eq!(self.token_type(), TokenType::INTCONST);
        self.token.parse::<u32>().unwrap()
    }

    pub fn string_val(&self) -> &str {
        // assert_eq!(self.token_type(), TokenType::STRINGCONST);
        (&self.token).trim_start_matches("\"").trim_end_matches("\"")
    }

}