use std::fs::File;
use std::io;
use std::convert::TryFrom;
use std::io::prelude::*;


pub struct CharStream {
	char_str_input: Vec<char>
}

impl CharStream {
	pub fn new(f: &str) -> CharStream {
		CharStream{
			char_str_input: f.chars().collect()
		}
	}

	// Returns true if more characters are available, false otherwise.
	pub fn more_available(&self) -> bool {
		return !(self.char_str_input.is_empty())
	}

	// Returns the next character without consuming it.
	// Returns None if no more characters are available.
	pub fn peek_next_char(&self) -> Option<char> {
		if !(self.char_str_input.is_empty()) {
			return Some(self.char_str_input[0])
		} else {
			return None
		}
	}

	// Returns the kth character ahead in the stream without consuming it.
	// peek_ahead_char(0) returns the same character as peek_next_char().
	// Returns None if no more characters are available at the position.
	// The input k cannot be negative.
	pub fn peek_ahead_char(&self, k: i32) -> Option<char> {

		if k > self.char_str_input.len() as i32 {
			return None
		}

		if self.char_str_input.is_empty() {
			return None
		}

		return Some(self.char_str_input[(k as usize)])
	}

	// Returns the next character and consumes it.
	// Returns None if no more characters are available.
	pub fn get_next_char(&mut self) -> Option<char> {

		if !(self.char_str_input.is_empty()) {
			let ret: Option<char> = Some(self.char_str_input[0]);
			self.char_str_input.remove(0);
			return ret;
		}

		return None
	}
}



