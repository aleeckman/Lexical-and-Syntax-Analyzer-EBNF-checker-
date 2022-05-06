//mod character_stream;
//use character_stream::*;

mod token;
mod character_stream;
mod scanner;
mod parser;

use token::*;
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufRead};
use crate::character_stream::CharStream;
use std::ptr::null;
use crate::scanner::Scanner;
use crate::parser::Parser;
use std::path::Path;
use std::env;

fn main() {
	//let args: Vec<String> = env::args().collect();
	//let filename = &args[1];
	let file = "./src/example1.x";

	let default_file: File = File::open("./src/example1.x").unwrap();
	let file_reader: BufReader<File> = BufReader::new(default_file);

	let mut x_file: Vec<String> = Vec::new();
	for line in file_reader.lines() { x_file.push(line.unwrap()); }

	let mut s = Scanner::new(x_file.clone());
	s.tokenize(x_file.clone());

	let mut p = Parser::new(s.clone());
	p.program();
	p.output_xhtml(); // If no errors found
}
