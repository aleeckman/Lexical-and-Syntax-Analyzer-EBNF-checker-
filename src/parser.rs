use crate::scanner::Scanner;
use std::process::exit;
use crate::token::Token;
use crate::token::TokenType;
use std::net::Shutdown::Read;
use std::io::Write;
use crate::parser::ErrorHandler::{RETURNSTATEMENT, PARAMETER, PARAMETERBLOCK, STATEMENT, INTEGERTYPE, ASSIGNMENT, WHILELOOP, EXPRESSION, SIMPLEEXPRESSION, TERM, FACTOR};
use crate::parser::ReturnType::{ENDS, EXISTS};
use crate::token::TokenType::INTCONSTANT;
use std::fs::File;

#[derive(PartialEq)]
pub enum ReturnType {
    EXISTS,
    ENDS,
    ERROR
}

pub enum ErrorHandler {
    PROGRAM,
    DECLARATION,
    MAINDECLARATION,
    FUNCTIONDEFINITION,
    DECLARATIONTYPE,
    VARIABLEDECLARATION,
    FUNCTIONDECLARATION,
    BLOCK,
    PARAMETERBLOCK,
    DATATYPE,
    CONSTANT,
    STATEMENT,
    PARAMETER,
    INTEGERTYPE,
    FLOATTYPE,
    ASSIGNMENT,
    WHILELOOP,
    IFSTATEMENT,
    RETURNSTATEMENT,
    EXPRESSION,
    SIMPLEEXPRESSION,
    TERM,
    FACTOR,
    RELATIONOPERATOR,
    ADDOPERATOR,
    MULTOPERATOR
}

pub struct Parser {
    pub scan:Scanner,
    pub pos: usize,
    pub line_num_error: Option<i32>,
    pub char_num_error: Option<i32>
}

impl Parser {
    pub fn new(s:Scanner) -> Parser {
        Parser {
            scan: s,
            pos: 0,
            line_num_error: None,
            char_num_error: None
        }
    }

