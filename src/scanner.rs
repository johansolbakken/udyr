use std::collections::HashMap;

use crate::error::error;
use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<String>,
    keywords: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: usize,
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

fn is_alpha_numeric(c: char) -> bool {
    return is_digit(c) || is_alpha(c);
}

impl Scanner {
    pub fn new(source: &String) -> Scanner {
        let mut scanner = Scanner {
            source: source.clone(),
            tokens: Vec::new(),
            errors: Vec::new(),
            keywords: HashMap::new(),
            start: 0,
            current: 0,
            line: 1,
        };

        scanner.keywords.insert(String::from("and"), TokenType::AND);
        scanner
            .keywords
            .insert(String::from("class"), TokenType::CLASS);
        scanner
            .keywords
            .insert(String::from("else"), TokenType::ELSE);
        scanner
            .keywords
            .insert(String::from("false"), TokenType::FALSE);
        scanner.keywords.insert(String::from("for"), TokenType::FOR);
        scanner.keywords.insert(String::from("fun"), TokenType::FUN);
        scanner.keywords.insert(String::from("if"), TokenType::IF);
        scanner.keywords.insert(String::from("nil"), TokenType::NIL);
        scanner.keywords.insert(String::from("or"), TokenType::OR);
        scanner
            .keywords
            .insert(String::from("print"), TokenType::PRINT);
        scanner
            .keywords
            .insert(String::from("return"), TokenType::RETURN);
        scanner
            .keywords
            .insert(String::from("super"), TokenType::SUPER);
        scanner
            .keywords
            .insert(String::from("this"), TokenType::THIS);
        scanner
            .keywords
            .insert(String::from("true"), TokenType::TRUE);
        scanner.keywords.insert(String::from("var"), TokenType::VAR);
        scanner
            .keywords
            .insert(String::from("while"), TokenType::WHILE);

        return scanner;
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ()> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // Single lexemes
            '(' => self.add_empty_token(TokenType::LeftParen),
            ')' => self.add_empty_token(TokenType::RightParen),
            '{' => self.add_empty_token(TokenType::LeftBrace),
            '}' => self.add_empty_token(TokenType::RightBrace),
            ',' => self.add_empty_token(TokenType::Comma),
            '.' => self.add_empty_token(TokenType::Dot),
            '-' => self.add_empty_token(TokenType::Minus),
            '+' => self.add_empty_token(TokenType::Plus),
            ';' => self.add_empty_token(TokenType::SEMICOLON),
            '*' => self.add_empty_token(TokenType::STAR),

            // operators
            '!' => {
                if self.match_next('=') {
                    self.add_empty_token(TokenType::BangEqual);
                    self.current += 1;
                } else {
                    self.add_empty_token(TokenType::BANG);
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_empty_token(TokenType::EqualEqual);
                    self.current += 1;
                } else {
                    self.add_empty_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_empty_token(TokenType::LessEqual);
                    self.current += 1;
                } else {
                    self.add_empty_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_empty_token(TokenType::GreaterEqual);
                    self.current += 1;
                } else {
                    self.add_empty_token(TokenType::GREATER);
                }
            }

            // Comments
            '/' => {
                if self.match_next('/') {
                    self.current += 1;

                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_empty_token(TokenType::SLASH)
                }
            }

            // Whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            // Strings
            '"' => self.string(),

            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    self.errors.push(error(self.line, "Unexpected character."));
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return c;
    }

    fn add_empty_token(&mut self, Type: TokenType) {
        self.add_token(Type, &String::new());
    }

    fn add_token(&mut self, Type: TokenType, literal: &String) {
        let text = String::from(&self.source[self.start..self.current]);
        self.tokens
            .push(Token::new(Type, &text, &literal, self.line))
    }

    fn match_next(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.errors.push(error(self.line, "Unterminated string!"));
        }
        self.advance(); // the closing "

