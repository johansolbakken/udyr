use crate::{
    node::{Node, NodeType},
    token::{self, TokenType},
};
use std::rc::Rc;

#[derive(Debug)]
pub struct Parser {
    pub(crate) tokens: Vec<token::Token>,
    pub(crate) current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<token::Token>) -> Parser {
        return Parser {
            tokens: tokens.clone(),
            current: 0,
        };
    }

    pub fn parse(&mut self) -> Rc<Node> {
        let expr = self.parse_exspression(5);
        let root = Rc::new(Node::new(NodeType::Program, &[expr.unwrap()]));
        return root;
    }

    pub fn parse_exspression(&mut self, recurse: usize) -> Result<Rc<Node>, String> {
        if recurse == 0 {
            return Err(String::from("Recursion error"));
        }

        let start = self.current;

        let binary = self.parse_binary(recurse - 1);
        match binary {
            Ok(node) => return Ok(Rc::new(Node::new(NodeType::Expression, &[node]))),
            Err(err) => {
                self.current = start;
            }
        }

        let group = self.parse_grouping(recurse - 1);
        match group {
            Ok(node) => return Ok(Rc::new(Node::new(NodeType::Expression, &[node]))),
            Err(err) => {
                self.current = start;
            }
        }

        let unary = self.parse_unary(recurse - 1);
        match unary {
            Ok(node) => return Ok(Rc::new(Node::new(NodeType::Expression, &[node]))),
            Err(err) => {
                self.current = start;
            }
        }

        let literal = self.parse_literal(recurse - 1);
        match literal {
            Ok(node) => return Ok(Rc::new(Node::new(NodeType::Expression, &[node]))),
            Err(err) => {
                self.current = start;
                return Err(err);
            }
        }
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }

    pub fn current_token(&self) -> token::Token {
        return self.tokens[self.current].clone();
    }

    pub fn parse_grouping(&mut self, recurse: usize) -> Result<Rc<Node>, String> {
        if recurse == 0 {
            return Err(String::from("Recursion error"));
        }

        if self.current_token().token_type != TokenType::LeftParen {
            return Err(String::from(""));
        }

        self.advance(); // "("

        let expr = self.parse_exspression(recurse - 1);

        self.advance(); // ")"

        match expr {
            Ok(node) => Ok(Rc::new(Node::new(NodeType::Grouping, &[node]))),
            Err(err) => Err(err),
        }
    }

    pub fn parse_binary(&mut self, recurse: usize) -> Result<Rc<Node>, String> {
        if recurse == 0 {
            return Err(String::from("Recursion error"));
        }
        let start = self.current;
        let expr1 = self.parse_exspression(recurse - 1);
        match expr1 {
            Ok(node) => {
                let op = self.parse_operator(recurse - 1);
                match op {
                    Ok(operator) => {
                        let expr2 = self.parse_exspression(recurse - 1);
                        match expr2 {
                            Ok(node2) => {
                                let expr = Node::new(NodeType::Binary, &[node, operator, node2]);
                                return Ok(Rc::new(expr));
                            }
                            Err(err) => {
                                self.current = start;
                                return Err(err);
                            }
                        }
                    }
                    Err(err) => {
                        self.current = start;
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                self.current = start;
                return Err(err);
            }
        }
    }

    pub fn parse_unary(&mut self, recurse: usize) -> Result<Rc<Node>, String> {
        if recurse == 0 {
            return Err(String::from("Recursion error"));
        }

        let start = self.current;

        if !(self.current_token().token_type == TokenType::BANG
            || self.current_token().token_type == TokenType::Minus)
        {
            return Err(String::from(""));
        }

        let sign = self.current_token();

        self.advance(); // jump over ! or -

        let expr = self.parse_exspression(recurse - 1);
        match expr {
            Ok(node) => {
                let mut unary = Node::new(NodeType::Unary, &[node]);
                unary.token = sign;
                return Ok(Rc::new(unary));
            }
            Err(err) => {
                self.current = start;
                return Err(err);
            }
        }
    }

    pub fn parse_operator(&mut self, recurse: usize) -> Result<Rc<Node>, String> {
        if recurse == 0 {
            return Err(String::from("Recursion error"));
        }
        match self.current_token().token_type {
            TokenType::EqualEqual
            | TokenType::BangEqual
            | TokenType::LESS
            | TokenType::LessEqual
            | TokenType::GreaterEqual
            | TokenType::GREATER
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::STAR
            | TokenType::SLASH => {
                let mut node = Node::new(NodeType::Operator, &[]);
                node.token = self.current_token();
                self.advance();
                return Ok(Rc::new(node));
            }
            _ => Err(String::from("")),
        }
    }

    pub fn parse_literal(&mut self, recurse: usize) -> Result<Rc<Node>, String> {
        if recurse == 0 {
            return Err(String::from("Recursion error"));
        }
        match self.current_token().token_type {
            TokenType::NUMBER | TokenType::STRING => {
                let mut node = Node::new(NodeType::Literal, &[]);
                node.token = self.current_token();
                self.advance();
                return Ok(Rc::new(node));
            }
            _ => { /* Do nothing */ }
        }
        match self.current_token().lexeme.as_str() {
            "true" | "false" | "nil" => {
                let mut node = Node::new(NodeType::Literal, &[]);
                node.token = self.current_token();
                self.advance();
                return Ok(Rc::new(node));
            }
            _ => Err(String::from("")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner;

    use super::*;

    #[test]
    fn test_literal_true() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("true"));
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);

        let node = parser.parse_literal(10).unwrap();
        assert_eq!(node.node_type, NodeType::Literal);
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.token.token_type, TokenType::TRUE);

        Ok(())
    }

    #[test]
    fn test_literal_nil() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("nil"));
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);

        let node = parser.parse_literal(10).unwrap();
        assert_eq!(node.node_type, NodeType::Literal);
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.token.token_type, TokenType::NIL);

        Ok(())
    }

    #[test]
    fn test_literal_num() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("123"));
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);

        let node = parser.parse_literal(10).unwrap();
        assert_eq!(node.node_type, NodeType::Literal);
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.token.token_type, TokenType::NUMBER);

        Ok(())
    }

    #[test]
    fn test_literal_string() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("\"123\""));
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);

        let node = parser.parse_literal(10).unwrap();
        assert_eq!(node.node_type, NodeType::Literal);
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.token.token_type, TokenType::STRING);

        Ok(())
    }

    #[test]
    fn test_operators() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("== != < <= >= > + - * /"));
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);

        let node_equal_equal = parser.parse_operator(10).unwrap();
        assert_eq!(node_equal_equal.node_type, NodeType::Operator);
        assert_eq!(node_equal_equal.children.len(), 0);
        assert_eq!(node_equal_equal.token.token_type, TokenType::EqualEqual);

        let node_bang_equal = parser.parse_operator(10).unwrap();
        assert_eq!(node_bang_equal.node_type, NodeType::Operator);
        assert_eq!(node_bang_equal.children.len(), 0);
        assert_eq!(node_bang_equal.token.token_type, TokenType::BangEqual);

        let node_less = parser.parse_operator(10).unwrap();
        assert_eq!(node_less.node_type, NodeType::Operator);
        assert_eq!(node_less.children.len(), 0);
        assert_eq!(node_less.token.token_type, TokenType::LESS);

        let node_less_equal = parser.parse_operator(10).unwrap();
        assert_eq!(node_less_equal.node_type, NodeType::Operator);
        assert_eq!(node_less_equal.children.len(), 0);
        assert_eq!(node_less_equal.token.token_type, TokenType::LessEqual);

        let node_greater_equal = parser.parse_operator(10).unwrap();
        assert_eq!(node_greater_equal.node_type, NodeType::Operator);
        assert_eq!(node_greater_equal.children.len(), 0);
        assert_eq!(node_greater_equal.token.token_type, TokenType::GreaterEqual);

        let node_greater = parser.parse_operator(10).unwrap();
        assert_eq!(node_greater.node_type, NodeType::Operator);
        assert_eq!(node_greater.children.len(), 0);
        assert_eq!(node_greater.token.token_type, TokenType::GREATER);

        let node_plus = parser.parse_operator(10).unwrap();
        assert_eq!(node_plus.node_type, NodeType::Operator);
        assert_eq!(node_plus.children.len(), 0);
        assert_eq!(node_plus.token.token_type, TokenType::Plus);

        let node_minus = parser.parse_operator(10).unwrap();
        assert_eq!(node_minus.node_type, NodeType::Operator);
        assert_eq!(node_minus.children.len(), 0);
        assert_eq!(node_minus.token.token_type, TokenType::Minus);

        let node_star = parser.parse_operator(10).unwrap();
        assert_eq!(node_star.node_type, NodeType::Operator);
        assert_eq!(node_star.children.len(), 0);
        assert_eq!(node_star.token.token_type, TokenType::STAR);

        let node_slash = parser.parse_operator(10).unwrap();
        assert_eq!(node_slash.node_type, NodeType::Operator);
        assert_eq!(node_slash.children.len(), 0);
        assert_eq!(node_slash.token.token_type, TokenType::SLASH);

        Ok(())
    }

    #[test]
    fn test_unary_minus() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("-5"));
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(&tokens);

        let node_minus = parser.parse_unary(10).unwrap();
        assert_eq!(node_minus.node_type, NodeType::Unary);
        assert_eq!(node_minus.token.token_type, TokenType::Minus);
        assert_eq!(node_minus.children.len(), 1);
        assert_eq!(node_minus.children[0].node_type, NodeType::Expression);
        assert_eq!(node_minus.children[0].children.len(), 1);
        assert_eq!(
            node_minus.children[0].children[0].node_type,
            NodeType::Literal
        );
        assert_eq!(
            node_minus.children[0].children[0].token.token_type,
            TokenType::NUMBER
        );
        assert_eq!(
            node_minus.children[0].children[0].token.lexeme.as_str(),
            "5"
        );

        Ok(())
    }
    #[test]
    fn test_unary_bang() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("!true"));
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(&tokens);

        let node_minus = parser.parse_unary(10).unwrap();
        assert_eq!(node_minus.node_type, NodeType::Unary);
        assert_eq!(node_minus.token.token_type, TokenType::BANG);
        assert_eq!(node_minus.children.len(), 1);
        assert_eq!(node_minus.children[0].node_type, NodeType::Expression);
        assert_eq!(node_minus.children[0].children.len(), 1);
        assert_eq!(
            node_minus.children[0].children[0].node_type,
            NodeType::Literal
        );
        assert_eq!(
            node_minus.children[0].children[0].token.token_type,
            TokenType::TRUE
        );

        Ok(())
    }

    #[test]
    fn test_binary_plus() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("5+4"));
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(&tokens);

        let node_plus = parser.parse_binary(10).unwrap();
        assert_eq!(node_plus.node_type, NodeType::Unary);
        assert_eq!(node_plus.token.token_type, TokenType::BANG);
        assert_eq!(node_plus.children.len(), 1);
        assert_eq!(node_plus.children[0].node_type, NodeType::Expression);
        assert_eq!(node_plus.children[0].children.len(), 1);
        assert_eq!(
            node_plus.children[0].children[0].node_type,
            NodeType::Literal
        );
        assert_eq!(
            node_plus.children[0].children[0].token.token_type,
            TokenType::TRUE
        );

        Ok(())
    }
}