    pub fn output_xhtml(&mut self) {

        let mut path = File::create("result.xhtml").expect("Couldn't create file");
        write!(path, "<!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>");
        write!(path, "<html xmlns='http://www.w3.org/1999/xhtml' xml:lang='en'>");
        write!(path, "<head>");
        write!(path, "<title>X Formatted file</title>");
        write!(path, "</head>");
        write!(path, "<body bgcolor='navy' text='yellow' link='yellow' vlink='yellow'>");
        write!(path, "<font face='Courier New'>");

        let mut color_of_text= "yellow";
        let mut bold = false;
        let mut tabs = 0;
        let mut prev_line = 1;
        let mut curr_line = 1;

        for tok in self.scan.token_vec.clone() {
            curr_line = tok.get_line_number();

            match tok.get_type().as_str() {
                "Function" => color_of_text = "orange",
                "Variable" => color_of_text = "yellow",
                "FloatConstant" => {color_of_text = "aqua"; bold=true;},
                "IntConstant" => {color_of_text = "aqua"; bold=true;},
                "Operator" => {color_of_text = "white"; bold=true;},
                "Keyword" => {color_of_text = "orange"; bold=true;},
                _ => color_of_text = "yellow"
            }

            if tok.get_text().eq("{") {
                tabs += 4;
            } else if tok.get_text().eq("}") {
                tabs -= 4;
            }

            if curr_line != prev_line {
                writeln!(path, "<br />");
                let mut i = 0;
                while i < tabs {
                    write!(path, "&ensp;");
                    i += 1;
                }
            }

            if bold {
                if tok.get_text().eq(";") || tok.get_text().eq("(") || tok.get_text().eq(")"){
                    write!(path, "<font color = '{}'><b>{}</b></font>", color_of_text, tok.get_text());
                } else {
                    write!(path, "<font color = '{}'><b> {} </b></font>", color_of_text, tok.get_text());
                }

            } else {
                write!(path, "<font color = '{}'> {}</font>", color_of_text, tok.get_text());
            }

            prev_line = tok.get_line_number();
        }

        write!(path, "</font>
        </body>
        </html>");
    }

    // Program := {Declaration} MainDeclaration {FunctionDefinition}
    pub fn program(&mut self) -> ReturnType {

        while self.declaration() == EXISTS { }

        if self.main_declaration() == EXISTS {
            while self.function_definition() == EXISTS { }
            return EXISTS
        } else {
            if self.scan.get_next_token(self.pos).is_none() {
                self.error(None, ErrorHandler::PROGRAM);
            }

            let mut token = self.scan.get_next_token(self.pos).unwrap().clone();
            self.error(Option::from(token.clone()), ErrorHandler::PROGRAM);
        }

        return ENDS
    }

    // Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)
    pub fn declaration(&mut self) -> ReturnType {

        if self.declaration_type() == EXISTS {

            if self.variable_declaration() == EXISTS {
                return EXISTS
            }

            else if self.function_declaration() == EXISTS {
                return EXISTS
            }

            else {
                if self.scan.get_next_token(self.pos).is_none() {
                    self.error(None, ErrorHandler::DECLARATION);
                }

                let mut token = self.scan.get_next_token(self.pos).unwrap().clone();
                self.error(Option::from(token.clone()), ErrorHandler::DECLARATION);
            }

        }

        return ENDS

    }

    // MainDeclaration := void main ( ) Block
    pub fn main_declaration(&mut self) -> ReturnType {

        if self.scan.get_next_token(self.pos).is_none() {
            return ReturnType::ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq("void") {
            self.pos += 1;

            if self.scan.get_next_token(self.pos).is_none() {
                return ENDS;
            }

            token = self.scan.get_next_token(self.pos).unwrap().clone();

            if token.get_text().eq("main") {
                self.pos += 1;

                if self.scan.get_next_token(self.pos).is_none() {
                    return ENDS;
                }

                token = self.scan.get_next_token(self.pos).unwrap().clone();

                if token.get_text().eq("(") {
                    self.pos += 1;

                    if self.scan.get_next_token(self.pos).is_none() {
                        return ENDS;
                    }

                    token = self.scan.get_next_token(self.pos).unwrap().clone();

                    if token.get_text().eq(")") {
                        self.pos += 1;

                        if self.block() == EXISTS {
                            return EXISTS
                        }

                        if self.scan.get_next_token(self.pos).is_none() {
                            return ENDS;
                        }

                        token = self.scan.get_next_token(self.pos).unwrap().clone();
                        self.error(Option::from(token.clone()), ErrorHandler::MAINDECLARATION);
                    }
                    self.error(Option::from(token.clone()), ErrorHandler::MAINDECLARATION);
                }
                self.error(Option::from(token.clone()), ErrorHandler::MAINDECLARATION);
            }
            self.error(Option::from(token.clone()), ErrorHandler::MAINDECLARATION);
        }

        return ENDS;

    }

    // FunctionDefinition := DeclarationType ParameterBlock Block
    pub fn function_definition(&mut self) -> ReturnType {

        if self.declaration_type() == EXISTS {

            if self.parameter_block() == EXISTS {

                if self.block() == EXISTS {
                    return EXISTS
                }

                if self.scan.get_next_token(self.pos).is_none() {
                    self.error(None, ErrorHandler::FUNCTIONDECLARATION);
                }

                let mut token = self.scan.get_next_token(self.pos).unwrap().clone();
                self.error(Option::from(token), ErrorHandler::FUNCTIONDECLARATION);
            }

            if self.scan.get_next_token(self.pos).is_none() {
                self.error(None, ErrorHandler::FUNCTIONDECLARATION);
            }

            let mut token = self.scan.get_next_token(self.pos).unwrap().clone();
            self.error(Option::from(token), ErrorHandler::FUNCTIONDECLARATION);
        }

        return ENDS;

    }

    // DeclarationType := DataType Identifier
    pub fn declaration_type(&mut self) -> ReturnType {
        if self.data_type() == EXISTS {
            if self.scan.get_next_token(self.pos).is_none() {
                self.error(None, ErrorHandler::DECLARATIONTYPE);
            }

            let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

            if token.get_type().as_str().eq("Variable") || token.get_type().as_str().eq("Function") {
                self.pos += 1;
                return EXISTS;
            } else {
                self.error(Option::from(token.clone()), ErrorHandler::DECLARATIONTYPE);
            }
        }

        return ENDS;
    }

    // VariableDeclaration := [= Constant] ;
    pub fn variable_declaration(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq("=") {
            self.pos += 1;
            if self.constant() != EXISTS {
               self.error(Option::from(token.clone()), ErrorHandler::VARIABLEDECLARATION);
            }
        }

        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq(";") {
            self.pos += 1;
            return EXISTS;
        }

        return ENDS;
    }

    // FunctionDeclaration := ParameterBlock ;
    pub fn function_declaration(&mut self) -> ReturnType {

        if self.parameter_block() != EXISTS {
            return ENDS;
        }

        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();
        if token.get_text().eq(";") {
            self.pos += 1;
            return EXISTS;
        } else {
            self.error(Option::from(token.clone()), ErrorHandler::FUNCTIONDECLARATION);
            return ENDS;
        }

    }

    // Block := {^ {Declaration} {Statement} {FunctionDefinition} }^
    pub fn block(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();
        if token.get_text().ne("{") {
            return ENDS;
        }

        self.pos += 1;

        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        token = self.scan.get_next_token(self.pos).unwrap().clone();
        if self.declaration() == EXISTS {
            while self.declaration() == EXISTS { }
        }

        if self.statement() == EXISTS {
            while self.statement() == EXISTS { }
        }

        if self.function_definition() == EXISTS {
            while self.function_definition() == EXISTS { }
        }

        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        token = self.scan.get_next_token(self.pos).unwrap().clone();
        if !token.get_text().eq("}") {
            self.error(Option::from(token.clone()), ErrorHandler::BLOCK);
        }

        self.pos += 1;
        return EXISTS;
    }

    // ParameterBlock := (^ [Parameter {, Parameter}] )^
    pub fn parameter_block(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq("(") {
            self.pos += 1;

            if self.parameter() == EXISTS {

                token = self.scan.get_next_token(self.pos).unwrap().clone();
                while token.get_text().eq(",") {
                    self.pos += 1;

                    if self.parameter() != EXISTS {
                        self.error(Option::from(token.clone()), PARAMETERBLOCK);
                    }
                }

                token = self.scan.get_next_token(self.pos).unwrap().clone();
                if token.get_text().eq(")") {
                    self.pos += 1;
                    return EXISTS;
                } else {
                    self.error(Option::from(token.clone()), PARAMETERBLOCK);
                }

            }
        }

        return ENDS;
    }

    // DataType := IntegerType | FloatType
    pub fn data_type(&mut self) -> ReturnType {
        if self.integer_type() == EXISTS {
            return EXISTS;
        }

        else if self.float_type() == EXISTS {
            return EXISTS
        }

        return ENDS;
    }

    // Constant := IntConstant | FloatConstant
    pub fn constant(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();
        self.pos += 1;

        if token.get_type().as_str().eq("IntConstant") {
            return EXISTS;
        }

        else if token.get_type().as_str().eq("FloatConstant") {
            return EXISTS;
        }

        return ENDS;
    }

    // Statement := (Assignment | WhileLoop | IfStatement | ReturnStatement | Expression ;)
    pub fn statement(&mut self) -> ReturnType {
        if self.assignment() == EXISTS {
            return EXISTS;
        }

        else if self.while_loop() == EXISTS {
            return EXISTS;
        }

        else if self.if_statement() == EXISTS {
            return EXISTS;
        }

        else if self.return_statement() == EXISTS {
            return EXISTS;
        }

        else if self.expression() == EXISTS {
            if self.scan.get_next_token(self.pos).is_none() {
                self.error(None, STATEMENT);
            }

            let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

            if token.get_text().eq(";") {
                self.pos+=1;
                return EXISTS;
            } else {
                self.error(Option::from(token.clone()), STATEMENT);
            }
        }

        return ENDS;
    }

    // Parameter := DataType Identifier
    pub fn parameter(&mut self) -> ReturnType {
        if self.data_type() != EXISTS {
            return ENDS;
        }

        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_type().as_str().eq("Variable") {
            self.pos += 1;
            return EXISTS;
        } else {
            self.error(Option::from(token.clone()), PARAMETER);
        }

        return ENDS;
    }

    // IntegerType := [unsigned^] (char^ | short^ | int^ | long^)
    pub fn integer_type(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }
        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();
        let mut unsigned_given = false;

        if token.get_text().eq("unsigned") {
            unsigned_given = true;
            self.pos += 1;
        }

        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq("char") {
            self.pos += 1;
            return EXISTS;
        }

        else if token.get_text().eq("short") {
            self.pos += 1;
            return EXISTS;
        }

        else if token.get_text().eq("int") {
            self.pos += 1;
            return EXISTS;
        }

        else if token.get_text().eq("long") {
            self.pos += 1;
            return EXISTS;
        }

        else {
            if unsigned_given {
                self.error(Option::from(token.clone()), INTEGERTYPE);
            }

            return ENDS
        }
    }

    // FloatType := float^ | double^
    pub fn float_type(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq("float") {
            self.pos += 1;
            return EXISTS;
        }

        else if token.get_text().eq("double") {
            self.pos += 1;
            return EXISTS;
        }

        return ENDS;
    }

    // Assignment := Identifier = {Identifier =} Expression ;
    pub fn assignment(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_type().as_str().eq("Variable") {

            self.pos += 1;
            token = self.scan.get_next_token(self.pos).unwrap().clone();



            if token.get_text().eq("=") {
                self.pos += 1;
                token = self.scan.get_next_token(self.pos).unwrap().clone();
                let mut next_token = self.scan.get_next_token(self.pos+1).unwrap().clone();
                let mut in_while = false;

                while token.get_type().as_str().eq("Variable") && next_token.get_text().eq("=") {
                    in_while = true;
                    self.pos += 1;
                    token = self.scan.get_next_token(self.pos).unwrap().clone();
                    next_token = self.scan.get_next_token(self.pos + 1).unwrap().clone();
                }

                if in_while {
                    self.pos+=1;
                }

                if self.expression() == EXISTS {
                    return EXISTS;
                } else {
                    self.error(Option::from(token.clone()), ASSIGNMENT);
                }
            } else {
                self.error(Option::from(token.clone()), ASSIGNMENT);
            }
        }

        return ENDS;

    }

    // WhileLoop := while^ (^ Expression )^ Block
    pub fn while_loop(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq("while") {
            self.pos += 1;
            token = self.scan.get_next_token(self.pos).unwrap().clone();

            if token.get_text().eq("(") {
                self.pos += 1;

                if self.expression() != EXISTS {
                    self.error(Option::from(token.clone()), WHILELOOP);
                }

                token = self.scan.get_next_token(self.pos).unwrap().clone();

                if token.get_text().eq(")") {
                    self.pos += 1;

                    if self.block() == EXISTS {
                        return EXISTS;
                    } else {
                        self.error(Option::from(token.clone()), WHILELOOP)
                    }

                } else {
                    self.error(Option::from(token.clone()), WHILELOOP)
                }

            } else {
                self.error(Option::from(token.clone()), WHILELOOP)
            }
        }

        return ENDS
    }

    // IfStatement := if^ (^ Expression )^ Block
    pub fn if_statement(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq("if") {
            self.pos += 1;
            token = self.scan.get_next_token(self.pos).unwrap().clone();

            if token.get_text().eq("(") {
                self.pos += 1;

                if self.expression() == ReturnType::EXISTS {
                    token = self.scan.get_next_token(self.pos).unwrap().clone();

                    if token.get_text().eq(")") {
                        self.pos += 1;

                        if self.block() == ReturnType::EXISTS {
                            return ReturnType::EXISTS
                        } else {
                            self.error(Option::from(token.clone()), ErrorHandler::IFSTATEMENT);
                        }
                    } else {
                        self.error(Option::from(token.clone()), ErrorHandler::IFSTATEMENT);
                    }
                } else {
                    self.error(Option::from(token.clone()), ErrorHandler::IFSTATEMENT);
                }
            } else {
                self.error(Option::from(token.clone()), ErrorHandler::IFSTATEMENT);
            }
        }

        return ENDS;

    }

    // ReturnStatement := return^ Expression ;
    pub fn return_statement(&mut self) -> ReturnType{

        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq("return") {
            self.pos += 1;
            if self.expression() == ReturnType::EXISTS {
                self.pos += 1;
                return ReturnType::EXISTS
            } else {
                self.error(Option::from(token.clone()), ErrorHandler::RETURNSTATEMENT);
            }

        }

        return ReturnType::ENDS;

    }

    // Expression := SimpleExpression [ RelationOperator SimpleExpression ]
    pub fn expression(&mut self) -> ReturnType {

        if self.scan.get_next_token(self.pos).is_none() {

            return ReturnType::ENDS;
        }

        if self.simple_expression() == ReturnType::EXISTS {

            if self.relation_operator() == ReturnType::EXISTS {

                if self.simple_expression() == ReturnType::EXISTS {
                    return ReturnType::EXISTS;
                } else {
                    let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

                    self.error(Option::from(token.clone()), EXPRESSION);
                }
            } else {
                return ReturnType::EXISTS;
            }

        }

        return ReturnType::ENDS;
    }

    // SimpleExpression := Term { AddOperator Term }
    pub fn simple_expression(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ReturnType::ENDS;
        }

        //let mut token = self.scan.get_next_token(self.pos).unwrap().clone().unwrap();

        if self.term() == ReturnType::EXISTS {

            while self.add_operator() == ReturnType::EXISTS {

                if self.term() != ReturnType::EXISTS {
                    let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

                    self.error(Option::from(token.clone()), SIMPLEEXPRESSION);
                }
            }

            return ReturnType::EXISTS;

        }

        return ReturnType::ENDS;

    }

