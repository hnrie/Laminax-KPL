use crate::errors::{KyaroError, Result};
use crate::token_types::{Token, TokenType, TokenValue, create_keywords_map};
use std::collections::HashMap;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    current_char: Option<char>,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        let current_char = chars.get(0).copied();
        
        Self {
            source: chars,
            position: 0,
            line: 1,
            column: 1,
            current_char,
            keywords: create_keywords_map(),
        }
    }
    
    fn advance(&mut self) {
        if self.current_char == Some('\n') {
            self.line += 1;
            self.column = 0;
        }
        
        self.position += 1;
        self.column += 1;
        
        self.current_char = if self.position < self.source.len() {
            Some(self.source[self.position])
        } else {
            None
        };
    }
    
    fn peek(&self, offset: usize) -> Option<char> {
        let peek_pos = self.position + offset;
        if peek_pos < self.source.len() {
            Some(self.source[peek_pos])
        } else {
            None
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        if self.current_char == Some('#') {
            while let Some(ch) = self.current_char {
                if ch == '\n' {
                    break;
                }
                self.advance();
            }
        }
    }
    
    fn read_number(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let mut num_str = String::new();
        let mut has_dot = false;
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        let value = if has_dot {
            num_str.parse::<f64>()
                .map_err(|_| KyaroError::lexer_error("Invalid number format", start_line, start_column))?
        } else {
            num_str.parse::<i64>()
                .map_err(|_| KyaroError::lexer_error("Invalid number format", start_line, start_column))?
                as f64
        };
        
        Ok(Token::number(value, start_line, start_column))
    }
    
    fn read_string(&mut self, quote_char: char) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        self.advance(); // Skip opening quote
        
        let mut string_value = String::new();
        
        while let Some(ch) = self.current_char {
            if ch == quote_char {
                break;
            }
            
            if ch == '\\' {
                self.advance();
                match self.current_char {
                    Some('n') => string_value.push('\n'),
                    Some('t') => string_value.push('\t'),
                    Some('r') => string_value.push('\r'),
                    Some('\\') => string_value.push('\\'),
                    Some(c) if c == quote_char => string_value.push(quote_char),
                    Some(c) => string_value.push(c),
                    None => return Err(KyaroError::lexer_error("Unterminated string", start_line, start_column)),
                }
                self.advance();
            } else {
                string_value.push(ch);
                self.advance();
            }
        }
        
        if self.current_char != Some(quote_char) {
            return Err(KyaroError::lexer_error("Unterminated string", start_line, start_column));
        }
        
        self.advance(); // Skip closing quote
        Ok(Token::string(string_value, start_line, start_column))
    }
    
    fn read_identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        let token_type = self.keywords.get(&identifier)
            .cloned()
            .unwrap_or(TokenType::Identifier);
        
        Token::new(token_type, TokenValue::Identifier(identifier), start_line, start_column)
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        
        while self.current_char.is_some() {
            self.skip_whitespace();
            
            if self.current_char.is_none() {
                break;
            }
            
            if self.current_char == Some('#') {
                self.skip_comment();
                continue;
            }
            
            if self.current_char == Some('\n') {
                tokens.push(Token::simple(TokenType::Newline, self.line, self.column));
                self.advance();
                continue;
            }
            
            if let Some(ch) = self.current_char {
                if ch.is_ascii_digit() {
                    tokens.push(self.read_number()?);
                    continue;
                }
                
                if ch == '"' || ch == '\'' {
                    tokens.push(self.read_string(ch)?);
                    continue;
                }
                
                if ch.is_alphabetic() || ch == '_' {
                    tokens.push(self.read_identifier());
                    continue;
                }
                
                let start_line = self.line;
                let start_column = self.column;
                
                match ch {
                    '+' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::PlusEquals, "+=".to_string(), start_line, start_column));
                        } else {
                            tokens.push(Token::symbol(TokenType::Plus, "+".to_string(), start_line, start_column));
                        }
                    }
                    '-' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::MinusEquals, "-=".to_string(), start_line, start_column));
                        } else if self.current_char == Some('>') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::Arrow, "->".to_string(), start_line, start_column));
                        } else {
                            tokens.push(Token::symbol(TokenType::Minus, "-".to_string(), start_line, start_column));
                        }
                    }
                    '*' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::StarEquals, "*=".to_string(), start_line, start_column));
                        } else if self.current_char == Some('*') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::Power, "**".to_string(), start_line, start_column));
                        } else {
                            tokens.push(Token::symbol(TokenType::Star, "*".to_string(), start_line, start_column));
                        }
                    }
                    '/' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::SlashEquals, "/=".to_string(), start_line, start_column));
                        } else {
                            tokens.push(Token::symbol(TokenType::Slash, "/".to_string(), start_line, start_column));
                        }
                    }
                    '%' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::Percent, "%".to_string(), start_line, start_column));
                    }
                    '=' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::EqualsEquals, "==".to_string(), start_line, start_column));
                        } else {
                            tokens.push(Token::symbol(TokenType::Equals, "=".to_string(), start_line, start_column));
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::NotEquals, "!=".to_string(), start_line, start_column));
                        } else {
                            return Err(KyaroError::lexer_error(format!("Unexpected character: {}", ch), start_line, start_column));
                        }
                    }
                    '<' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::LessEquals, "<=".to_string(), start_line, start_column));
                        } else {
                            tokens.push(Token::symbol(TokenType::LessThan, "<".to_string(), start_line, start_column));
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            self.advance();
                            tokens.push(Token::symbol(TokenType::GreaterEquals, ">=".to_string(), start_line, start_column));
                        } else {
                            tokens.push(Token::symbol(TokenType::GreaterThan, ">".to_string(), start_line, start_column));
                        }
                    }
                    '(' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::LParen, "(".to_string(), start_line, start_column));
                    }
                    ')' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::RParen, ")".to_string(), start_line, start_column));
                    }
                    '{' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::LBrace, "{".to_string(), start_line, start_column));
                    }
                    '}' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::RBrace, "}".to_string(), start_line, start_column));
                    }
                    '[' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::LBracket, "[".to_string(), start_line, start_column));
                    }
                    ']' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::RBracket, "]".to_string(), start_line, start_column));
                    }
                    ',' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::Comma, ",".to_string(), start_line, start_column));
                    }
                    '.' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::Dot, ".".to_string(), start_line, start_column));
                    }
                    ':' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::Colon, ":".to_string(), start_line, start_column));
                    }
                    ';' => {
                        self.advance();
                        tokens.push(Token::symbol(TokenType::Semicolon, ";".to_string(), start_line, start_column));
                    }
                    _ => {
                        return Err(KyaroError::lexer_error(format!("Unexpected character: {}", ch), start_line, start_column));
                    }
                }
            }
        }
        
        tokens.push(Token::simple(TokenType::Eof, self.line, self.column));
        Ok(tokens)
    }
}
