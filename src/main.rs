use std::{str::Chars, iter::Peekable};

mod test;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location(pub usize, pub usize);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Token {
    Word(String),
    String(String),
    EndOfFile,
}

struct Lexer {
    input: Peekable<Chars<'static>>,
    tokens: Vec<Token>,
    token_locations: Vec<Location>,
    cur: usize,
    bol: usize,
    row: usize,
    last_row: usize,
    current_string: String,
    in_string: bool,
}
impl Lexer {
    pub fn new(input: &'static str) -> Self {
        println!("{}", input);
        Self {
            input: input.chars().to_owned().peekable(),
            tokens: Vec::new(),
            token_locations: Vec::new(),
            cur: 0,
            bol: 0,
            row: 0,
            last_row: 0,
            current_string: String::new(),
            in_string: false,
        }
    }
    fn parse(&mut self) {
        while self.tokens.last() != Some(&Token::EndOfFile) {
            if let Some(next_token) = self.get_token() {
                self.tokens.push(next_token)
            }
        }
        println!("{:#?}", self.tokens)
    }
    fn get_token(&mut self) -> Option<Token> {
        let mut last_char: char = ' ';
        let mut identifier: String;

        // get start of next thing that is not whitespace
        while last_char.is_whitespace() {
            self.cur += 1;
            if last_char == '\n' {
                self.row += 1;
                self.bol = self.cur;
            }
            if let Some(next_char) = self.input.next() {
                last_char = next_char;
            } else {
                println!("EOF");
                return Some(Token::EndOfFile)
            }
        }
        identifier = last_char.to_string();
        if last_char.is_ascii_punctuation() {
            if last_char == '"' {
                self.in_string = true;
                identifier = "".to_string();
            }
        }
        
        while let Some(next_char) = self.input.peek() {
            if next_char.is_alphanumeric() {
                identifier.push(*next_char);
                self.input.next();
                
            } else {
                if self.in_string {
                    if self.current_string.len() > 0 {
                        identifier.insert_str(0, " ");
                    }
                    self.current_string += &identifier;
                    if next_char == &'"' {
                        self.input.next();
                        self.in_string = false;
                        let token = Token::String(self.current_string.clone());
                        self.current_string.clear();
                        return Some(token)
                    }
                } else {
                    return Some(Token::Word(identifier))
                }
                break
            }
        }
        return None
    }
}

fn main() {
    let mut lexer = Lexer::new(include_str!("../test2.spell"));
    lexer.parse()
}