    // Term := Factor { MultOperator Factor }
    pub fn term(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        //let mut token = self.scan.get_next_token(self.pos).unwrap().clone().unwrap();

        if self.factor() == ReturnType::EXISTS {

            while self.mult_operator() == ReturnType::EXISTS {

                if self.factor() != ReturnType::EXISTS {
                    let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

                    self.error(Option::from(token.clone()), TERM);
                }
            }

            return ReturnType::EXISTS;

        }

        return ReturnType::ENDS;
    }

    // Factor := ( (^ Expression )^ ) | Constant | (Identifier [ (^ [ Expression {, Expression } ] )^ ] )
    pub fn factor(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;
        }

        let mut token = self.scan.get_next_token(self.pos).unwrap().clone();

        if token.get_text().eq("(") {
            self.pos += 1;
            if self.expression() == EXISTS {
                token = self.scan.get_next_token(self.pos).unwrap().clone();

                if token.get_text().eq(")") {
                    self.pos += 1;
                    return EXISTS;
                } else {
                    self.error(Option::from(token.clone()), ErrorHandler::FACTOR);
                }
            } else {
                self.error(Option::from(token.clone()), ErrorHandler::FACTOR);
            }
        }

        else if self.constant() == EXISTS {
            return EXISTS;
        }

        else if token.get_type().as_str().eq("Function") || token.get_type().as_str().eq("Variable") {
            let mut function = false;
            if token.get_type().as_str().eq("Function") {
                function = true;
            }

            //self.pos += 1;
            token = self.scan.get_next_token(self.pos).unwrap().clone();

            if token.get_text().eq("(") {
                self.pos += 1;
                if self.expression() == EXISTS {
                    token = self.scan.get_next_token(self.pos).unwrap().clone();
                    while token.get_text().eq(",") {
                        self.pos += 1;

                        if self.expression() != EXISTS {
                            self.error(Option::from(token.clone()), FACTOR);
                        }

                        token = self.scan.get_next_token(self.pos).unwrap().clone();
                    }
                }

                if token.get_text().eq(")") {
                    self.pos += 1;
                    return ReturnType::EXISTS;
                } else {
                    self.error(Option::from(token.clone()), ErrorHandler::FACTOR);
                }
            } else {
                if function == false {
                    return EXISTS;
                } else {
                    self.error(Option::from(token.clone()), ErrorHandler::FACTOR);
                }
            }
        }

