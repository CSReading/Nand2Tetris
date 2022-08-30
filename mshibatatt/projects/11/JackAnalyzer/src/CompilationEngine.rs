use crate::JackTokenizer::JackTokenizer;
use crate::JackTokenizer::TokenType;
use crate::SymbolTable::SymbolTable;
use crate::SymbolTable::Kind;
use crate::VMWriter::VMWriter;
use crate::VMWriter::Segment;
use crate::VMWriter::Command;

pub struct CompilationEngine {
    infile: JackTokenizer,
    class_name: String,
    indent_num: usize,
    symbol_table: SymbolTable,
    pub writer: VMWriter,
    label_counter: usize,
}

impl CompilationEngine {
    pub fn new(infile: JackTokenizer, outfilename: &str) -> Self {
        Self {
            infile: infile,
            class_name: String::from(""),
            indent_num: 0,
            symbol_table: SymbolTable::new(),
            writer: VMWriter::new(outfilename.to_owned()),
            label_counter: 0,
        }
    }

    pub fn compile_class(&mut self) {
        self.infile.advance(); // initial setup

        assert_eq!(self.infile.key_word(), "class");
        self.infile.advance();

        // save "className"
        assert!(self.infile.token_type() == TokenType::IDENTIFIER);
        self.class_name += self.infile.identifier();
        self.infile.advance();
        
        assert_eq!(self.infile.symbol(), '{');
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
        assert_eq!(self.infile.symbol(), '}');
        self.infile.advance();
    }
    
