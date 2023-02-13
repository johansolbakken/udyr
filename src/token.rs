#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: String,
    pub(crate) line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &String, literal: &String, line: usize) -> Token {
        return Token {
            token_type,
            lexeme: lexeme.clone(),
            literal: literal.clone(),
            line,
        };
    }

    pub fn empty() -> Token {
        Token {
            token_type: TokenType::None,
            lexeme: String::from(""),
            literal: String::from(""),
            line: 0,
        }
    }
}
