use crate::errors::{KyaroError, Result};
use crate::ast_nodes::*;
use crate::environment::{Environment, Value};
use crate::token_types::TokenType;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        
        // Add built-in functions
        env.define("print".to_string(), Value::Function {
            name: "print".to_string(),
            parameters: vec!["value".to_string()],
            body: ASTNode::Null(NullNode::new()), // Placeholder for built-in
        });
        
        Self {
            environment: env,
        }
    }
    
    pub fn interpret(&mut self, node: ASTNode) -> Result<Option<Value>> {
        self.evaluate(node)
    }
    
    fn evaluate(&mut self, node: ASTNode) -> Result<Option<Value>> {
        match node {
            ASTNode::Number(n) => Ok(Some(Value::Number(n.value))),
            ASTNode::String(s) => Ok(Some(Value::String(s.value))),
            ASTNode::Boolean(b) => Ok(Some(Value::Boolean(b.value))),
            ASTNode::Null(_) => Ok(Some(Value::Null)),
            
            ASTNode::Identifier(id) => {
                let value = self.environment.get(&id.name)?;
                Ok(Some(value))
            }
            
            ASTNode::BinaryOp(op) => self.evaluate_binary_op(op),
            ASTNode::UnaryOp(op) => self.evaluate_unary_op(op),
            ASTNode::Assignment(assign) => self.evaluate_assignment(assign),
            ASTNode::Call(call) => self.evaluate_call(call),
            ASTNode::Block(block) => self.evaluate_block(block),
            ASTNode::If(if_node) => self.evaluate_if(if_node),
            ASTNode::While(while_node) => self.evaluate_while(while_node),
            ASTNode::Function(func) => self.evaluate_function(func),
            ASTNode::Return(ret) => self.evaluate_return(ret),
            ASTNode::List(list) => self.evaluate_list(list),
            
            _ => Err(KyaroError::runtime_error("Unsupported AST node", 0, 0)),
        }
    }
    
    fn evaluate_binary_op(&mut self, op: BinaryOpNode) -> Result<Option<Value>> {
        let left = self.evaluate(*op.left)?.unwrap_or(Value::Null);
        let right = self.evaluate(*op.right)?.unwrap_or(Value::Null);
        
        match op.operator {
            TokenType::Plus => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Some(Value::Number(a + b))),
                (Value::String(a), Value::String(b)) => Ok(Some(Value::String(a + &b))),
                _ => Err(KyaroError::runtime_error("Invalid operands for +", 0, 0)),
            },
            TokenType::Minus => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Some(Value::Number(a - b))),
                _ => Err(KyaroError::runtime_error("Invalid operands for -", 0, 0)),
            },
            TokenType::Star => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Some(Value::Number(a * b))),
                _ => Err(KyaroError::runtime_error("Invalid operands for *", 0, 0)),
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if b == 0.0 {
                        Err(KyaroError::runtime_error("Division by zero", 0, 0))
                    } else {
                        Ok(Some(Value::Number(a / b)))
                    }
                },
                _ => Err(KyaroError::runtime_error("Invalid operands for /", 0, 0)),
            },
            TokenType::EqualsEquals => Ok(Some(Value::Boolean(self.values_equal(&left, &right)))),
            TokenType::NotEquals => Ok(Some(Value::Boolean(!self.values_equal(&left, &right)))),
            TokenType::LessThan => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Some(Value::Boolean(a < b))),
                _ => Err(KyaroError::runtime_error("Invalid operands for <", 0, 0)),
            },
            TokenType::GreaterThan => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Some(Value::Boolean(a > b))),
                _ => Err(KyaroError::runtime_error("Invalid operands for >", 0, 0)),
            },
            TokenType::KeywordAnd => Ok(Some(Value::Boolean(left.is_truthy() && right.is_truthy()))),
            TokenType::KeywordOr => Ok(Some(Value::Boolean(left.is_truthy() || right.is_truthy()))),
            _ => Err(KyaroError::runtime_error("Unsupported binary operator", 0, 0)),
        }
    }
    
    fn evaluate_unary_op(&mut self, op: UnaryOpNode) -> Result<Option<Value>> {
        let operand = self.evaluate(*op.operand)?.unwrap_or(Value::Null);
        
        match op.operator {
            TokenType::Minus => match operand {
                Value::Number(n) => Ok(Some(Value::Number(-n))),
                _ => Err(KyaroError::runtime_error("Invalid operand for unary -", 0, 0)),
            },
            TokenType::KeywordNot => Ok(Some(Value::Boolean(!operand.is_truthy()))),
            _ => Err(KyaroError::runtime_error("Unsupported unary operator", 0, 0)),
        }
    }
    
    fn evaluate_assignment(&mut self, assign: AssignmentNode) -> Result<Option<Value>> {
        let value = self.evaluate(*assign.value)?.unwrap_or(Value::Null);
        self.environment.define(assign.name, value.clone());
        Ok(Some(value))
    }
    
    fn evaluate_call(&mut self, call: CallNode) -> Result<Option<Value>> {
        let callee = self.evaluate(*call.callee)?.unwrap_or(Value::Null);
        
        match callee {
            Value::Function { name, parameters, body } => {
                if name == "print" {
                    // Built-in print function
                    if !call.arguments.is_empty() {
                        let arg = self.evaluate(call.arguments[0].clone())?.unwrap_or(Value::Null);
                        println!("{}", arg.to_string());
                    }
                    Ok(Some(Value::Null))
                } else {
                    // User-defined function
                    if call.arguments.len() != parameters.len() {
                        return Err(KyaroError::runtime_error(
                            format!("Expected {} arguments, got {}", parameters.len(), call.arguments.len()),
                            0,
                            0,
                        ));
                    }
                    
                    // Create new environment for function scope
                    let mut func_env = Environment::with_parent(self.environment.clone());
                    
                    // Bind parameters
                    for (param, arg) in parameters.iter().zip(call.arguments.iter()) {
                        let arg_value = self.evaluate(arg.clone())?.unwrap_or(Value::Null);
                        func_env.define(param.clone(), arg_value);
                    }
                    
                    // Execute function body
                    let old_env = std::mem::replace(&mut self.environment, func_env);
                    let result = self.evaluate(body);
                    self.environment = old_env;
                    
                    result
                }
            }
            _ => Err(KyaroError::runtime_error("Not a function", 0, 0)),
        }
    }
    
    fn evaluate_block(&mut self, block: BlockNode) -> Result<Option<Value>> {
        let mut last_value = None;
        
        for stmt in block.statements {
            last_value = self.evaluate(stmt)?;
        }
        
        Ok(last_value)
    }
    
    fn evaluate_if(&mut self, if_node: IfNode) -> Result<Option<Value>> {
        let condition = self.evaluate(*if_node.condition)?.unwrap_or(Value::Null);
        
        if condition.is_truthy() {
            self.evaluate(*if_node.then_branch)
        } else if let Some(else_branch) = if_node.else_branch {
            self.evaluate(*else_branch)
        } else {
            Ok(Some(Value::Null))
        }
    }
    
    fn evaluate_while(&mut self, while_node: WhileNode) -> Result<Option<Value>> {
        let mut last_value = None;
        
        loop {
            let condition = self.evaluate((*while_node.condition).clone())?.unwrap_or(Value::Null);
            if !condition.is_truthy() {
                break;
            }
            
            last_value = self.evaluate((*while_node.body).clone())?;
        }
        
        Ok(last_value)
    }
    
    fn evaluate_function(&mut self, func: FunctionNode) -> Result<Option<Value>> {
        let function_value = Value::Function {
            name: func.name.clone(),
            parameters: func.parameters,
            body: *func.body,
        };
        
        self.environment.define(func.name, function_value.clone());
        Ok(Some(function_value))
    }
    
    fn evaluate_return(&mut self, ret: ReturnNode) -> Result<Option<Value>> {
        if let Some(value) = ret.value {
            self.evaluate(*value)
        } else {
            Ok(Some(Value::Null))
        }
    }
    
    fn evaluate_list(&mut self, list: ListNode) -> Result<Option<Value>> {
        let mut values = Vec::new();
        
        for element in list.elements {
            let value = self.evaluate(element)?.unwrap_or(Value::Null);
            values.push(value);
        }
        
        Ok(Some(Value::List(values)))
    }
    
    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
