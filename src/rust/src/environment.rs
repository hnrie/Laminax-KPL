use std::collections::HashMap;
use crate::errors::{KyaroError, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    List(Vec<Value>),
    Function {
        name: String,
        parameters: Vec<String>,
        body: crate::ast_nodes::ASTNode,
    },
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Null => "null",
            Value::List(_) => "list",
            Value::Function { .. } => "function",
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Function { .. } => true,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "True".to_string() } else { "False".to_string() },
            Value::Null => "None".to_string(),
            Value::List(l) => {
                let elements: Vec<String> = l.iter().map(|v| match v {
                    Value::String(s) => format!("'{}'", s),
                    _ => v.to_string()
                }).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Function { name, .. } => format!("<function {}>", name),
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    pub fn get(&self, name: &str) -> Result<Value> {
        if let Some(value) = self.variables.get(name) {
            Ok(value.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get(name)
        } else {
            Err(KyaroError::runtime_error(
                format!("Undefined variable '{}'", name),
                0,
                0,
            ))
        }
    }
    
    pub fn set(&mut self, name: &str, value: Value) -> Result<()> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(ref mut parent) = self.parent {
            parent.set(name, value)
        } else {
            Err(KyaroError::runtime_error(
                format!("Undefined variable '{}'", name),
                0,
                0,
            ))
        }
    }
}
