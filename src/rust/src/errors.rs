use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum KyaroError {
    #[error("Lexer error at line {line}, column {column}: {message}")]
    LexerError {
        message: String,
        line: usize,
        column: usize,
    },
    
    #[error("Parser error at line {line}, column {column}: {message}")]
    ParserError {
        message: String,
        line: usize,
        column: usize,
    },
    
    #[error("Runtime error at line {line}, column {column}: {message}")]
    RuntimeError {
        message: String,
        line: usize,
        column: usize,
    },
    
    #[error("Error: {message}")]
    GenericError { message: String },
}

impl KyaroError {
    pub fn lexer_error(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::LexerError {
            message: message.into(),
            line,
            column,
        }
    }
    
    pub fn parser_error(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::ParserError {
            message: message.into(),
            line,
            column,
        }
    }
    
    pub fn runtime_error(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::RuntimeError {
            message: message.into(),
            line,
            column,
        }
    }
    
    pub fn generic_error(message: impl Into<String>) -> Self {
        Self::GenericError {
            message: message.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, KyaroError>;
