#!/usr/bin/env python3
"""
Compatibility test suite for Laminax-KPL Python vs Rust implementations.
This script runs identical Kyaro programs through both interpreters and compares outputs.
"""

import subprocess
import sys
import os
from pathlib import Path

# Test programs to run
TEST_PROGRAMS = [
    {
        "name": "Basic arithmetic",
        "code": """
let x = 10
let y = 20
print(x + y)
print(x - y)
print(x * y)
print(x / y)
"""
    },
    {
        "name": "String operations",
        "code": """
let greeting = "Hello"
let name = "World"
let message = greeting + ", " + name + "!"
print(message)
"""
    },
    {
        "name": "Boolean logic",
        "code": """
let a = true
let b = false
print(a and b)
print(a or b)
print(not a)
print(not b)
"""
    },
    {
        "name": "Lists",
        "code": """
let numbers = [1, 2, 3, 4, 5]
print(numbers)
let mixed = [1, "hello", true, null]
print(mixed)
"""
    },
    {
        "name": "Variables and assignment",
        "code": """
let count = 0
print(count)
let count = count + 1
print(count)
let count = count * 2
print(count)
"""
    },
    {
        "name": "Comparison operators",
        "code": """
let x = 10
let y = 20
print(x < y)
print(x > y)
print(x == y)
print(x != y)
"""
    }
]

def run_python_interpreter(code):
    """Run code through the Python interpreter."""
    try:
        # Write code to temporary file
        with open("/tmp/test_kyaro.kyaro", "w") as f:
            f.write(code)
        
        # Run Python interpreter
        result = subprocess.run(
            ["python", "src/python/main.py", "/tmp/test_kyaro.kyaro"],
            capture_output=True,
            text=True,
            cwd=Path(__file__).parent.parent
        )
        
        if result.returncode != 0:
            return None, result.stderr
        
        return result.stdout.strip(), None
    except Exception as e:
        return None, str(e)

def run_rust_interpreter(code):
    """Run code through the Rust interpreter."""
    try:
        # Write code to temporary file
        with open("/tmp/test_kyaro.kyaro", "w") as f:
            f.write(code)
        
        # Run Rust interpreter
        result = subprocess.run(
            ["cargo", "run", "/tmp/test_kyaro.kyaro"],
            capture_output=True,
            text=True,
            cwd=Path(__file__).parent.parent / "src" / "rust",
            env={**os.environ, "PATH": f"{os.environ.get('HOME')}/.cargo/bin:{os.environ.get('PATH', '')}"}
        )
        
        if result.returncode != 0:
            return None, result.stderr
        
        return result.stdout.strip(), None
    except Exception as e:
        return None, str(e)

def main():
    """Run all compatibility tests."""
    print("🦀 Laminax-KPL Compatibility Test Suite")
    print("=" * 50)
    
    passed = 0
    failed = 0
    
    for i, test in enumerate(TEST_PROGRAMS, 1):
        print(f"\n{i}. Testing: {test['name']}")
        print("-" * 30)
        
        # Run Python version
        python_output, python_error = run_python_interpreter(test['code'])
        if python_error:
            print(f"❌ Python interpreter failed: {python_error}")
            failed += 1
            continue
        
        # Run Rust version
        rust_output, rust_error = run_rust_interpreter(test['code'])
        if rust_error:
            print(f"❌ Rust interpreter failed: {rust_error}")
            failed += 1
            continue
        
        # Compare outputs
        if python_output == rust_output:
            print(f"✅ PASS - Both interpreters produced identical output:")
            print(f"   Output: {repr(python_output)}")
            passed += 1
        else:
            print(f"❌ FAIL - Output mismatch:")
            print(f"   Python: {repr(python_output)}")
            print(f"   Rust:   {repr(rust_output)}")
            failed += 1
    
    # Summary
    print("\n" + "=" * 50)
    print(f"📊 Test Results: {passed} passed, {failed} failed")
    
    if failed == 0:
        print("🎉 All tests passed! Both implementations are compatible.")
        return 0
    else:
        print("⚠️  Some tests failed. Check the output above for details.")
        return 1

if __name__ == "__main__":
    sys.exit(main())
