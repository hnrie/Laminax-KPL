class ASTNode:
    pass

class NumberNode(ASTNode):
    def __init__(self, value):
        self.value = value
    
    def __repr__(self):
        return f'NumberNode({self.value})'

class StringNode(ASTNode):
    def __init__(self, value):
        self.value = value
    
    def __repr__(self):
        return f'StringNode({self.value!r})'

class BooleanNode(ASTNode):
    def __init__(self, value):
        self.value = value
    
    def __repr__(self):
        return f'BooleanNode({self.value})'

class NullNode(ASTNode):
    def __repr__(self):
        return 'NullNode()'

class IdentifierNode(ASTNode):
    def __init__(self, name):
        self.name = name
    
    def __repr__(self):
        return f'IdentifierNode({self.name})'

class BinaryOpNode(ASTNode):
    def __init__(self, left, operator, right):
        self.left = left
        self.operator = operator
        self.right = right
    
    def __repr__(self):
        return f'BinaryOpNode({self.left}, {self.operator}, {self.right})'

class UnaryOpNode(ASTNode):
    def __init__(self, operator, operand):
        self.operator = operator
        self.operand = operand
    
    def __repr__(self):
        return f'UnaryOpNode({self.operator}, {self.operand})'

class AssignmentNode(ASTNode):
    def __init__(self, name, value):
        self.name = name
        self.value = value
    
    def __repr__(self):
        return f'AssignmentNode({self.name}, {self.value})'

class CompoundAssignmentNode(ASTNode):
    def __init__(self, name, operator, value):
        self.name = name
        self.operator = operator
        self.value = value
    
    def __repr__(self):
        return f'CompoundAssignmentNode({self.name}, {self.operator}, {self.value})'

class CallNode(ASTNode):
    def __init__(self, callee, arguments):
        self.callee = callee
        self.arguments = arguments
    
    def __repr__(self):
        return f'CallNode({self.callee}, {self.arguments})'

class FunctionNode(ASTNode):
    def __init__(self, name, parameters, body):
        self.name = name
        self.parameters = parameters
        self.body = body
    
    def __repr__(self):
        return f'FunctionNode({self.name}, {self.parameters}, ...)'

class ReturnNode(ASTNode):
    def __init__(self, value):
        self.value = value
    
    def __repr__(self):
        return f'ReturnNode({self.value})'

class IfNode(ASTNode):
    def __init__(self, condition, then_branch, elif_branches, else_branch):
        self.condition = condition
        self.then_branch = then_branch
        self.elif_branches = elif_branches
        self.else_branch = else_branch
    
    def __repr__(self):
        return f'IfNode({self.condition}, ...)'

class WhileNode(ASTNode):
    def __init__(self, condition, body):
        self.condition = condition
        self.body = body
    
    def __repr__(self):
        return f'WhileNode({self.condition}, ...)'

class ForNode(ASTNode):
    def __init__(self, variable, iterable, body):
        self.variable = variable
        self.iterable = iterable
        self.body = body
    
    def __repr__(self):
        return f'ForNode({self.variable}, {self.iterable}, ...)'

class BreakNode(ASTNode):
    def __repr__(self):
        return 'BreakNode()'

class ContinueNode(ASTNode):
    def __repr__(self):
        return 'ContinueNode()'

class BlockNode(ASTNode):
    def __init__(self, statements):
        self.statements = statements
    
    def __repr__(self):
        return f'BlockNode({len(self.statements)} statements)'

class ListNode(ASTNode):
    def __init__(self, elements):
        self.elements = elements
    
    def __repr__(self):
        return f'ListNode({len(self.elements)} elements)'

class IndexNode(ASTNode):
    def __init__(self, object, index):
        self.object = object
        self.index = index
    
    def __repr__(self):
        return f'IndexNode({self.object}, {self.index})'

class MemberAccessNode(ASTNode):
    def __init__(self, object, member):
        self.object = object
        self.member = member
    
    def __repr__(self):
        return f'MemberAccessNode({self.object}, {self.member})'
