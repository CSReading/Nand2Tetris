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
    arg_counter: usize,
    var_counter: usize,
    field_counter: usize,
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
            arg_counter: 0,
            var_counter: 0,
            field_counter: 0,
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

        self.field_counter = 0;
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
        let mut first_loop = true;
        let mut type_ = String::new();
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
                        "field" => {
                            self.field_counter += 1;
                            Kind::FIELD
                        },
                        _ => unreachable!(),
                    };
                    if first_loop {
                        type_ = self.infile.token.to_owned();
                        self.infile.advance();
                        first_loop = false;
                    }

                    assert!(self.infile.token_type() == TokenType::IDENTIFIER);
                    let var_name = self.infile.identifier();
                    self.symbol_table.define(var_name, &type_, kind);
                    self.infile.advance();
                }
            }
        }
    }
    
    fn compile_subroutine(&mut self) {
        self.symbol_table.start_subroutine();
        let mut is_method = false;
        let mut is_constructor = false;

        // "constructor", "function", or "method" 
        assert!(self.infile.token_type() == TokenType::KEYWORD);
        if self.infile.key_word() == "method" {
            is_method = true;
            self.symbol_table.define("this", &self.class_name, Kind::ARG);
        }
        if self.infile.key_word() == "constructor" {
            is_constructor = true;
        }
        self.infile.advance();

        // "void" or type
        self.infile.advance();

        assert!(self.infile.token_type() == TokenType::IDENTIFIER);
        let mut subroutine_name = self.class_name.to_owned();
        subroutine_name += ".";
        subroutine_name += self.infile.identifier();
        self.infile.advance();

        assert_eq!(self.infile.symbol(), '(');
        self.infile.advance();
        self.compile_parameter_list();
        assert_eq!(self.infile.symbol(), ')');
        self.infile.advance();
    
        // subroutineBody    
        assert_eq!(self.infile.symbol(), '{');
        self.infile.advance();        
        
        self.var_counter = 0;
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
        self.writer.write_function(&subroutine_name, self.var_counter);
        // Memory allocate if object
        if is_constructor {
            self.writer.write_push(Segment::CONST, self.field_counter);
            self.writer.write_call("Memory.alloc", 1);
            self.writer.write_pop(Segment::POINTER, 0);
        }
        
        // set Argument 0 as Pointer 0 in case method
        if is_method {
            self.writer.write_push(Segment::ARG, 0);
            self.writer.write_pop(Segment::POINTER, 0);
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
                    } else if self.infile.symbol() == ',' {
                        self.infile.advance();
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
                    self.var_counter += 1;
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
        let mut is_array = false;

        assert_eq!(self.infile.token.as_str(), "let");
        self.infile.advance();

        // check varName
        assert!(self.infile.token_type() == TokenType::IDENTIFIER);
        let var_name = self.infile.identifier().to_owned(); 
        self.infile.advance();

        assert!(self.infile.token_type() == TokenType::SYMBOL);
        if self.infile.symbol() == '[' {
            is_array = true;
            self.infile.advance();

            let array_base = *self.symbol_table.index_of(&var_name).unwrap();
            match *self.symbol_table.kind_of(&var_name).unwrap() {
                Kind::STATIC => self.writer.write_push(Segment::STATIC, array_base),
                Kind::FIELD => self.writer.write_push(Segment::THIS, array_base),
                Kind::ARG => self.writer.write_push(Segment::ARG, array_base),
                Kind::VAR => self.writer.write_push(Segment::LOCAL, array_base),
            }

            self.compile_expression();
            assert_eq!(self.infile.token.as_str(), "]");  
            self.infile.advance();     
            // to find pointer of a[k]
            self.writer.write_arithmetic(Command::ADD);
            // use that segment and set address at "that 0"
        } 

        assert_eq!(self.infile.symbol(), '=');
        self.infile.advance();
        self.compile_expression();

        // assignment
        if is_array {
            // assign in the address
            self.writer.write_pop(Segment::TEMP, 0);
            self.writer.write_pop(Segment::POINTER, 1);
            self.writer.write_push(Segment::TEMP, 0);
            self.writer.write_pop(Segment::THAT, 0);
        } else {
            let kind = self.symbol_table.kind_of(&var_name).unwrap();
            let index = self.symbol_table.index_of(&var_name).unwrap();
            match *kind {
                Kind::STATIC => self.writer.write_pop(Segment::STATIC, *index),
                Kind::FIELD => self.writer.write_pop(Segment::THIS, *index),
                Kind::ARG => self.writer.write_pop(Segment::ARG, *index),
                Kind::VAR => self.writer.write_pop(Segment::LOCAL, *index),
            }
        }
        
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
        self.writer.write_arithmetic(Command::NOT);
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
        self.writer.write_arithmetic(Command::NOT);
        self.writer.write_if(&else_label);
        self.infile.advance();

        assert_eq!(self.infile.token.as_str(), "{");
        self.infile.advance();

        self.compile_statements();

        // parse "}"
        assert_eq!(self.infile.token.as_str(), "}");
        self.writer.write_goto(&endif_label);
        self.infile.advance();

        self.writer.write_label(&else_label);
        // in case "else"
        if self.infile.token_type() == TokenType::KEYWORD {
            if self.infile.key_word() == "else" {
                // parse "else"
                self.infile.advance();
                
                // parse "{"
                assert_eq!(self.infile.token.as_str(), "{");
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
            // if no varname, then call from same class
            let mut name_2 = self.class_name.to_owned();
            name_2 += ".";
            name_2 += &name_1;
            self.writer.write_push(Segment::POINTER, 0);

            self.infile.advance();
            self.arg_counter = 1;
            self.compile_expression_list();
            self.writer.write_call(&name_2, self.arg_counter);
        } else {
            assert_eq!(self.infile.symbol(), '.');
            // check if name_1 is class name or object name
            let mut name_2 = match self.symbol_table.type_of(&name_1) {
                None => {
                    self.arg_counter = 0;
                    name_1.to_owned()
                }
                Some(t) => {
                    self.arg_counter = 1;
                    t.to_owned()
                }
            };
            self.infile.advance();
            name_2 += ".";
            name_2 += &self.infile.identifier();
            self.infile.advance();
            assert_eq!(self.infile.symbol(), '(');
            self.infile.advance();
            // push object if name_1 on calling method
            if self.arg_counter == 1 {
                match *self.symbol_table.kind_of(&name_1).unwrap() {
                    Kind::STATIC => self.writer.write_push(Segment::STATIC, *self.symbol_table.index_of(&name_1).unwrap()),
                    Kind::FIELD => self.writer.write_push(Segment::THIS, *self.symbol_table.index_of(&name_1).unwrap()),
                    Kind::ARG => self.writer.write_push(Segment::ARG, *self.symbol_table.index_of(&name_1).unwrap()),
                    Kind::VAR => self.writer.write_push(Segment::LOCAL, *self.symbol_table.index_of(&name_1).unwrap()),
                }
                
            }
            self.compile_expression_list();
            self.writer.write_call(&name_2, self.arg_counter);
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
                            // in case function or method
                            self.infile.idx = current_idx;
                            self.infile.next_idx = current_next_idx;
                            self.infile.token = self.infile.file[self.infile.idx..self.infile.next_idx].to_owned();
                            self.compile_subroutine_call();
                        },
                        '[' => {
                            // in case array 
                            self.infile.idx = current_idx;
                            self.infile.next_idx = current_next_idx;
                            self.infile.token = self.infile.file[self.infile.idx..self.infile.next_idx].to_owned();
                            // parse varName
                            let array_base = *self.symbol_table.index_of(&self.infile.identifier()).unwrap();
                            match *self.symbol_table.kind_of(&self.infile.identifier()).unwrap() {
                                Kind::STATIC => self.writer.write_push(Segment::STATIC, array_base),
                                Kind::FIELD => self.writer.write_push(Segment::THIS, array_base),
                                Kind::ARG => self.writer.write_push(Segment::ARG, array_base),
                                Kind::VAR => self.writer.write_push(Segment::LOCAL, array_base),
                            }
                            self.infile.advance();
                            assert_eq!(self.infile.token.as_str(), "[");
                            self.infile.advance();
                            self.compile_expression();
                            assert_eq!(self.infile.token.as_str(), "]");  
                            self.infile.advance();     
                            // to push a[k]
                            self.writer.write_arithmetic(Command::ADD);
                            // use that segment push the number of memory address
                            self.writer.write_pop(Segment::POINTER, 1);
                            self.writer.write_push(Segment::THAT, 0);
                        }
                        _ => {
                            // in case single varName
                            self.infile.idx = current_idx;
                            self.infile.next_idx = current_next_idx;
                            self.infile.token = self.infile.file[self.infile.idx..self.infile.next_idx].to_owned();
                            let kind = self.symbol_table.kind_of(&self.infile.token).unwrap();
                            let index = self.symbol_table.index_of(&self.infile.token).unwrap();
                            match *kind {
                                Kind::STATIC => self.writer.write_push(Segment::STATIC, *index),
                                Kind::FIELD => self.writer.write_push(Segment::THIS, *index),
                                Kind::ARG => self.writer.write_push(Segment::ARG, *index),
                                Kind::VAR => self.writer.write_push(Segment::LOCAL, *index),
                            }                            
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
            TokenType::STRINGCONST => {
                self.writer.write_push(Segment::CONST, self.infile.string_val().len()); 
                self.writer.write_call("String.new", 1);
                for i in self.infile.string_val().as_bytes() {
                    self.writer.write_push(Segment::CONST, *i as usize);
                    self.writer.write_call("String.appendChar", 2);
                }
                self.infile.advance();
            },
            TokenType::KEYWORD => {
                match self.infile.key_word() {
                    "true" => {
                        self.writer.write_push(Segment::CONST, 1);
                        self.writer.write_arithmetic(Command::NEG);
                    },
                    "false" | "null" => {
                        self.writer.write_push(Segment::CONST, 0);
                    },
                    "this" => {
                        self.writer.write_push(Segment::POINTER, 0);
                    },
                    _ => unreachable!(),
                }
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
        self.arg_counter += 1;
        self.compile_expression();

        loop {
            match self.infile.token_type() {
                TokenType::SYMBOL => {
                    match self.infile.symbol() {
                        ',' => {
                            self.infile.advance();
                            self.arg_counter += 1;
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