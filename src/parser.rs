use crate::token::{self, TokenType};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Program,
    Expression,
    Binary,
    Unary,
    Grouping,
    Operator,
    Literal,
}

pub struct Node {
    Type: NodeType,
    children: Vec<Rc<Node>>,
    token: token::Token,
}

impl Node {
    pub fn new(Type: NodeType, children: &[Rc<Node>]) -> Node {
        let mut node = Node {
            Type,
            children: Vec::new(),
            token: token::Token::empty(),
        };

        for child in children {
            node.children.push(child.clone())
        }

        return node;
    }
}

pub struct Parser {
    tokens: Vec<token::Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<token::Token>) -> Parser {
        return Parser {
            tokens: tokens.clone(),
            current: 0,
        };
    }

    pub fn parse(&mut self) -> Rc<Node> {
        let expr = self.parse_exspression();
        let root = Rc::new(Node::new(NodeType::Program, &[expr.unwrap()]));
        return root;
    }

    pub fn parse_exspression(&mut self) -> Result<Rc<Node>, String> {
        let group = self.parse_grouping();
        match group {
            Ok(node) => return Ok(Rc::new(Node::new(NodeType::Expression, &[node]))),
            Err(err) => { /* Do nothing */ }
        }

        let binary = self.parse_binary();
        match binary {
            Ok(node) => return Ok(Rc::new(Node::new(NodeType::Expression, &[node]))),
            Err(err) => { /* Do nothing */ }
        }

        let unary = self.parse_unary();
        match unary {
            Ok(node) => return Ok(Rc::new(Node::new(NodeType::Expression, &[node]))),
            Err(err) => { /* Do nothing */ }
        }

        let literal = self.parse_literal();
        match literal {
            Ok(node) => Ok(Rc::new(Node::new(NodeType::Expression, &[node]))),
            Err(err) => Err(err),
        }
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }

    pub fn current_token(&self) -> token::Token {
        return self.tokens[self.current].clone();
    }

    pub fn parse_grouping(&mut self) -> Result<Rc<Node>, String> {
        if self.current_token().Type != TokenType::LeftParen {
            return Err(String::from(""));
        }

        self.advance(); // "("

        let expr = self.parse_exspression();

        self.advance(); // ")"

        match expr {
            Ok(node) => Ok(Rc::new(Node::new(NodeType::Grouping, &[node]))),
            Err(err) => Err(err),
        }
    }

    pub fn parse_binary(&mut self) -> Result<Rc<Node>, String> {
        let start = self.current;
        let expr1 = self.parse_exspression();
        match expr1 {
            Ok(node) => {
                let op = self.parse_operator();
                match op {
                    Ok(operator) => {
                        let expr2 = self.parse_exspression();
                        match expr2 {
                            Ok(node2) => {
                                let expr =
                                    Node::new(NodeType::Expression, &[node, operator, node2]);
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

    pub fn parse_unary(&mut self) -> Result<Rc<Node>, String> {
        if !(self.current_token().Type == TokenType::BANG
            || self.current_token().Type == TokenType::Minus)
        {
            return Err(String::from(""));
        }

        let sign = self.current_token();

        self.advance(); // jump over ! or -

        let expr = self.parse_exspression();
        match expr {
            Ok(node) => {
                let mut unary = Node::new(NodeType::Unary, &[node]);
                unary.token = sign;
                return Ok(Rc::new(unary));
            }
            Err(err) => Err(err),
        }
    }

    pub fn parse_operator(&mut self) -> Result<Rc<Node>, String> {
        match self.current_token().Type {
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

    pub fn parse_literal(&mut self) -> Result<Rc<Node>, String> {
        match self.current_token().Type {
            TokenType::NUMBER | TokenType::STRING => {
                let mut node = Node::new(NodeType::Literal, &[]);
                node.token = self.current_token();
                return Ok(Rc::new(node));
            }
            _ => {}
        }
        match self.current_token().lexeme.as_str() {
            "true" | "false" | "nil" => {
                let mut node = Node::new(NodeType::Literal, &[]);
                node.token = self.current_token();
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

        let node = parser.parse_literal().unwrap();
        assert_eq!(node.Type, NodeType::Literal);
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.token.Type, TokenType::TRUE);

        Ok(())
    }

    #[test]
    fn test_literal_nil() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("nil"));
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);

        let node = parser.parse_literal().unwrap();
        assert_eq!(node.Type, NodeType::Literal);
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.token.Type, TokenType::NIL);

        Ok(())
    }

    #[test]
    fn test_literal_num() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("123"));
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);

        let node = parser.parse_literal().unwrap();
        assert_eq!(node.Type, NodeType::Literal);
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.token.Type, TokenType::NUMBER);

        Ok(())
    }

    #[test]
    fn test_literal_string() -> Result<(), String> {
        let mut scanner = scanner::Scanner::new(&String::from("\"123\""));
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(&tokens);

        let node = parser.parse_literal().unwrap();
        assert_eq!(node.Type, NodeType::Literal);
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.token.Type, TokenType::STRING);

        Ok(())
    }
}
