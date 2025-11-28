use crate::errors::{KyaroError, Result};
use crate::token_types::{Token, TokenType, TokenValue};
use crate::ast_nodes::*;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let current_token = tokens.get(0).cloned();
        Self {
            tokens,
            position: 0,
            current_token,
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
        self.current_token = if self.position < self.tokens.len() {
            Some(self.tokens[self.position].clone())
        } else {
            None
        };
    }
    
    fn peek(&self, offset: usize) -> Option<&Token> {
        let peek_pos = self.position + offset;
        if peek_pos < self.tokens.len() {
            Some(&self.tokens[peek_pos])
        } else {
            None
        }
    }
    
    fn expect(&mut self, token_type: TokenType) -> Result<Token> {
        if let Some(ref token) = self.current_token {
            if token.token_type != token_type {
                return Err(KyaroError::parser_error(
                    format!("Expected {:?}, got {:?}", token_type, token.token_type),
                    token.line,
                    token.column,
                ));
            }
            let token = token.clone();
            self.advance();
            Ok(token)
        } else {
            Err(KyaroError::parser_error(
                format!("Expected {:?}, got EOF", token_type),
                0,
                0,
            ))
        }
    }
    
    fn skip_newlines(&mut self) {
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Newline {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    pub fn parse(&mut self) -> Result<ASTNode> {
        let mut statements = Vec::new();
        self.skip_newlines();
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Eof {
                break;
            }
            
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.skip_newlines();
        }
        
        Ok(ASTNode::Block(BlockNode::new(statements)))
    }
    
    fn parse_statement(&mut self) -> Result<ASTNode> {
        self.skip_newlines();
        
        if let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::KeywordLet => self.parse_let_statement(),
                TokenType::KeywordFunc => self.parse_function(),
                TokenType::KeywordIf => self.parse_if_statement(),
                TokenType::KeywordWhile => self.parse_while_statement(),
                TokenType::KeywordFor => self.parse_for_statement(),
                TokenType::KeywordReturn => self.parse_return_statement(),
                TokenType::KeywordBreak => {
                    self.advance();
                    Ok(ASTNode::Break(BreakNode::new()))
                }
                TokenType::KeywordContinue => {
                    self.advance();
                    Ok(ASTNode::Continue(ContinueNode::new()))
                }
                _ => self.parse_expression_statement(),
            }
        } else {
            Err(KyaroError::parser_error("Unexpected end of input", 0, 0))
        }
    }
    
    fn parse_let_statement(&mut self) -> Result<ASTNode> {
        self.advance(); // consume 'let'
        let name_token = self.expect(TokenType::Identifier)?;
        let name = match name_token.value {
            TokenValue::Identifier(name) => name,
            _ => return Err(KyaroError::parser_error("Expected identifier", name_token.line, name_token.column)),
        };
        
        self.expect(TokenType::Equals)?;
        let value = self.parse_expression()?;
        
        Ok(ASTNode::Assignment(AssignmentNode::new(name, value)))
    }
    
    fn parse_function(&mut self) -> Result<ASTNode> {
        self.advance(); // consume 'func'
        let name_token = self.expect(TokenType::Identifier)?;
        let name = match name_token.value {
            TokenValue::Identifier(name) => name,
            _ => return Err(KyaroError::parser_error("Expected identifier", name_token.line, name_token.column)),
        };
        
        self.expect(TokenType::LParen)?;
        
        let mut parameters = Vec::new();
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::RParen {
                break;
            }
            
            let param_token = self.expect(TokenType::Identifier)?;
            let param_name = match param_token.value {
                TokenValue::Identifier(name) => name,
                _ => return Err(KyaroError::parser_error("Expected identifier", param_token.line, param_token.column)),
            };
            parameters.push(param_name);
            
            if let Some(ref token) = self.current_token {
                if token.token_type == TokenType::Comma {
                    self.advance();
                }
            }
        }
        
        self.expect(TokenType::RParen)?;
        self.expect(TokenType::LBrace)?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::RBrace {
                break;
            }
            
            let stmt = self.parse_statement()?;
            body.push(stmt);
            self.skip_newlines();
        }
        
        self.expect(TokenType::RBrace)?;
        
        Ok(ASTNode::Function(FunctionNode::new(
            name,
            parameters,
            ASTNode::Block(BlockNode::new(body)),
        )))
    }
    
    fn parse_if_statement(&mut self) -> Result<ASTNode> {
        self.advance(); // consume 'if'
        let condition = self.parse_expression()?;
        self.expect(TokenType::LBrace)?;
        self.skip_newlines();
        
        let mut then_branch = Vec::new();
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::RBrace {
                break;
            }
            
            let stmt = self.parse_statement()?;
            then_branch.push(stmt);
            self.skip_newlines();
        }
        
        self.expect(TokenType::RBrace)?;
        self.skip_newlines();
        
        // For now, simplified - no elif/else support
        let elif_branches = Vec::new();
        let else_branch = None;
        
        Ok(ASTNode::If(IfNode::new(
            condition,
            ASTNode::Block(BlockNode::new(then_branch)),
            elif_branches,
            else_branch,
        )))
    }
    
    fn parse_while_statement(&mut self) -> Result<ASTNode> {
        self.advance(); // consume 'while'
        let condition = self.parse_expression()?;
        self.expect(TokenType::LBrace)?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::RBrace {
                break;
            }
            
            let stmt = self.parse_statement()?;
            body.push(stmt);
            self.skip_newlines();
        }
        
        self.expect(TokenType::RBrace)?;
        
        Ok(ASTNode::While(WhileNode::new(
            condition,
            ASTNode::Block(BlockNode::new(body)),
        )))
    }
    
    fn parse_for_statement(&mut self) -> Result<ASTNode> {
        self.advance(); // consume 'for'
        let var_token = self.expect(TokenType::Identifier)?;
        let variable = match var_token.value {
            TokenValue::Identifier(name) => name,
            _ => return Err(KyaroError::parser_error("Expected identifier", var_token.line, var_token.column)),
        };
        
        self.expect(TokenType::KeywordIn)?;
        let iterable = self.parse_expression()?;
        self.expect(TokenType::LBrace)?;
        self.skip_newlines();
        
        let mut body = Vec::new();
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::RBrace {
                break;
            }
            
            let stmt = self.parse_statement()?;
            body.push(stmt);
            self.skip_newlines();
        }
        
        self.expect(TokenType::RBrace)?;
        
        Ok(ASTNode::For(ForNode::new(
            variable,
            iterable,
            ASTNode::Block(BlockNode::new(body)),
        )))
    }
    
    fn parse_return_statement(&mut self) -> Result<ASTNode> {
        self.advance(); // consume 'return'
        
        // Check if there's an expression to return
        if let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Newline || token.token_type == TokenType::RBrace {
                return Ok(ASTNode::Return(ReturnNode::new(None)));
            }
        }
        
        let value = self.parse_expression()?;
        Ok(ASTNode::Return(ReturnNode::new(Some(value))))
    }
    
    fn parse_expression_statement(&mut self) -> Result<ASTNode> {
        self.parse_expression()
    }
    
    fn parse_expression(&mut self) -> Result<ASTNode> {
        self.parse_logical_or()
    }
    
    fn parse_logical_or(&mut self) -> Result<ASTNode> {
        let mut left = self.parse_logical_and()?;
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::KeywordOr {
                let operator = token.token_type.clone();
                self.advance();
                let right = self.parse_logical_and()?;
                left = ASTNode::BinaryOp(BinaryOpNode::new(left, operator, right));
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_logical_and(&mut self) -> Result<ASTNode> {
        let mut left = self.parse_equality()?;
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::KeywordAnd {
                let operator = token.token_type.clone();
                self.advance();
                let right = self.parse_equality()?;
                left = ASTNode::BinaryOp(BinaryOpNode::new(left, operator, right));
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_equality(&mut self) -> Result<ASTNode> {
        let mut left = self.parse_comparison()?;
        
        while let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::EqualsEquals | TokenType::NotEquals => {
                    let operator = token.token_type.clone();
                    self.advance();
                    let right = self.parse_comparison()?;
                    left = ASTNode::BinaryOp(BinaryOpNode::new(left, operator, right));
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<ASTNode> {
        let mut left = self.parse_term()?;
        
        while let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::LessThan | TokenType::GreaterThan | 
                TokenType::LessEquals | TokenType::GreaterEquals => {
                    let operator = token.token_type.clone();
                    self.advance();
                    let right = self.parse_term()?;
                    left = ASTNode::BinaryOp(BinaryOpNode::new(left, operator, right));
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_term(&mut self) -> Result<ASTNode> {
        let mut left = self.parse_factor()?;
        
        while let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::Plus | TokenType::Minus => {
                    let operator = token.token_type.clone();
                    self.advance();
                    let right = self.parse_factor()?;
                    left = ASTNode::BinaryOp(BinaryOpNode::new(left, operator, right));
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_factor(&mut self) -> Result<ASTNode> {
        let mut left = self.parse_unary()?;
        
        while let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::Star | TokenType::Slash | TokenType::Percent => {
                    let operator = token.token_type.clone();
                    self.advance();
                    let right = self.parse_unary()?;
                    left = ASTNode::BinaryOp(BinaryOpNode::new(left, operator, right));
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<ASTNode> {
        if let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::Minus | TokenType::KeywordNot => {
                    let operator = token.token_type.clone();
                    self.advance();
                    let operand = self.parse_unary()?;
                    return Ok(ASTNode::UnaryOp(UnaryOpNode::new(operator, operand)));
                }
                _ => {}
            }
        }
        
        self.parse_call()
    }
    
    fn parse_call(&mut self) -> Result<ASTNode> {
        let mut expr = self.parse_primary()?;
        
        while let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::LParen => {
                    self.advance();
                    let mut arguments = Vec::new();
                    
                    while let Some(ref token) = self.current_token {
                        if token.token_type == TokenType::RParen {
                            break;
                        }
                        
                        arguments.push(self.parse_expression()?);
                        
                        if let Some(ref token) = self.current_token {
                            if token.token_type == TokenType::Comma {
                                self.advance();
                            }
                        }
                    }
                    
                    self.expect(TokenType::RParen)?;
                    expr = ASTNode::Call(CallNode::new(expr, arguments));
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<ASTNode> {
        if let Some(ref token) = self.current_token {
            match &token.token_type {
                TokenType::Number => {
                    if let TokenValue::Number(value) = &token.value {
                        let value = *value;
                        self.advance();
                        Ok(ASTNode::Number(NumberNode::new(value)))
                    } else {
                        Err(KyaroError::parser_error("Invalid number token", token.line, token.column))
                    }
                }
                TokenType::String => {
                    if let TokenValue::String(value) = &token.value {
                        let value = value.clone();
                        self.advance();
                        Ok(ASTNode::String(StringNode::new(value)))
                    } else {
                        Err(KyaroError::parser_error("Invalid string token", token.line, token.column))
                    }
                }
                TokenType::KeywordTrue => {
                    self.advance();
                    Ok(ASTNode::Boolean(BooleanNode::new(true)))
                }
                TokenType::KeywordFalse => {
                    self.advance();
                    Ok(ASTNode::Boolean(BooleanNode::new(false)))
                }
                TokenType::KeywordNull => {
                    self.advance();
                    Ok(ASTNode::Null(NullNode::new()))
                }
                TokenType::Identifier => {
                    if let TokenValue::Identifier(name) = &token.value {
                        let name = name.clone();
                        self.advance();
                        Ok(ASTNode::Identifier(IdentifierNode::new(name)))
                    } else {
                        Err(KyaroError::parser_error("Invalid identifier token", token.line, token.column))
                    }
                }
                TokenType::LParen => {
                    self.advance();
                    let expr = self.parse_expression()?;
                    self.expect(TokenType::RParen)?;
                    Ok(expr)
                }
                TokenType::LBracket => {
                    self.advance();
                    let mut elements = Vec::new();
                    
                    while let Some(ref token) = self.current_token {
                        if token.token_type == TokenType::RBracket {
                            break;
                        }
                        
                        elements.push(self.parse_expression()?);
                        
                        if let Some(ref token) = self.current_token {
                            if token.token_type == TokenType::Comma {
                                self.advance();
                            }
                        }
                    }
                    
                    self.expect(TokenType::RBracket)?;
                    Ok(ASTNode::List(ListNode::new(elements)))
                }
                _ => Err(KyaroError::parser_error(
                    format!("Unexpected token: {:?}", token.token_type),
                    token.line,
                    token.column,
                )),
            }
        } else {
            Err(KyaroError::parser_error("Unexpected end of input", 0, 0))
        }
    }
}
