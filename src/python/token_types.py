from enum import Enum, auto

class TokenType(Enum):
    NUMBER = auto()
    STRING = auto()
    IDENTIFIER = auto()
    
    KEYWORD_LET = auto()
    KEYWORD_FUNC = auto()
    KEYWORD_IF = auto()
    KEYWORD_ELSE = auto()
    KEYWORD_ELIF = auto()
    KEYWORD_WHILE = auto()
    KEYWORD_FOR = auto()
    KEYWORD_IN = auto()
    KEYWORD_RETURN = auto()
    KEYWORD_BREAK = auto()
    KEYWORD_CONTINUE = auto()
    KEYWORD_TRUE = auto()
    KEYWORD_FALSE = auto()
    KEYWORD_NULL = auto()
    KEYWORD_AND = auto()
    KEYWORD_OR = auto()
    KEYWORD_NOT = auto()
    KEYWORD_CLASS = auto()
    KEYWORD_IMPORT = auto()
    
    PLUS = auto()
    MINUS = auto()
    STAR = auto()
    SLASH = auto()
    PERCENT = auto()
    POWER = auto()
    
    EQUALS = auto()
    EQUALS_EQUALS = auto()
    NOT_EQUALS = auto()
    LESS_THAN = auto()
    GREATER_THAN = auto()
    LESS_EQUALS = auto()
    GREATER_EQUALS = auto()
    
    PLUS_EQUALS = auto()
    MINUS_EQUALS = auto()
    STAR_EQUALS = auto()
    SLASH_EQUALS = auto()
    
    LPAREN = auto()
    RPAREN = auto()
    LBRACE = auto()
    RBRACE = auto()
    LBRACKET = auto()
    RBRACKET = auto()
    
    COMMA = auto()
    DOT = auto()
    COLON = auto()
    SEMICOLON = auto()
    ARROW = auto()
    
    NEWLINE = auto()
    EOF = auto()

KEYWORDS = {
    'let': TokenType.KEYWORD_LET,
    'func': TokenType.KEYWORD_FUNC,
    'if': TokenType.KEYWORD_IF,
    'else': TokenType.KEYWORD_ELSE,
    'elif': TokenType.KEYWORD_ELIF,
    'while': TokenType.KEYWORD_WHILE,
    'for': TokenType.KEYWORD_FOR,
    'in': TokenType.KEYWORD_IN,
    'return': TokenType.KEYWORD_RETURN,
    'break': TokenType.KEYWORD_BREAK,
    'continue': TokenType.KEYWORD_CONTINUE,
    'true': TokenType.KEYWORD_TRUE,
    'false': TokenType.KEYWORD_FALSE,
    'null': TokenType.KEYWORD_NULL,
    'and': TokenType.KEYWORD_AND,
    'or': TokenType.KEYWORD_OR,
    'not': TokenType.KEYWORD_NOT,
    'class': TokenType.KEYWORD_CLASS,
    'import': TokenType.KEYWORD_IMPORT,
}

class Token:
    def __init__(self, token_type, value, line, column):
        self.type = token_type
        self.value = value
        self.line = line
        self.column = column
    
    def __repr__(self):
        return f'Token({self.type}, {self.value!r}, {self.line}:{self.column})'
