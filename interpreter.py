from ast_nodes import *
from environment import Environment
from kyaro_builtins import BUILTIN_FUNCTIONS
from image_functions import IMAGE_FUNCTIONS
from ai_ml_functions import AI_ML_FUNCTIONS
from file_system_functions import FILE_SYSTEM_FUNCTIONS
from token_types import TokenType
from errors import RuntimeError as KyaroRuntimeError

class BreakException(Exception):
    pass

class ContinueException(Exception):
    pass

class ReturnException(Exception):
    def __init__(self, value):
        self.value = value

class KyaroFunction:
    def __init__(self, name, parameters, body, closure):
        self.name = name
        self.parameters = parameters
        self.body = body
        self.closure = closure
    
    def __repr__(self):
        return f'<function {self.name}>'

class Interpreter:
    def __init__(self):
        self.global_env = Environment()
        self.current_env = self.global_env
        
        for name, func in BUILTIN_FUNCTIONS.items():
            self.global_env.define(name, func)
        
        for name, func in IMAGE_FUNCTIONS.items():
            self.global_env.define(name, func)
        
        for name, func in AI_ML_FUNCTIONS.items():
            self.global_env.define(name, func)
        
        for name, func in FILE_SYSTEM_FUNCTIONS.items():
            self.global_env.define(name, func)
    
    def interpret(self, node):
        return self.evaluate(node)
    
    def evaluate(self, node):
        if isinstance(node, NumberNode):
            return node.value
        
        elif isinstance(node, StringNode):
            return node.value
        
        elif isinstance(node, BooleanNode):
            return node.value
        
        elif isinstance(node, NullNode):
            return None
        
        elif isinstance(node, IdentifierNode):
            return self.current_env.get(node.name)
        
        elif isinstance(node, BinaryOpNode):
            return self.evaluate_binary_op(node)
        
        elif isinstance(node, UnaryOpNode):
            return self.evaluate_unary_op(node)
        
        elif isinstance(node, AssignmentNode):
            value = self.evaluate(node.value)
            if self.current_env.exists(node.name):
                self.current_env.set(node.name, value)
            else:
                self.current_env.define(node.name, value)
            return value
        
        elif isinstance(node, CompoundAssignmentNode):
            current_value = self.current_env.get(node.name)
            new_value_part = self.evaluate(node.value)
            
            if node.operator == TokenType.PLUS_EQUALS:
                new_value = current_value + new_value_part
            elif node.operator == TokenType.MINUS_EQUALS:
                new_value = current_value - new_value_part
            elif node.operator == TokenType.STAR_EQUALS:
                new_value = current_value * new_value_part
            elif node.operator == TokenType.SLASH_EQUALS:
                new_value = current_value / new_value_part
            
            self.current_env.set(node.name, new_value)
            return new_value
        
        elif isinstance(node, CallNode):
            return self.evaluate_call(node)
        
        elif isinstance(node, FunctionNode):
            func = KyaroFunction(node.name, node.parameters, node.body, self.current_env)
            self.current_env.define(node.name, func)
            return func
        
        elif isinstance(node, ReturnNode):
            value = self.evaluate(node.value) if node.value else None
            raise ReturnException(value)
        
        elif isinstance(node, IfNode):
            return self.evaluate_if(node)
        
        elif isinstance(node, WhileNode):
            return self.evaluate_while(node)
        
        elif isinstance(node, ForNode):
            return self.evaluate_for(node)
        
        elif isinstance(node, BreakNode):
            raise BreakException()
        
        elif isinstance(node, ContinueNode):
            raise ContinueException()
        
        elif isinstance(node, BlockNode):
            result = None
            for statement in node.statements:
                result = self.evaluate(statement)
            return result
        
        elif isinstance(node, ListNode):
            return [self.evaluate(element) for element in node.elements]
        
        elif isinstance(node, IndexNode):
            obj = self.evaluate(node.object)
            index = self.evaluate(node.index)
            
            if isinstance(obj, (list, str)):
                return obj[int(index)]
            raise KyaroRuntimeError(f"Cannot index type {type(obj).__name__}")
        
        elif isinstance(node, MemberAccessNode):
            obj = self.evaluate(node.object)
            
            if isinstance(obj, str):
                if node.member == 'upper':
                    return lambda: obj.upper()
                elif node.member == 'lower':
                    return lambda: obj.lower()
                elif node.member == 'split':
                    return lambda sep=' ': obj.split(sep)
            
            raise KyaroRuntimeError(f"Type {type(obj).__name__} has no member '{node.member}'")
        
        else:
            raise KyaroRuntimeError(f"Unknown node type: {type(node).__name__}")
    
    def evaluate_binary_op(self, node):
        left = self.evaluate(node.left)
        right = self.evaluate(node.right)
        
        if node.operator == TokenType.PLUS:
            return left + right
        elif node.operator == TokenType.MINUS:
            return left - right
        elif node.operator == TokenType.STAR:
            return left * right
        elif node.operator == TokenType.SLASH:
            if right == 0:
                raise KyaroRuntimeError("Division by zero")
            return left / right
        elif node.operator == TokenType.PERCENT:
            return left % right
        elif node.operator == TokenType.POWER:
            return left ** right
        elif node.operator == TokenType.EQUALS_EQUALS:
            return left == right
        elif node.operator == TokenType.NOT_EQUALS:
            return left != right
        elif node.operator == TokenType.LESS_THAN:
            return left < right
        elif node.operator == TokenType.GREATER_THAN:
            return left > right
        elif node.operator == TokenType.LESS_EQUALS:
            return left <= right
        elif node.operator == TokenType.GREATER_EQUALS:
            return left >= right
        elif node.operator == TokenType.KEYWORD_AND:
            return self.is_truthy(left) and self.is_truthy(right)
        elif node.operator == TokenType.KEYWORD_OR:
            return left if self.is_truthy(left) else right
        else:
            raise KyaroRuntimeError(f"Unknown binary operator: {node.operator}")
    
    def evaluate_unary_op(self, node):
        operand = self.evaluate(node.operand)
        
        if node.operator == TokenType.MINUS:
            return -operand
        elif node.operator == TokenType.KEYWORD_NOT:
            return not self.is_truthy(operand)
        else:
            raise KyaroRuntimeError(f"Unknown unary operator: {node.operator}")
    
    def evaluate_call(self, node):
        callee = self.evaluate(node.callee)
        arguments = [self.evaluate(arg) for arg in node.arguments]
        
        if isinstance(callee, KyaroFunction):
            if len(arguments) != len(callee.parameters):
                raise KyaroRuntimeError(
                    f"Function {callee.name} expects {len(callee.parameters)} arguments, got {len(arguments)}"
                )
            
            func_env = Environment(callee.closure)
            for param, arg in zip(callee.parameters, arguments):
                func_env.define(param, arg)
            
            prev_env = self.current_env
            self.current_env = func_env
            
            try:
                self.evaluate(callee.body)
                return None
            except ReturnException as e:
                return e.value
            finally:
                self.current_env = prev_env
        elif callable(callee):
            return callee(*arguments)
        else:
            raise KyaroRuntimeError(f"Cannot call non-function type: {type(callee).__name__}")
    
    def evaluate_if(self, node):
        condition = self.evaluate(node.condition)
        
        if self.is_truthy(condition):
            return self.evaluate(node.then_branch)
        
        for elif_condition, elif_body in node.elif_branches:
            if self.is_truthy(self.evaluate(elif_condition)):
                return self.evaluate(elif_body)
        
        if node.else_branch:
            return self.evaluate(node.else_branch)
        
        return None
    
    def evaluate_while(self, node):
        result = None
        try:
            while self.is_truthy(self.evaluate(node.condition)):
                try:
                    result = self.evaluate(node.body)
                except ContinueException:
                    continue
        except BreakException:
            pass
        
        return result
    
    def evaluate_for(self, node):
        iterable = self.evaluate(node.iterable)
        
        if not isinstance(iterable, (list, str)):
            raise KyaroRuntimeError("For loop requires an iterable (list or string)")
        
        result = None
        loop_env = Environment(self.current_env)
        prev_env = self.current_env
        self.current_env = loop_env
        
        try:
            for item in iterable:
                loop_env.define(node.variable, item)
                try:
                    result = self.evaluate(node.body)
                except ContinueException:
                    continue
        except BreakException:
            pass
        finally:
            self.current_env = prev_env
        
        return result
    
    def is_truthy(self, value):
        if value is None:
            return False
        if isinstance(value, bool):
            return value
        if isinstance(value, (int, float)):
            return value != 0
        if isinstance(value, str):
            return len(value) > 0
        return True