        let value = String::from(&self.source[self.start + 1..self.current - 1]);
        self.add_token(TokenType::STRING, &value);
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance(); // Consume the .

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::NUMBER,
            &String::from(&self.source[self.start..self.current]),
        )
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = String::from(&self.source[self.start..self.current]);
        let mut Type = TokenType::IDENTIFIER;
        if self.keywords.contains_key(&text) {
            Type = self.keywords.get(&text).unwrap().clone();
        }

        self.add_empty_token(Type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_create() -> Result<(), String> {
        let scanner = Scanner::new(&String::from("Hei"));
        assert_eq!(scanner.source, "Hei");
        assert_eq!(scanner.start, 0);
        assert_eq!(scanner.current, 0);
        assert_eq!(scanner.line, 1);
        assert_eq!(scanner.tokens.is_empty(), true);
        assert_eq!(scanner.errors.is_empty(), true);
        Ok(())
    }

    #[test]
    fn test_scan_tokens() -> Result<(), String> {
        let mut scanner = Scanner::new(&String::from("*+}(.,-;"));
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 8);
        assert_eq!((&tokens[0]).token_type, TokenType::STAR);
        assert_eq!((&tokens[1]).token_type, TokenType::Plus);
        assert_eq!((&tokens[2]).token_type, TokenType::RightBrace);
        assert_eq!((&tokens[3]).token_type, TokenType::LeftParen);
        assert_eq!((&tokens[4]).token_type, TokenType::Dot);
        assert_eq!((&tokens[5]).token_type, TokenType::Comma);
        assert_eq!((&tokens[6]).token_type, TokenType::Minus);
        assert_eq!((&tokens[7]).token_type, TokenType::SEMICOLON);

        Ok(())
    }

    #[test]
    fn test_double_operator() -> Result<(), String> {
        let mut scanner = Scanner::new(&String::from("<+<=+!+=="));
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 7);
        assert_eq!((&tokens[0]).token_type, TokenType::LESS);
        assert_eq!((&tokens[1]).token_type, TokenType::Plus);
        assert_eq!((&tokens[2]).token_type, TokenType::LessEqual);
        assert_eq!((&tokens[3]).token_type, TokenType::Plus);
        assert_eq!((&tokens[4]).token_type, TokenType::BANG);
        assert_eq!((&tokens[5]).token_type, TokenType::Plus);
        assert_eq!((&tokens[6]).token_type, TokenType::EqualEqual);

        Ok(())
    }

    #[test]
    fn test_comments() -> Result<(), String> {
        let mut scanner = Scanner::new(&String::from("+//hello\n+"));
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 2);

        Ok(())
    }

    #[test]
    fn test_empty_comments() -> Result<(), String> {
        let mut scanner = Scanner::new(&String::from("//hello\n"));
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 0);

        Ok(())
    }

    #[test]
    fn test_lines() -> Result<(), String> {
        let mut scanner = Scanner::new(&String::from("+\n-\n//hello\n/"));
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!((&tokens[0]).line, 1);
        assert_eq!((&tokens[1]).line, 2);
        assert_eq!((&tokens[2]).line, 4);

        Ok(())
    }

    #[test]
    fn test_strings() -> Result<(), String> {
        let mut scanner = Scanner::new(&String::from("+\"Hello\"-\"Hello2\""));
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!((&tokens[1]).token_type, TokenType::STRING);
        assert_eq!((&tokens[1]).literal, "Hello");
        assert_eq!((&tokens[3]).literal, "Hello2");

        Ok(())
    }

    #[test]
    fn test_numbers() -> Result<(), String> {
        let mut scanner = Scanner::new(&String::from("123+123.123"));
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!((&tokens[0]).literal, "123");
        assert_eq!((&tokens[1]).token_type, TokenType::Plus);
        assert_eq!((&tokens[2]).literal, "123.123");

        Ok(())
    }

    #[test]
    fn test_identifier() -> Result<(), String> {
        let mut scanner =
            Scanner::new(&String::from("var + myClass - class + superFres // var \n"));
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(tokens.len(), 7);
        assert_eq!((&tokens[0]).token_type, TokenType::VAR);
        assert_eq!((&tokens[1]).token_type, TokenType::Plus);
        assert_eq!((&tokens[2]).token_type, TokenType::IDENTIFIER);
        assert_eq!((&tokens[2]).lexeme, "myClass");
        assert_eq!((&tokens[3]).token_type, TokenType::Minus);
        assert_eq!((&tokens[4]).token_type, TokenType::CLASS);
        assert_eq!((&tokens[5]).token_type, TokenType::Plus);
        assert_eq!((&tokens[6]).token_type, TokenType::IDENTIFIER);
        assert_eq!((&tokens[6]).lexeme, "superFres");

        Ok(())
    }
}
