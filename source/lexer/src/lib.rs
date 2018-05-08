#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod lexer {
	use std::fs::File;
	use std::io::prelude::*;

	pub enum Token {
		OpenBrace, // {
		CloseBrace, // }
		OpenParenthesis, // (
		CloseParenthesis, // )
		Semicolon, // ;
		Int, // int
		Return, // return
		Identifier, // string starting with letter
		IntLiteral, // integer literal
	}

	pub fn lex(filename : &str) -> String {
		let mut file = File::open(filename).expect("File not found!");
		let mut contents = String::new();
		file.read_to_string(&mut contents)
			.expect("fucked up the file read my guide");
		return contents;
	}
}

