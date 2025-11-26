import sys
from lexer import Lexer
from parser import Parser
from interpreter import Interpreter
from errors import KyaroError

def run_repl():
    print("Laminax Kyaro Programming Language v1.0")
    print("By Laminax (https://laminax.org)")
    print("Type 'exit()' to quit\n")
    
    interpreter = Interpreter()
    
    while True:
        try:
            line = input("kyaro> ")
            
            if not line.strip():
                continue
            
            lexer = Lexer(line)
            tokens = lexer.tokenize()
            
            parser = Parser(tokens)
            ast = parser.parse()
            
            result = interpreter.interpret(ast)
            
            if result is not None:
                print(result)
        
        except EOFError:
            print("\nGoodbye!")
            break
        except KeyboardInterrupt:
            print("\nKeyboardInterrupt")
            continue
        except KyaroError as e:
            print(f"{e}")
        except Exception as e:
            print(f"Internal error: {e}")

def run_file(filename):
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            source = f.read()
        
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        
        parser = Parser(tokens)
        ast = parser.parse()
        
        interpreter = Interpreter()
        interpreter.interpret(ast)
    
    except FileNotFoundError:
        print(f"Error: File '{filename}' not found")
        sys.exit(1)
    except KyaroError as e:
        print(f"{e}")
        sys.exit(1)
    except Exception as e:
        print(f"Internal error: {e}")
        sys.exit(1)

def main():
    if len(sys.argv) == 1:
        run_repl()
    elif len(sys.argv) == 2:
        run_file(sys.argv[1])
    else:
        print("Usage: python main.py [filename.kyaro]")
        sys.exit(1)

if __name__ == '__main__':
    main()
