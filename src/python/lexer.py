from token_types import Token, TokenType, KEYWORDS
from errors import LexerError

class Lexer:
    def __init__(self, source):
        self.source = source
        self.position = 0
        self.line = 1
        self.column = 1
        self.current_char = self.source[0] if source else None
    
    def advance(self):
        if self.current_char == '\n':
            self.line += 1
            self.column = 0
        self.position += 1
        self.column += 1
        if self.position < len(self.source):
            self.current_char = self.source[self.position]
        else:
            self.current_char = None
    
    def peek(self, offset=1):
        peek_pos = self.position + offset
        if peek_pos < len(self.source):
            return self.source[peek_pos]
        return None
    
    def skip_whitespace(self):
        while self.current_char and self.current_char in ' \t\r':
            self.advance()
    
    def skip_comment(self):
        if self.current_char == '#':
            while self.current_char and self.current_char != '\n':
                self.advance()
    
    def read_number(self):
        start_line = self.line
        start_column = self.column
        num_str = ''
        has_dot = False
        
        while self.current_char and (self.current_char.isdigit() or self.current_char == '.'):
            if self.current_char == '.':
                if has_dot:
                    raise LexerError("Invalid number format", start_line, start_column)
                has_dot = True
            num_str += self.current_char
            self.advance()
        
        value = float(num_str) if has_dot else int(num_str)
        return Token(TokenType.NUMBER, value, start_line, start_column)
    
    def read_string(self, quote_char):
        start_line = self.line
        start_column = self.column
        self.advance()
        
        string_value = ''
        while self.current_char and self.current_char != quote_char:
            if self.current_char == '\\':
                self.advance()
                if self.current_char == 'n':
                    string_value += '\n'
                elif self.current_char == 't':
                    string_value += '\t'
                elif self.current_char == 'r':
                    string_value += '\r'
                elif self.current_char == '\\':
                    string_value += '\\'
                elif self.current_char == quote_char:
                    string_value += quote_char
                else:
                    string_value += self.current_char
                self.advance()
            else:
                string_value += self.current_char
                self.advance()
        
        if self.current_char != quote_char:
            raise LexerError("Unterminated string", start_line, start_column)
        
        self.advance()
        return Token(TokenType.STRING, string_value, start_line, start_column)
    
    def read_identifier(self):
        start_line = self.line
        start_column = self.column
        identifier = ''
        
        while self.current_char and (self.current_char.isalnum() or self.current_char == '_'):
            identifier += self.current_char
            self.advance()
        
        token_type = KEYWORDS.get(identifier, TokenType.IDENTIFIER)
        return Token(token_type, identifier, start_line, start_column)
    
    def tokenize(self):
        tokens = []
        
        while self.current_char:
            self.skip_whitespace()
            
            if not self.current_char:
                break
            
            if self.current_char == '#':
                self.skip_comment()
                continue
            
            if self.current_char == '\n':
                tokens.append(Token(TokenType.NEWLINE, '\n', self.line, self.column))
                self.advance()
                continue
            
            if self.current_char.isdigit():
                tokens.append(self.read_number())
                continue
            
            if self.current_char in '"\'':
                tokens.append(self.read_string(self.current_char))
                continue
            
            if self.current_char.isalpha() or self.current_char == '_':
                tokens.append(self.read_identifier())
                continue
            
            start_line = self.line
            start_column = self.column
            
            if self.current_char == '+':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    tokens.append(Token(TokenType.PLUS_EQUALS, '+=', start_line, start_column))
                else:
                    tokens.append(Token(TokenType.PLUS, '+', start_line, start_column))
            elif self.current_char == '-':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    tokens.append(Token(TokenType.MINUS_EQUALS, '-=', start_line, start_column))
                elif self.current_char == '>':
                    self.advance()
                    tokens.append(Token(TokenType.ARROW, '->', start_line, start_column))
                else:
                    tokens.append(Token(TokenType.MINUS, '-', start_line, start_column))
            elif self.current_char == '*':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    tokens.append(Token(TokenType.STAR_EQUALS, '*=', start_line, start_column))
                elif self.current_char == '*':
                    self.advance()
                    tokens.append(Token(TokenType.POWER, '**', start_line, start_column))
                else:
                    tokens.append(Token(TokenType.STAR, '*', start_line, start_column))
            elif self.current_char == '/':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    tokens.append(Token(TokenType.SLASH_EQUALS, '/=', start_line, start_column))
                else:
                    tokens.append(Token(TokenType.SLASH, '/', start_line, start_column))
            elif self.current_char == '%':
                self.advance()
                tokens.append(Token(TokenType.PERCENT, '%', start_line, start_column))
            elif self.current_char == '=':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    tokens.append(Token(TokenType.EQUALS_EQUALS, '==', start_line, start_column))
                else:
                    tokens.append(Token(TokenType.EQUALS, '=', start_line, start_column))
            elif self.current_char == '!':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    tokens.append(Token(TokenType.NOT_EQUALS, '!=', start_line, start_column))
                else:
                    raise LexerError(f"Unexpected character: {self.current_char}", start_line, start_column)
            elif self.current_char == '<':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    tokens.append(Token(TokenType.LESS_EQUALS, '<=', start_line, start_column))
                else:
                    tokens.append(Token(TokenType.LESS_THAN, '<', start_line, start_column))
            elif self.current_char == '>':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    tokens.append(Token(TokenType.GREATER_EQUALS, '>=', start_line, start_column))
                else:
                    tokens.append(Token(TokenType.GREATER_THAN, '>', start_line, start_column))
            elif self.current_char == '(':
                self.advance()
                tokens.append(Token(TokenType.LPAREN, '(', start_line, start_column))
            elif self.current_char == ')':
                self.advance()
                tokens.append(Token(TokenType.RPAREN, ')', start_line, start_column))
            elif self.current_char == '{':
                self.advance()
                tokens.append(Token(TokenType.LBRACE, '{', start_line, start_column))
            elif self.current_char == '}':
                self.advance()
                tokens.append(Token(TokenType.RBRACE, '}', start_line, start_column))
            elif self.current_char == '[':
                self.advance()
                tokens.append(Token(TokenType.LBRACKET, '[', start_line, start_column))
            elif self.current_char == ']':
                self.advance()
                tokens.append(Token(TokenType.RBRACKET, ']', start_line, start_column))
            elif self.current_char == ',':
                self.advance()
                tokens.append(Token(TokenType.COMMA, ',', start_line, start_column))
            elif self.current_char == '.':
                self.advance()
                tokens.append(Token(TokenType.DOT, '.', start_line, start_column))
            elif self.current_char == ':':
                self.advance()
                tokens.append(Token(TokenType.COLON, ':', start_line, start_column))
            elif self.current_char == ';':
                self.advance()
                tokens.append(Token(TokenType.SEMICOLON, ';', start_line, start_column))
            else:
                raise LexerError(f"Unexpected character: {self.current_char}", start_line, start_column)
        
        tokens.append(Token(TokenType.EOF, None, self.line, self.column))
        return tokens
