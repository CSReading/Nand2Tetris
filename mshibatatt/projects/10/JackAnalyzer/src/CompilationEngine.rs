use crate::JackTokenizer::JackTokenizer;
use crate::JackTokenizer::TokenType;

pub struct CompilationEngine {
    infile: JackTokenizer,
    pub xml: String,
    indent_num: usize,
    counter: usize,
}

impl CompilationEngine {
    pub fn new(infile: JackTokenizer) -> Self {
        Self {
            infile: infile,
            xml: String::from(""),
            indent_num: 0,
            counter: 0,
        }
    }

    pub fn token_test(&mut self) {
        self.xml += "<tokens>\n";
        while self.infile.has_more_tokens() {
            self.infile.advance();
            self.parse();
        }
        self.xml += "</tokens>";
    }

    fn indent(&mut self) {
        for _ in 0..2*self.indent_num {
            self.xml += " "; 
        }
    }

    fn parse(&mut self) {
        self.counter += 1;
        
        self.indent();
        match self.infile.token_type() {
            TokenType::KEYWORD => {
                self.xml += "<keyword> ";
                self.xml += self.infile.key_word();
                self.xml += " </keyword>\n";
            },
            TokenType::SYMBOL => {
                self.xml += "<symbol> ";
                let symbol_string = self.infile.symbol().to_string();
                self.xml += match self.infile.symbol() {
                    '<' => "&lt;",
                    '>' => "&gt;",
                    '&' => "&amp;",
                    _ => &symbol_string,
                };
                self.xml += " </symbol>\n";
            },
            TokenType::INTCONST => {
                self.xml += "<integerConstant> ";
                self.xml += &self.infile.int_val().to_string();
                self.xml += " </integerConstant>\n";
            },
            TokenType::STRINGCONST => {
                self.xml += "<stringConstant> ";
                self.xml += self.infile.string_val();
                self.xml += " </stringConstant>\n";
            },
            TokenType::IDENTIFIER => {
                self.xml += "<identifier> ";
                self.xml += self.infile.identifier();
                self.xml += " </identifier>\n";
            },
        }
    }

    pub fn compile_class(&mut self) {
        self.xml += "<class>\n";
        self.indent_num += 1;
        self.infile.advance(); // initial setup

        // parse "class"
        self.parse();
        self.infile.advance();
        // parse "className"
        self.parse();
        self.infile.advance();
        // parse "{"
        self.parse();
        self.infile.advance(); 


        loop {
            match self.infile.token_type() {
                TokenType::KEYWORD => {
                    match self.infile.key_word() {
                        "static" | "field" => self.compile_class_var_dec(),
                        "constructor" | "function" | "method" => self.compile_subroutine(),
                        _ => unreachable!(),
                    }
                },
                TokenType::SYMBOL => break,
                _ => unreachable!(),
            }
        }

        // parse "}"
        self.parse();
        self.infile.advance();

        self.indent_num -= 1;
        self.xml += "</class>\n";
    }
    
