use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number,
    String,
    Identifier,
    
    // Keywords
    KeywordLet,
    KeywordFunc,
    KeywordIf,
    KeywordElse,
    KeywordElif,
    KeywordWhile,
    KeywordFor,
    KeywordIn,
    KeywordReturn,
    KeywordBreak,
    KeywordContinue,
    KeywordTrue,
    KeywordFalse,
    KeywordNull,
    KeywordAnd,
    KeywordOr,
    KeywordNot,
    KeywordClass,
    KeywordImport,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    
    // Comparison
    Equals,
    EqualsEquals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessEquals,
    GreaterEquals,
    
    // Compound assignment
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    
    // Punctuation
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    
    // Special
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: TokenValue,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    Number(f64),
    String(String),
    Identifier(String),
    Symbol(String),
    None,
}

impl Token {
    pub fn new(token_type: TokenType, value: TokenValue, line: usize, column: usize) -> Self {
        Self {
            token_type,
            value,
            line,
            column,
        }
    }
    
    pub fn number(value: f64, line: usize, column: usize) -> Self {
        Self::new(TokenType::Number, TokenValue::Number(value), line, column)
    }
    
    pub fn string(value: String, line: usize, column: usize) -> Self {
        Self::new(TokenType::String, TokenValue::String(value), line, column)
    }
    
    pub fn identifier(value: String, line: usize, column: usize) -> Self {
        Self::new(TokenType::Identifier, TokenValue::Identifier(value), line, column)
    }
    
    pub fn symbol(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Self::new(token_type, TokenValue::Symbol(value), line, column)
    }
    
    pub fn simple(token_type: TokenType, line: usize, column: usize) -> Self {
        Self::new(token_type, TokenValue::None, line, column)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token({:?}, {:?}, {}:{})", self.token_type, self.value, self.line, self.column)
    }
}

pub fn create_keywords_map() -> HashMap<String, TokenType> {
    let mut keywords = HashMap::new();
    
    keywords.insert("let".to_string(), TokenType::KeywordLet);
    keywords.insert("func".to_string(), TokenType::KeywordFunc);
    keywords.insert("if".to_string(), TokenType::KeywordIf);
    keywords.insert("else".to_string(), TokenType::KeywordElse);
    keywords.insert("elif".to_string(), TokenType::KeywordElif);
    keywords.insert("while".to_string(), TokenType::KeywordWhile);
    keywords.insert("for".to_string(), TokenType::KeywordFor);
    keywords.insert("in".to_string(), TokenType::KeywordIn);
    keywords.insert("return".to_string(), TokenType::KeywordReturn);
    keywords.insert("break".to_string(), TokenType::KeywordBreak);
    keywords.insert("continue".to_string(), TokenType::KeywordContinue);
    keywords.insert("true".to_string(), TokenType::KeywordTrue);
    keywords.insert("false".to_string(), TokenType::KeywordFalse);
    keywords.insert("null".to_string(), TokenType::KeywordNull);
    keywords.insert("and".to_string(), TokenType::KeywordAnd);
    keywords.insert("or".to_string(), TokenType::KeywordOr);
    keywords.insert("not".to_string(), TokenType::KeywordNot);
    keywords.insert("class".to_string(), TokenType::KeywordClass);
    keywords.insert("import".to_string(), TokenType::KeywordImport);
    
    keywords
}
