use crate::token_types::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Number(NumberNode),
    String(StringNode),
    Boolean(BooleanNode),
    Null(NullNode),
    Identifier(IdentifierNode),
    BinaryOp(BinaryOpNode),
    UnaryOp(UnaryOpNode),
    Assignment(AssignmentNode),
    CompoundAssignment(CompoundAssignmentNode),
    Call(CallNode),
    Function(FunctionNode),
    Return(ReturnNode),
    If(IfNode),
    While(WhileNode),
    For(ForNode),
    Break(BreakNode),
    Continue(ContinueNode),
    Block(BlockNode),
    List(ListNode),
    Index(IndexNode),
    MemberAccess(MemberAccessNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberNode {
    pub value: f64,
}

impl NumberNode {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringNode {
    pub value: String,
}

impl StringNode {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanNode {
    pub value: bool,
}

impl BooleanNode {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullNode;

impl NullNode {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierNode {
    pub name: String,
}

impl IdentifierNode {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOpNode {
    pub left: Box<ASTNode>,
    pub operator: TokenType,
    pub right: Box<ASTNode>,
}

impl BinaryOpNode {
    pub fn new(left: ASTNode, operator: TokenType, right: ASTNode) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOpNode {
    pub operator: TokenType,
    pub operand: Box<ASTNode>,
}

impl UnaryOpNode {
    pub fn new(operator: TokenType, operand: ASTNode) -> Self {
        Self {
            operator,
            operand: Box::new(operand),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentNode {
    pub name: String,
    pub value: Box<ASTNode>,
}

impl AssignmentNode {
    pub fn new(name: String, value: ASTNode) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundAssignmentNode {
    pub name: String,
    pub operator: TokenType,
    pub value: Box<ASTNode>,
}

impl CompoundAssignmentNode {
    pub fn new(name: String, operator: TokenType, value: ASTNode) -> Self {
        Self {
            name,
            operator,
            value: Box::new(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallNode {
    pub callee: Box<ASTNode>,
    pub arguments: Vec<ASTNode>,
}

impl CallNode {
    pub fn new(callee: ASTNode, arguments: Vec<ASTNode>) -> Self {
        Self {
            callee: Box::new(callee),
            arguments,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionNode {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<ASTNode>,
}

impl FunctionNode {
    pub fn new(name: String, parameters: Vec<String>, body: ASTNode) -> Self {
        Self {
            name,
            parameters,
            body: Box::new(body),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnNode {
    pub value: Option<Box<ASTNode>>,
}

impl ReturnNode {
    pub fn new(value: Option<ASTNode>) -> Self {
        Self {
            value: value.map(Box::new),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfNode {
    pub condition: Box<ASTNode>,
    pub then_branch: Box<ASTNode>,
    pub elif_branches: Vec<(ASTNode, ASTNode)>, // (condition, body) pairs
    pub else_branch: Option<Box<ASTNode>>,
}

impl IfNode {
    pub fn new(
        condition: ASTNode,
        then_branch: ASTNode,
        elif_branches: Vec<(ASTNode, ASTNode)>,
        else_branch: Option<ASTNode>,
    ) -> Self {
        Self {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            elif_branches,
            else_branch: else_branch.map(Box::new),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileNode {
    pub condition: Box<ASTNode>,
    pub body: Box<ASTNode>,
}

impl WhileNode {
    pub fn new(condition: ASTNode, body: ASTNode) -> Self {
        Self {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForNode {
    pub variable: String,
    pub iterable: Box<ASTNode>,
    pub body: Box<ASTNode>,
}

impl ForNode {
    pub fn new(variable: String, iterable: ASTNode, body: ASTNode) -> Self {
        Self {
            variable,
            iterable: Box::new(iterable),
            body: Box::new(body),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakNode;

impl BreakNode {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueNode;

impl ContinueNode {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockNode {
    pub statements: Vec<ASTNode>,
}

impl BlockNode {
    pub fn new(statements: Vec<ASTNode>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListNode {
    pub elements: Vec<ASTNode>,
}

impl ListNode {
    pub fn new(elements: Vec<ASTNode>) -> Self {
        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexNode {
    pub object: Box<ASTNode>,
    pub index: Box<ASTNode>,
}

impl IndexNode {
    pub fn new(object: ASTNode, index: ASTNode) -> Self {
        Self {
            object: Box::new(object),
            index: Box::new(index),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberAccessNode {
    pub object: Box<ASTNode>,
    pub member: String,
}

impl MemberAccessNode {
    pub fn new(object: ASTNode, member: String) -> Self {
        Self {
            object: Box::new(object),
            member,
        }
    }
}
