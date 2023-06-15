use std::{path::Path, fs::File, io::{self, BufRead}, str::Chars};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Bool(bool)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Seperator {
    OpenBlock,
    CloseBlock,
}
impl TryFrom<&str> for Seperator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "." => Ok(Seperator::CloseBlock),
            "..." => Ok(Seperator::OpenBlock),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Keyword {
    If,
    Else,
    EndIf,
    Struct,
    Print,
    Statement,
}
impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "endif" => Ok(Keyword::EndIf),
            "struct" => Ok(Keyword::Struct),
            "print" => Ok(Keyword::Print),
            "statement" => Ok(Keyword::Statement),
            _ =>    Err(())
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operator {
    Add,
    Minus,
    Multiply,
    Divide,
    Equals,
    EqualTo,
    NotEqualTo,
}
impl TryFrom<&str> for Operator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Minus),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "=" => Ok(Operator::Equals),
            "==" => Ok(Operator::EqualTo),
            "!=" => Ok(Operator::NotEqualTo),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Token {
    EndOfFile,
    Identifier(String),
    Operator(Operator),
    Keyword(Keyword),
    Seperator(Seperator),
    Literal(Value),
    Descriptor(String),
    Comment(String),
}

#[derive(Debug, Clone)]
struct Lexer {
    in_block: bool,
    in_quote: bool,
    current_identifier: String,
    tokens: Vec<Token>,
    input: Chars<'static>,
}
impl Lexer {
    pub fn new(input: &'static str) -> Self {
        println!("{input}");
        Self {
            in_block: false,
            in_quote: false,
            current_identifier: String::new(),
            tokens: Vec::new(),
            input: input.chars().to_owned()
        }
    }
    fn parse(&mut self) {
        while self.tokens.last() != Some(&Token::EndOfFile) {
            if let Some(next_token) = self.get_token() {self.tokens.push(next_token)}
        }
        println!("{:#?}", self.tokens)
    }

    fn get_token(&mut self) -> Option<Token> {
        let mut last_char: char = ' ';
        let mut identifier: String;
        while last_char.is_whitespace() {
            if let Some(next_char) = self.input.next() {
                last_char = next_char;
            } else {
                println!("EOF");
                return Some(Token::EndOfFile)
            }
        }
        if last_char == '"' {
            self.in_quote = !self.in_quote;
        }
        if last_char == '.' {
            return Some(Token::Seperator(Seperator::CloseBlock))
        }
        if last_char.is_alphabetic() {
            identifier = last_char.to_string();
            while let Some(next_char) = self.input.next() {
                if next_char.is_alphanumeric() {
                    identifier.push(next_char);
                } else {
                    println!("identifier: {}, next_char: {}", identifier, next_char);
                    if self.in_quote {
                        self.current_identifier += identifier.as_str();
                        if next_char == '"' {
                            self.in_quote = !self.in_quote;
                            let token = Token::Literal(Value::String(self.current_identifier.clone()));
                            self.current_identifier.clear();
                            return Some(token)

                        } else {
                            self.current_identifier += " ";
                        }
                    }
                    break;
                }
            }
            //if let Ok(res) = Keyword::try_from(identifier.as_str()) {
            //    return Token::Keyword(res)
            //} else {
            //    
            //}
        } 
        return None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Hello, world!");
    let test = "\"".chars().next().unwrap();
    let mut lexer = Lexer::new(include_str!("../test2.spell"));
    lexer.parse();
}