        return ReturnType::ENDS;
    }

    // RelationOperator := ( == ) | < | > | ( <= ) | ( >= ) | ( != )
    pub fn relation_operator(&mut self) -> ReturnType {
        let op = ["==", ">", "<", "<=", ">=", "!="];

        if self.scan.get_next_token(self.pos).is_none() {
            return ENDS;

        }

        if op.contains(&self.scan.get_next_token(self.pos).unwrap().get_text()) {
            self.pos += 1;
            return EXISTS;
        }

        return ENDS;
    }

    // AddOperator := + | -
    pub fn add_operator(&mut self) -> ReturnType {
        if self.scan.get_next_token(self.pos).is_none() {
            return ReturnType::ENDS;
        }

        if self.scan.get_next_token(self.pos).unwrap().get_text().eq("+") ||
            self.scan.get_next_token(self.pos).unwrap().get_text().eq("-") {
            self.pos += 1;
            return ReturnType::EXISTS;
        }

        return ReturnType::ENDS;
    }

    // MultOperator := * | /
    pub fn mult_operator(&mut self) -> ReturnType {

        if self.scan.get_next_token(self.pos).is_none() {
            return ReturnType::ENDS;
        }

        if self.scan.get_next_token(self.pos).unwrap().get_text().eq("*") ||
            self.scan.get_next_token(self.pos).unwrap().get_text().eq("/") {
            self.pos += 1;
            return ReturnType::EXISTS;
        }

        return ReturnType::ENDS;
    }

    pub fn error(&mut self, t: Option<Token>, e: ErrorHandler) {
        println!("\nEBNF Rule Broken:");

        match e {
            ErrorHandler::PROGRAM => println!("Program := {{ Declaration }} MainDeclaration {{ FunctionDefinition }}"),
            ErrorHandler::DECLARATION => println!("Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)"),
            ErrorHandler::MAINDECLARATION => println!("MainDeclaration := void main ( ) Block"),
            ErrorHandler::FUNCTIONDEFINITION => println!("FunctionDefinition := DeclarationType ParameterBlock Block"),
            ErrorHandler::DECLARATIONTYPE => println!("DeclarationType := DataType Identifier"),
            ErrorHandler::VARIABLEDECLARATION => println!("VariableDeclaration := [= Constant] ;"),
            ErrorHandler::FUNCTIONDECLARATION => println!("FunctionDeclaration := ParameterBlock ;"),
            ErrorHandler::BLOCK => println!("Block := {{ {{Declaration}} {{Statement}} {{FunctionDefinition}} }}"),
            ErrorHandler::PARAMETERBLOCK => println!("ParameterBlock := ( [Parameter {{, Parameter}}] )"),
            ErrorHandler::DATATYPE => println!("DataType := IntegerType | FloatType"),
            ErrorHandler::CONSTANT => println!("Constant := IntConstant | FloatConstant"),
            ErrorHandler::STATEMENT => println!("Statement := (Assignment | WhileLoop | IfStatement | ReturnStatement | Expression ;)"),
            ErrorHandler::PARAMETER => println!("Parameter := DataType Identifier"),
            ErrorHandler::INTEGERTYPE => println!("IntegerType := [unsigned^] (char^ | short^ | int^ | long^)"),
            ErrorHandler::FLOATTYPE => println!("FloatType := float^ | double^"),
            ErrorHandler::ASSIGNMENT => println!("Assignment := Identifier = {{Identifier =}} Expression ;"),
            ErrorHandler::WHILELOOP => println!("WhileLoop := while^ (^ Expression )^ Block"),
            ErrorHandler::IFSTATEMENT => println!("IfStatement := if^ (^ Expression )^ Block"),
            ErrorHandler::RETURNSTATEMENT => println!("ReturnStatement := return^ Expression ;"),
            ErrorHandler::EXPRESSION => println!("Expression := SimpleExpression [ RelationOperator SimpleExpression ]"),
            ErrorHandler::SIMPLEEXPRESSION => println!("SimpleExpression := Term {{ AddOperator Term }}"),
            ErrorHandler::TERM => println!("Term := Factor {{ MultOperator Factor }}"),
            ErrorHandler::FACTOR => println!("Factor := ( (^ Expression )^ ) | Constant | (Identifier [ (^ [ Expression {{, Expression }} ] )^ ] )"),
            ErrorHandler::RELATIONOPERATOR => println!("RelationOperator := ( == ) | < | > | ( <= ) | ( >= ) | ( != )"),
            ErrorHandler::ADDOPERATOR => println!("AddOperator := + | -"),
            ErrorHandler::MULTOPERATOR => println!("MultOperator := * | /"),
            _ => println!("Unknown Error Detected")

        }

        match t {
            Some(t) => {
                //let mut prev_t = self.scan.get_next_token(self.pos-1).unwrap().clone();
                //let mut next_t = self.scan.get_next_token(self.pos+1).unwrap().clone();
                //println!("Previous Token: {}, Line Number: {}, Char Number: {}", prev_t.get_text(), prev_t.get_line_number(), prev_t.get_char_pos());
                println!("Current Token: {}, Line Number: {}, Char Number: {}", t.get_text(), t.get_line_number(), t.get_char_pos());
                //println!("Next Token: {}, Line Number: {}, Char Number: {}", next_t.get_text(), next_t.get_line_number(), next_t.get_char_pos());
            },
            _ => println!("No Token Found Error")
        }

        self.output_xhtml();
        exit(1);
    }
}