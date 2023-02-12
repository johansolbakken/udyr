use crate::error::error;
use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<String>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &String) -> Scanner {
        return Scanner {
            source: source.clone(),
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
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
            _ => self.errors.push(error(self.line, "Unexpected character.")),
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
        assert_eq!((&tokens[0]).Type, TokenType::STAR);
        assert_eq!((&tokens[1]).Type, TokenType::Plus);
        assert_eq!((&tokens[2]).Type, TokenType::RightBrace);
        assert_eq!((&tokens[3]).Type, TokenType::LeftParen);
        assert_eq!((&tokens[4]).Type, TokenType::Dot);
        assert_eq!((&tokens[5]).Type, TokenType::Comma);
        assert_eq!((&tokens[6]).Type, TokenType::Minus);
        assert_eq!((&tokens[7]).Type, TokenType::SEMICOLON);

        Ok(())
    }
}