    fn compile_class_var_dec(&mut self) {
        assert!(self.infile.token_type() == TokenType::KEYWORD);
        let kind_string = self.infile.key_word().to_owned();
        self.infile.advance();

        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => match self.infile.symbol() {
                    ';' => {
                        self.infile.advance();
                        break;
                    },
                    ',' => {
                        self.infile.advance();
                    },
                    _ => unreachable!(), 
                },
                _ => {
                    // define variables
                    let kind = match kind_string.as_str() {
                        "static" => Kind::STATIC,
                        "field" => Kind::FIELD,
                        _ => unreachable!(),
                    };

                    let type_ = self.infile.token.to_owned();
                    self.infile.advance();
                    assert!(self.infile.token_type() == TokenType::IDENTIFIER);
                    let var_name = self.infile.identifier();
                    self.symbol_table.define(var_name, &type_, kind);
                }
            }
        }
    }
    
    fn compile_subroutine(&mut self) {
        assert!(self.infile.token_type() == TokenType::KEYWORD);
        self.infile.advance();

        let is_void = self.infile.token_type() == TokenType::KEYWORD;
        if is_void {
            assert_eq!(self.infile.key_word(), "void");
        }
        self.infile.advance();

        assert!(self.infile.token_type() == TokenType::IDENTIFIER);
        let mut subroutine_name = self.class_name.to_owned();
        subroutine_name += ".";
        subroutine_name += self.infile.identifier();
        self.symbol_table.define("this", &self.class_name, Kind::ARG);
        self.infile.advance();

        assert_eq!(self.infile.symbol(), '(');
        self.infile.advance();
        self.compile_parameter_list();
        assert_eq!(self.infile.symbol(), ')');
        self.infile.advance();
        self.writer.write_function(&subroutine_name, self.symbol_table.var_count(&Kind::ARG));
        // set Argument 0 as Pointer 0 NO Need?
        // self.writer.write_push(Segment::ARG, 0);
        // self.writer.write_pop(Segment::POINTER, 0);
    
        // subroutineBody    
        assert_eq!(self.infile.symbol(), '{');
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
        
        assert_eq!(self.infile.symbol(), '}');
        self.infile.advance();
    }
    
    fn compile_parameter_list(&mut self) { 
        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    if self.infile.symbol() == ')' {
                        break;
                    }
                },
                _ => (),
            }
            let type_ = &self.infile.token.to_owned();
            self.infile.advance();
            assert!(self.infile.token_type() == TokenType::IDENTIFIER);
            self.symbol_table.define(&self.infile.identifier(), type_, Kind::ARG);
            self.infile.advance();
        }
    }
    
    fn compile_var_dec(&mut self) {
        assert_eq!(self.infile.key_word(), "var");
        self.infile.advance();
        let type_ = &self.infile.token.to_owned();
        self.infile.advance();
        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    match self.infile.symbol() {
                        ';' =>  {
                            self.infile.advance();
                            break;
                        },
                        ',' => self.infile.advance(),
                        _ => unreachable!(),
                    }
                },
                TokenType::IDENTIFIER => {
                    self.symbol_table.define(&self.infile.identifier(), type_, Kind::VAR);
                    self.infile.advance();
                },
                _ => unreachable!(),
            }
        }
    }
    
    fn compile_statements(&mut self) {
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
    }
    
    fn compile_do(&mut self) {
        assert_eq!(self.infile.key_word(), "do");
        self.infile.advance();
        self.compile_subroutine_call();
        assert_eq!(self.infile.symbol(), ';');
        self.infile.advance();
    }
    
    fn compile_let(&mut self) {
        assert_eq!(self.infile.token.as_str(), "let");
        self.infile.advance();

        // check varName
        assert!(self.infile.token_type() == TokenType::IDENTIFIER);
        let var_name = self.infile.identifier();
        let kind = self.symbol_table.kind_of(&var_name);
        let index = self.symbol_table.index_of(&var_name); 
        self.infile.advance();

        assert!(self.infile.token_type() == TokenType::SYMBOL);
        if self.infile.symbol() == '[' {
            self.infile.advance();
            self.compile_expression();
            assert_eq!(self.infile.symbol(), ']');
            self.infile.advance();
        } 

        assert_eq!(self.infile.symbol(), '=');
        self.infile.advance();
        self.compile_expression();

        // TODO; assignemnt by VM syntax

        assert_eq!(self.infile.symbol(), ';');
        self.infile.advance();
    }
    
    fn compile_while(&mut self) {

        assert_eq!(self.infile.key_word(), "while");
        let mut loop_label = "while_loop_".to_owned();
        loop_label += &self.label_counter.to_string();
        let mut out_label = "while_out_".to_owned();
        out_label += &self.label_counter.to_string();
        self.label_counter += 1;

        self.writer.write_label(&loop_label);
        self.infile.advance();
        
        assert_eq!(self.infile.symbol(), '(');
        self.infile.advance();

        self.compile_expression();
        // TODO: check condition to break
        self.writer.write_if(&out_label);

        assert_eq!(self.infile.symbol(), ')');
        self.infile.advance();

        assert_eq!(self.infile.symbol(), '{');
        self.infile.advance();

        self.compile_statements();

        assert_eq!(self.infile.symbol(), '}');
        self.infile.advance();

        self.writer.write_goto(&loop_label);
        self.writer.write_label(&out_label);
    }
    
    fn compile_return(&mut self) {
        assert_eq!(self.infile.key_word(), "return");
        self.infile.advance();

        if self.infile.token_type() == TokenType::SYMBOL {
            if self.infile.symbol() == ';' {
                self.writer.write_push(Segment::CONST, 0);
                self.infile.advance();
            }
        } else {
            self.compile_expression();
            assert_eq!(self.infile.symbol(), ';');
            self.infile.advance();
        }
        self.writer.write_return();
    }
    
    fn compile_if(&mut self) {
        // parse "if"
        assert_eq!(self.infile.token.as_str(), "if");
        self.infile.advance();

        let mut endif_label = "endif_".to_owned();
        endif_label += &self.label_counter.to_string();
        let mut else_label = "else_".to_owned();
        else_label += &self.label_counter.to_string();
        self.label_counter += 1;

        // parse "("
        assert_eq!(self.infile.token.as_str(), "(");
        self.infile.advance(); 

        self.compile_expression();

        assert_eq!(self.infile.token.as_str(), ")");
        // TODO: check condition
        self.writer.write_if(&else_label);
        self.infile.advance();

        assert_eq!(self.infile.token.as_str(), "{");
        self.infile.advance();

        self.compile_statements();

        // parse "}"
        assert_eq!(self.infile.token.as_str(), "}");
        // TODO: check condition
        self.writer.write_if(&endif_label);
        self.infile.advance();

        // in case "else"
        if self.infile.token_type() == TokenType::KEYWORD {
            if self.infile.key_word() == "else" {
                // parse "else"
                self.infile.advance();
                
                // parse "{"
                assert_eq!(self.infile.token.as_str(), "{");
                self.writer.write_label(&else_label);
                self.infile.advance();

                self.compile_statements();

                // parse "}"
                assert_eq!(self.infile.token.as_str(), "}");
                self.infile.advance();                
            }
        }

        self.writer.write_label(&endif_label);       
    }

    fn compile_subroutine_call(&mut self) {
        assert!(self.infile.token_type() == TokenType::IDENTIFIER);
        let mut name_1 = self.infile.identifier().to_owned();
        self.infile.advance();

        // parse "(" or "."
        assert!(self.infile.token_type() == TokenType::SYMBOL);
        if self.infile.symbol() == '(' {
            self.infile.advance();
            self.compile_expression_list();
            // TODO; keep n_args 
            let n_args = 1;
            self.writer.write_call(&name_1, n_args);
        } else {
            assert_eq!(self.infile.symbol(), '.');
            self.infile.advance();
            name_1 += ".";
            name_1 += &self.infile.identifier();
            self.infile.advance();
            assert_eq!(self.infile.symbol(), '(');
            self.infile.advance();
            self.compile_expression_list();
            // TODO; keep n_args 
            let n_args = 1;
            self.writer.write_call(&name_1, n_args);
        }

        assert_eq!(self.infile.symbol(), ')');
        self.infile.advance();
    }

    fn compile_expression(&mut self) {
        self.compile_term();
        
        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    // only in case Op, call compile_term again
                    match self.infile.symbol() {
                        '+' => {
                            self.infile.advance();
                            self.compile_term();
                            self.writer.write_arithmetic(Command::ADD);
                        },
                        '-' => {
                            self.infile.advance();
                            self.compile_term();
                            self.writer.write_arithmetic(Command::SUB);
                        },
                        '*' => {
                            self.infile.advance();
                            self.compile_term();
                            self.writer.write_call("Math.multiply", 2);
                        },
                        '/' => {
                            self.infile.advance();
                            self.compile_term();
                            self.writer.write_call("Math.divide", 2);
                        },
                        '&' => {
                            self.infile.advance();
                            self.compile_term();
                            self.writer.write_arithmetic(Command::AND);
                        },
                        '|' => {
                            self.infile.advance();
                            self.compile_term();
                            self.writer.write_arithmetic(Command::OR);
                        },
                        '<' => {
                            self.infile.advance();
                            self.compile_term();
                            self.writer.write_arithmetic(Command::LT);
                        },
                        '>' => {
                            self.infile.advance();
                            self.compile_term();
                            self.writer.write_arithmetic(Command::GT);
                        },
                        '=' => {
                            self.infile.advance();
                            self.compile_term();
                            self.writer.write_arithmetic(Command::EQ);
                        },
                        _=> break,
                    }
                },
                _ => break,
            }
        }
    }
    
    fn compile_term(&mut self) {
        match self.infile.token_type() {
            TokenType::SYMBOL => {
                match self.infile.symbol() {
                    '-' => {
                        self.infile.advance();
                        self.compile_term();
                        self.writer.write_arithmetic(Command::NEG);
                    },
                    '~' => {
                        self.infile.advance();
                        self.compile_term();
                        self.writer.write_arithmetic(Command::NOT);
                    },
                    '(' => {
                        self.infile.advance();
                        self.compile_expression();
                        assert_eq!(self.infile.token.as_str(), ")");
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
                            // parse varName
                            self.infile.advance();
                            assert_eq!(self.infile.token.as_str(), "[");
                            self.infile.advance();
                            self.compile_expression();
                            assert_eq!(self.infile.token.as_str(), "]");  
                            self.infile.advance();                           
                        }
                        _ => {
                            self.infile.idx = current_idx;
                            self.infile.next_idx = current_next_idx;
                            self.infile.token = self.infile.file[self.infile.idx..self.infile.next_idx].to_owned();
                            self.infile.advance();
                        },
                    }
                } else {
                    self.infile.idx = current_idx;
                    self.infile.next_idx = current_next_idx;
                    self.infile.token = self.infile.file[self.infile.idx..self.infile.next_idx].to_owned();
                    self.infile.advance();
                }
            },
            TokenType::INTCONST => {
                self.writer.write_push(Segment::CONST, self.infile.int_val());
                self.infile.advance();
            },
            TokenType::STRINGCONST => unimplemented!(),
            _ => {
                self.infile.advance();
            },
        }
    }
    
    fn compile_expression_list(&mut self) {

        if self.infile.token_type() == TokenType::SYMBOL {
            if self.infile.symbol() == ')' {
                return;
            }
        } 

        self.compile_expression();

        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    match self.infile.symbol() {
                        ',' => {
                            self.infile.advance();
                            self.compile_expression();
                        },
                        _ => break,
                    }
                },
                _ => break,
            }
        }
    }

}