    fn compile_class_var_dec(&mut self) {
        self.indent();
        self.xml += "<classVarDec>\n";
        self.indent_num += 1;
        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    self.parse();
                    if self.infile.symbol() == ';' {
                        self.infile.advance();
                        break;
                    }
                },
                _ => self.parse(),
            }
            self.infile.advance();
        }
        self.indent_num -= 1;
        self.indent();
        self.xml += "</classVarDec>\n";
    }
    
    fn compile_subroutine(&mut self) {
        self.indent();
        self.xml += "<subroutineDec>\n";
        self.indent_num += 1;
        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    if self.infile.symbol() == '(' {
                        self.parse();
                        self.infile.advance();
                        self.compile_parameter_list();
                        self.parse(); // parse )
                        self.infile.advance();
                        break;
                    }
                },
                _ => (),
            }
            self.parse();
            self.infile.advance();
        }
        self.indent();
        self.xml += "<subroutineBody>\n";
        self.indent_num += 1;
        
        // parse "{"
        self.parse();
        self.infile.advance();        
        
        loop {
            match self.infile.token_type() {
                TokenType::KEYWORD => {
                    if self.infile.key_word() == "var" {
                        self.compile_var_dec();
                    } else {
                        break;
                    }
                },
                _ => break,
            }
        }
        self.compile_statements();
        
        // parse "}"
        self.parse();
        self.infile.advance();

        self.indent_num -= 1;
        self.indent();
        self.xml += "</subroutineBody>\n";
        self.indent_num -= 1;
        self.indent();
        self.xml += "</subroutineDec>\n";
    }
    
    fn compile_parameter_list(&mut self) { 
        self.indent();      
        self.xml += "<parameterList>\n";
        self.indent_num += 1;
        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    if self.infile.symbol() == ')' {
                        break;
                    }
                },
                _ => (),
            }
            self.parse();
            self.infile.advance();
        }
        self.indent();
        self.indent_num -= 1;
        self.xml += "</parameterList>\n";
    }
    
    fn compile_var_dec(&mut self) {
        self.xml += "<varDec>\n";
        self.indent_num += 1;
        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    self.parse();
                    if self.infile.symbol() == ';' {
                        self.infile.advance();
                        break;
                    }
                },
                _ => self.parse(),
            }
            self.infile.advance();
        }
        self.indent_num -= 1;
        self.indent();
        self.xml += "</varDec>\n";
    }
    
    fn compile_statements(&mut self) {
        self.indent();
        self.xml += "<statements>\n";
        self.indent_num += 1;
        loop {
            if self.infile.token_type() == TokenType:: KEYWORD {
                match self.infile.key_word() {
                    "let" => self.compile_let(),
                    "if" => self.compile_if(),
                    "while" => self.compile_while(),
                    "do" => self.compile_do(),
                    "return" => self.compile_return(),
                    _ => break,
                }
            } else {
                break;
            }
        }
        self.indent_num -= 1;
        self.indent();
        self.xml += "</statements>\n";
    }
    
    fn compile_do(&mut self) {
        self.indent();
        self.xml += "<doStatement>\n";
        self.indent_num += 1;
        // parse do
        self.parse();
        self.infile.advance();
        self.compile_subroutine_call();
        // parse ;
        self.parse();
        self.infile.advance();        
        self.indent_num -= 1;
        self.indent();
        self.xml += "</doStatement>\n";
    }
    
    fn compile_let(&mut self) {
        self.indent();
        self.xml += "<letStatement>\n";
        self.indent_num += 1;

        // parse "let"
        assert_eq!(self.infile.token.as_str(), "let");
        self.parse();
        self.infile.advance();

        // parse varName
        self.parse();
        self.infile.advance();

        assert!(self.infile.token_type() == TokenType::SYMBOL);
        if self.infile.symbol() == '[' {
            self.parse();
            self.infile.advance();
            self.compile_expression();
            assert_eq!(self.infile.symbol(), ']');
            self.parse();
            self.infile.advance();
        } 

        assert_eq!(self.infile.symbol(), '=');
        self.parse();
        self.infile.advance();
        self.compile_expression();

        // parse ";"
        assert_eq!(self.infile.symbol(), ';');
        self.parse();
        self.infile.advance();

        self.indent_num -= 1;
        self.indent();
        self.xml += "</letStatement>\n";
    }
    
    fn compile_while(&mut self) {
        self.indent();
        self.xml += "<whileStatement>\n";
        self.indent_num += 1;

        // parse "while"
        self.parse();
        self.infile.advance();
        // parse "("
        self.parse();
        self.infile.advance();

        self.compile_expression();

        // parse ")"
        self.parse();
        self.infile.advance();
        // parse "{"
        self.parse();
        self.infile.advance();

        self.compile_statements();

        // parse "}"
        self.parse();
        self.infile.advance();

        self.indent_num -= 1;
        self.indent();
        self.xml += "</whileStatement>\n";
    }
    
    fn compile_return(&mut self) {
        self.indent();
        self.xml += "<returnStatement>\n";
        self.indent_num += 1;
        // parse "return"
        self.parse();
        self.infile.advance();

        if self.infile.token_type() == TokenType::SYMBOL {
            if self.infile.symbol() == ';' {
                self.parse();
                self.infile.advance();
            }
        } else {
            self.compile_expression();
            // parse ";"
            self.parse();
            self.infile.advance();
        }

        self.indent_num -= 1;
        self.indent();
        self.xml += "</returnStatement>\n";
    }
    
    fn compile_if(&mut self) {
        self.indent();
        self.xml += "<ifStatement>\n";
        self.indent_num += 1;

        // parse "if"
        assert_eq!(self.infile.token.as_str(), "if");
        self.parse();
        self.infile.advance();

        // parse "("
        assert_eq!(self.infile.token.as_str(), "(");
        self.parse();
        self.infile.advance(); 

        self.compile_expression();

        // parse ")"
        assert_eq!(self.infile.token.as_str(), ")");
        self.parse();
        self.infile.advance();

        // parse "{"
        assert_eq!(self.infile.token.as_str(), "{");
        self.parse();
        self.infile.advance();

        self.compile_statements();

        // parse "}"
        assert_eq!(self.infile.token.as_str(), "}");
        self.parse();
        self.infile.advance();

        // in case "else"
        if self.infile.token_type() == TokenType::KEYWORD {
            if self.infile.key_word() == "else" {
                // parse "else"
                self.parse();
                self.infile.advance();
                
                // parse "{"
                assert_eq!(self.infile.token.as_str(), "{");
                self.parse();
                self.infile.advance();

                self.compile_statements();

                // parse "}"
                assert_eq!(self.infile.token.as_str(), "}");
                self.parse();
                self.infile.advance();                
            }
        }

        self.indent_num -= 1;
        self.indent();
        self.xml += "</ifStatement>\n";        
    }

    fn compile_subroutine_call(&mut self) {
        assert!(self.infile.token_type() == TokenType::IDENTIFIER);
        self.parse();
        self.infile.advance();

        // parse "(" or "."
        assert!(self.infile.token_type() == TokenType::SYMBOL);
        if self.infile.symbol() == '(' {
            self.parse();
            self.infile.advance();
            self.compile_expression_list();
        } else {
            assert_eq!(self.infile.symbol(), '.');
            self.parse();
            self.infile.advance();
            // parse subroutineName
            self.parse();
            self.infile.advance();
            // parse "("
            assert_eq!(self.infile.symbol(), '(');
            self.parse();
            self.infile.advance();
            self.compile_expression_list();
        }

        assert_eq!(self.infile.symbol(), ')');
        self.parse();
        self.infile.advance();
    }

    fn compile_expression(&mut self) {
        self.indent();
        self.xml += "<expression>\n";
        self.indent_num += 1;

        self.compile_term();
        
        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    // only in case Op, call compile_term again
                    match self.infile.symbol() {
                        '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=' => {
                            self.parse();
                            self.infile.advance();
                            self.compile_term();
                        },
                        _=> break,
                    }
                },
                _ => break,
            }
        }
        self.indent_num -= 1;
        self.indent();
        self.xml += "</expression>\n";
    }
    
    fn compile_term(&mut self) {
        self.indent();
        self.xml += "<term>\n";
        self.indent_num += 1;
        match self.infile.token_type() {
            TokenType::SYMBOL => {
                match self.infile.symbol() {
                    '-' | '~' => {
                        self.parse();
                        self.infile.advance();
                        self.compile_term();
                    },
                    '(' => {
                        self.parse();
                        self.infile.advance();
                        self.compile_expression();
                        self.parse();
                        self.infile.advance();
                    },
                    _ => {},                   
                }
            },
            TokenType::IDENTIFIER => {
                let current_idx = self.infile.idx;
                let current_next_idx = self.infile.next_idx;
                self.infile.advance();
                if self.infile.token_type() == TokenType::SYMBOL {
                    match self.infile.symbol() {
                        '(' | '.' => {
                            self.infile.idx = current_idx;
                            self.infile.next_idx = current_next_idx;
                            self.infile.token = self.infile.file[self.infile.idx..self.infile.next_idx].to_owned();
                            self.compile_subroutine_call();
                        },
                        '[' => {
                            self.infile.idx = current_idx;
                            self.infile.next_idx = current_next_idx;
                            self.infile.token = self.infile.file[self.infile.idx..self.infile.next_idx].to_owned();
                            self.parse(); // parse varName
                            self.infile.advance();
                            self.parse(); // parse [
                            self.infile.advance();
                            self.compile_expression();
                            self.parse(); // parse ]  
                            self.infile.advance();                           
                        }
                        _ => {
                            self.infile.idx = current_idx;
                            self.infile.next_idx = current_next_idx;
                            self.infile.token = self.infile.file[self.infile.idx..self.infile.next_idx].to_owned();
                            self.parse();
                            self.infile.advance();
                        },
                    }
                } else {
                    self.infile.idx = current_idx;
                    self.infile.next_idx = current_next_idx;
                    self.infile.token = self.infile.file[self.infile.idx..self.infile.next_idx].to_owned();
                    self.parse();
                    self.infile.advance();
                }
            },
            _ => {
                self.parse();
                self.infile.advance();
            },
        }
        self.indent_num -= 1;
        self.indent();
        self.xml += "</term>\n";
    }
    
    fn compile_expression_list(&mut self) {
        self.indent();
        self.xml += "<expressionList>\n";
        self.indent_num += 1;

        if self.infile.token_type() == TokenType::SYMBOL {
            if self.infile.symbol() == ')' {
                self.indent_num -= 1;
                self.indent();
                self.xml += "</expressionList>\n";
                return;
            }
        } 

        self.compile_expression();

        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    match self.infile.symbol() {
                        ',' => {
                            self.parse();
                            self.infile.advance();
                            self.compile_expression();
                        },
                        _ => break,
                    }
                },
                _ => break,
            }
        }

        self.indent_num -= 1;
        self.indent();
        self.xml += "</expressionList>\n";
    }

}