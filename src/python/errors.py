class KyaroError(Exception):
    def __init__(self, message, line=None, column=None):
        self.message = message
        self.line = line
        self.column = column
        super().__init__(self.format_error())
    
    def format_error(self):
        if self.line is not None and self.column is not None:
            return f"Error at line {self.line}, column {self.column}: {self.message}"
        return f"Error: {self.message}"

class LexerError(KyaroError):
    pass

class ParserError(KyaroError):
    pass

class RuntimeError(KyaroError):
    pass
