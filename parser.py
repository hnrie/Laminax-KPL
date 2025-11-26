from token_types import Token, TokenType
from ast_nodes import *
from errors import ParserError

class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.position = 0
        self.current_token = self.tokens[0] if tokens else None
    
    def advance(self):
        self.position += 1
        if self.position < len(self.tokens):
            self.current_token = self.tokens[self.position]
        else:
            self.current_token = None
    
    def peek(self, offset=1):
        peek_pos = self.position + offset
        if peek_pos < len(self.tokens):
            return self.tokens[peek_pos]
        return None
    
    def expect(self, token_type):
        if self.current_token.type != token_type:
            raise ParserError(
                f"Expected {token_type}, got {self.current_token.type}",
                self.current_token.line,
                self.current_token.column
            )
        token = self.current_token
        self.advance()
        return token
    
    def skip_newlines(self):
        while self.current_token and self.current_token.type == TokenType.NEWLINE:
            self.advance()
    
    def parse(self):
        statements = []
        self.skip_newlines()
        
        while self.current_token and self.current_token.type != TokenType.EOF:
            stmt = self.parse_statement()
            if stmt:
                statements.append(stmt)
            self.skip_newlines()
        
        return BlockNode(statements)
    
    def parse_statement(self):
        self.skip_newlines()
        
        if not self.current_token or self.current_token.type == TokenType.EOF:
            return None
        
        if self.current_token.type == TokenType.KEYWORD_LET:
            return self.parse_let_statement()
        elif self.current_token.type == TokenType.KEYWORD_FUNC:
            return self.parse_function()
        elif self.current_token.type == TokenType.KEYWORD_IF:
            return self.parse_if_statement()
        elif self.current_token.type == TokenType.KEYWORD_WHILE:
            return self.parse_while_statement()
        elif self.current_token.type == TokenType.KEYWORD_FOR:
            return self.parse_for_statement()
        elif self.current_token.type == TokenType.KEYWORD_RETURN:
            return self.parse_return_statement()
        elif self.current_token.type == TokenType.KEYWORD_BREAK:
            self.advance()
            return BreakNode()
        elif self.current_token.type == TokenType.KEYWORD_CONTINUE:
            self.advance()
            return ContinueNode()
        else:
            return self.parse_expression_statement()
    
    def parse_let_statement(self):
        self.advance()
        name = self.expect(TokenType.IDENTIFIER).value
        self.expect(TokenType.EQUALS)
        value = self.parse_expression()
        return AssignmentNode(name, value)
    
    def parse_function(self):
        self.advance()
        name = self.expect(TokenType.IDENTIFIER).value
        self.expect(TokenType.LPAREN)
        
        parameters = []
        while self.current_token.type != TokenType.RPAREN:
            parameters.append(self.expect(TokenType.IDENTIFIER).value)
            if self.current_token.type == TokenType.COMMA:
                self.advance()
        
        self.expect(TokenType.RPAREN)
        self.expect(TokenType.LBRACE)
        self.skip_newlines()
        
        body = []
        while self.current_token.type != TokenType.RBRACE:
            stmt = self.parse_statement()
            if stmt:
                body.append(stmt)
            self.skip_newlines()
        
        self.expect(TokenType.RBRACE)
        return FunctionNode(name, parameters, BlockNode(body))
    
    def parse_if_statement(self):
        self.advance()
        condition = self.parse_expression()
        self.expect(TokenType.LBRACE)
        self.skip_newlines()
        
        then_branch = []
        while self.current_token.type != TokenType.RBRACE:
            stmt = self.parse_statement()
            if stmt:
                then_branch.append(stmt)
            self.skip_newlines()
        
        self.expect(TokenType.RBRACE)
        self.skip_newlines()
        
        elif_branches = []
        while self.current_token and self.current_token.type == TokenType.KEYWORD_ELIF:
            self.advance()
            elif_condition = self.parse_expression()
            self.expect(TokenType.LBRACE)
            self.skip_newlines()
            
            elif_body = []
            while self.current_token.type != TokenType.RBRACE:
                stmt = self.parse_statement()
                if stmt:
                    elif_body.append(stmt)
                self.skip_newlines()
            
            self.expect(TokenType.RBRACE)
            self.skip_newlines()
            elif_branches.append((elif_condition, BlockNode(elif_body)))
        
        else_branch = None
        if self.current_token and self.current_token.type == TokenType.KEYWORD_ELSE:
            self.advance()
            self.expect(TokenType.LBRACE)
            self.skip_newlines()
            
            else_body = []
            while self.current_token.type != TokenType.RBRACE:
                stmt = self.parse_statement()
                if stmt:
                    else_body.append(stmt)
                self.skip_newlines()
            
            self.expect(TokenType.RBRACE)
            else_branch = BlockNode(else_body)
        
        return IfNode(condition, BlockNode(then_branch), elif_branches, else_branch)
    
    def parse_while_statement(self):
        self.advance()
        condition = self.parse_expression()
        self.expect(TokenType.LBRACE)
        self.skip_newlines()
        
        body = []
        while self.current_token.type != TokenType.RBRACE:
            stmt = self.parse_statement()
            if stmt:
                body.append(stmt)
            self.skip_newlines()
        
        self.expect(TokenType.RBRACE)
        return WhileNode(condition, BlockNode(body))
    
    def parse_for_statement(self):
        self.advance()
        variable = self.expect(TokenType.IDENTIFIER).value
        self.expect(TokenType.KEYWORD_IN)
        iterable = self.parse_expression()
        self.expect(TokenType.LBRACE)
        self.skip_newlines()
        
        body = []
        while self.current_token.type != TokenType.RBRACE:
            stmt = self.parse_statement()
            if stmt:
                body.append(stmt)
            self.skip_newlines()
        
        self.expect(TokenType.RBRACE)
        return ForNode(variable, iterable, BlockNode(body))
    
    def parse_return_statement(self):
        self.advance()
        if self.current_token.type in [TokenType.NEWLINE, TokenType.RBRACE, TokenType.EOF]:
            return ReturnNode(NullNode())
        value = self.parse_expression()
        return ReturnNode(value)
    
    def parse_expression_statement(self):
        expr = self.parse_expression()
        
        if isinstance(expr, IdentifierNode):
            if self.current_token.type == TokenType.EQUALS:
                self.advance()
                value = self.parse_expression()
                return AssignmentNode(expr.name, value)
            elif self.current_token.type in [TokenType.PLUS_EQUALS, TokenType.MINUS_EQUALS, 
                                             TokenType.STAR_EQUALS, TokenType.SLASH_EQUALS]:
                operator = self.current_token.type
                self.advance()
                value = self.parse_expression()
                return CompoundAssignmentNode(expr.name, operator, value)
        
        return expr
    
    def parse_expression(self):
        return self.parse_or()
    
    def parse_or(self):
        left = self.parse_and()
        
        while self.current_token and self.current_token.type == TokenType.KEYWORD_OR:
            operator = self.current_token.type
            self.advance()
            right = self.parse_and()
            left = BinaryOpNode(left, operator, right)
        
        return left
    
    def parse_and(self):
        left = self.parse_equality()
        
        while self.current_token and self.current_token.type == TokenType.KEYWORD_AND:
            operator = self.current_token.type
            self.advance()
            right = self.parse_equality()
            left = BinaryOpNode(left, operator, right)
        
        return left
    
    def parse_equality(self):
        left = self.parse_comparison()
        
        while self.current_token and self.current_token.type in [TokenType.EQUALS_EQUALS, TokenType.NOT_EQUALS]:
            operator = self.current_token.type
            self.advance()
            right = self.parse_comparison()
            left = BinaryOpNode(left, operator, right)
        
        return left
    
    def parse_comparison(self):
        left = self.parse_addition()
        
        while self.current_token and self.current_token.type in [TokenType.LESS_THAN, TokenType.GREATER_THAN, 
                                                                   TokenType.LESS_EQUALS, TokenType.GREATER_EQUALS]:
            operator = self.current_token.type
            self.advance()
            right = self.parse_addition()
            left = BinaryOpNode(left, operator, right)
        
        return left
    
    def parse_addition(self):
        left = self.parse_multiplication()
        
        while self.current_token and self.current_token.type in [TokenType.PLUS, TokenType.MINUS]:
            operator = self.current_token.type
            self.advance()
            right = self.parse_multiplication()
            left = BinaryOpNode(left, operator, right)
        
        return left
    
    def parse_multiplication(self):
        left = self.parse_power()
        
        while self.current_token and self.current_token.type in [TokenType.STAR, TokenType.SLASH, TokenType.PERCENT]:
            operator = self.current_token.type
            self.advance()
            right = self.parse_power()
            left = BinaryOpNode(left, operator, right)
        
        return left
    
    def parse_power(self):
        left = self.parse_unary()
        
        if self.current_token and self.current_token.type == TokenType.POWER:
            operator = self.current_token.type
            self.advance()
            right = self.parse_power()
            return BinaryOpNode(left, operator, right)
        
        return left
    
    def parse_unary(self):
        if self.current_token and self.current_token.type in [TokenType.MINUS, TokenType.KEYWORD_NOT]:
            operator = self.current_token.type
            self.advance()
            operand = self.parse_unary()
            return UnaryOpNode(operator, operand)
        
        return self.parse_postfix()
    
    def parse_postfix(self):
        expr = self.parse_primary()
        
        while True:
            if self.current_token and self.current_token.type == TokenType.LPAREN:
                self.advance()
                arguments = []
                
                while self.current_token.type != TokenType.RPAREN:
                    arguments.append(self.parse_expression())
                    if self.current_token.type == TokenType.COMMA:
                        self.advance()
                
                self.expect(TokenType.RPAREN)
                expr = CallNode(expr, arguments)
            
            elif self.current_token and self.current_token.type == TokenType.LBRACKET:
                self.advance()
                index = self.parse_expression()
                self.expect(TokenType.RBRACKET)
                expr = IndexNode(expr, index)
            
            elif self.current_token and self.current_token.type == TokenType.DOT:
                self.advance()
                member = self.expect(TokenType.IDENTIFIER).value
                expr = MemberAccessNode(expr, member)
            
            else:
                break
        
        return expr
    
    def parse_primary(self):
        if self.current_token.type == TokenType.NUMBER:
            value = self.current_token.value
            self.advance()
            return NumberNode(value)
        
        elif self.current_token.type == TokenType.STRING:
            value = self.current_token.value
            self.advance()
            return StringNode(value)
        
        elif self.current_token.type == TokenType.KEYWORD_TRUE:
            self.advance()
            return BooleanNode(True)
        
        elif self.current_token.type == TokenType.KEYWORD_FALSE:
            self.advance()
            return BooleanNode(False)
        
        elif self.current_token.type == TokenType.KEYWORD_NULL:
            self.advance()
            return NullNode()
        
        elif self.current_token.type == TokenType.IDENTIFIER:
            name = self.current_token.value
            self.advance()
            return IdentifierNode(name)
        
        elif self.current_token.type == TokenType.LPAREN:
            self.advance()
            expr = self.parse_expression()
            self.expect(TokenType.RPAREN)
            return expr
        
        elif self.current_token.type == TokenType.LBRACKET:
            self.advance()
            elements = []
            
            while self.current_token.type != TokenType.RBRACKET:
                elements.append(self.parse_expression())
                if self.current_token.type == TokenType.COMMA:
                    self.advance()
            
            self.expect(TokenType.RBRACKET)
            return ListNode(elements)
        
        else:
            raise ParserError(
                f"Unexpected token: {self.current_token.type}",
                self.current_token.line,
                self.current_token.column
            )
