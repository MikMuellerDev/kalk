use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),

    Plus,
    Minus,
    Mul,
    Div,

    LParen,
    RParen,

    EOF,
}

const SINGLE_CHARS: [char; 6] = ['+', '-', '*', '/', '(', ')'];
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const IGNORE: [char; 3] = [' ', '\t', '\r'];

pub struct Lexer<'lexer> {
    current_char: Option<char>,
    current_char_index: u64,

    program: Chars<'lexer>,
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(program: &'lexer str) -> Self {
        Self {
            program: program.chars(),
            current_char: None,
            current_char_index: 0,
        }
    }

    /// Moves one position to the right, advances to the next character in the program
    fn advance(&mut self) {
        self.current_char_index += 1;
        self.current_char = self.program.next()
    }

    /// Provides one character lookahead
    fn lookahead(&self) -> Option<char> {
        self.program.clone().next()
    }

    fn make_single_char(&mut self, input: char) -> Token {
        self.advance();
        match input {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mul,
            '/' => Token::Div,
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => unreachable!("Called make_single_char with unimplemented terminal symbol"),
        }
    }

    fn make_number(&mut self) -> Token {
        // Create a new number string which initially contains the current char which can be
        // assumed to be a number
        let mut number = self.current_char.unwrap().to_string();
        self.advance();

        while self.current_char != None && DIGITS.contains(&self.current_char.unwrap()) {
            number.push(self.current_char.unwrap());
            self.advance();
        }

        // Do a non-mutative lookahead to determine whether to proceed number making after a dot
        if let Some(next) = self.lookahead() {
            if self.current_char == Some('.') && DIGITS.contains(&next) {
                number.push('.');
                self.advance();

                number.push(next);
                self.advance();

                while self.current_char != None && DIGITS.contains(&self.current_char.unwrap()) {
                    number.push(self.current_char.unwrap());
                    self.advance();
                }
            }
        }
        Token::Number(number)
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        self.advance();

        while let Some(current_char) = self.current_char {
            if IGNORE.contains(&current_char) {
                self.advance();
            } else if SINGLE_CHARS.contains(&current_char) {
                tokens.push(self.make_single_char(current_char));
            } else if DIGITS.contains(&current_char) {
                tokens.push(self.make_number())
            } else {
                panic!(
                    "Syntax error: invalid character: `{current_char}` at position {}",
                    self.current_char_index
                )
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}
