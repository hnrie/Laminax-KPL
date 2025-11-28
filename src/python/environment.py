class Environment:
    def __init__(self, parent=None):
        self.parent = parent
        self.variables = {}
    
    def define(self, name, value):
        self.variables[name] = value
    
    def get(self, name):
        if name in self.variables:
            return self.variables[name]
        if self.parent:
            return self.parent.get(name)
        raise NameError(f"Undefined variable: {name}")
    
    def set(self, name, value):
        if name in self.variables:
            self.variables[name] = value
            return
        if self.parent:
            self.parent.set(name, value)
            return
        raise NameError(f"Undefined variable: {name}")
    
    def exists(self, name):
        if name in self.variables:
            return True
        if self.parent:
            return self.parent.exists(name)
        return False
