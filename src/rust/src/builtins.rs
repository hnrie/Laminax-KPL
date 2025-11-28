use crate::environment::Value;
use crate::errors::{KyaroError, Result};
use std::collections::HashMap;
use rand::prelude::*;
use rand_distr::*;
use statrs::statistics::*;
use libm::*;

pub type BuiltinFunction = fn(&[Value]) -> Result<Value>;

pub fn get_builtin_functions() -> HashMap<String, BuiltinFunction> {
    let mut functions = HashMap::new();
    
    // I/O functions
    functions.insert("print".to_string(), builtin_print as BuiltinFunction);
    functions.insert("input".to_string(), builtin_input as BuiltinFunction);
    
    // Type conversion functions
    functions.insert("str".to_string(), builtin_str as BuiltinFunction);
    functions.insert("int".to_string(), builtin_int as BuiltinFunction);
    functions.insert("float".to_string(), builtin_float as BuiltinFunction);
    functions.insert("type".to_string(), builtin_type as BuiltinFunction);
    
    // Collection functions
    functions.insert("len".to_string(), builtin_len as BuiltinFunction);
    functions.insert("range".to_string(), builtin_range as BuiltinFunction);
    functions.insert("append".to_string(), builtin_append as BuiltinFunction);
    functions.insert("pop".to_string(), builtin_pop as BuiltinFunction);
    functions.insert("push".to_string(), builtin_push as BuiltinFunction);
    
    // Math functions
    functions.insert("abs".to_string(), builtin_abs as BuiltinFunction);
    functions.insert("min".to_string(), builtin_min as BuiltinFunction);
    functions.insert("max".to_string(), builtin_max as BuiltinFunction);
    functions.insert("sum".to_string(), builtin_sum as BuiltinFunction);
    functions.insert("sqrt".to_string(), builtin_sqrt as BuiltinFunction);
    functions.insert("pow".to_string(), builtin_pow as BuiltinFunction);
    functions.insert("exp".to_string(), builtin_exp as BuiltinFunction);
    
    // Logarithmic functions
    functions.insert("log".to_string(), builtin_log as BuiltinFunction);
    functions.insert("log10".to_string(), builtin_log10 as BuiltinFunction);
    functions.insert("log2".to_string(), builtin_log2 as BuiltinFunction);
    functions.insert("ln".to_string(), builtin_ln as BuiltinFunction);
    
    // Rounding functions
    functions.insert("floor".to_string(), builtin_floor as BuiltinFunction);
    functions.insert("ceil".to_string(), builtin_ceil as BuiltinFunction);
    functions.insert("round".to_string(), builtin_round as BuiltinFunction);
    functions.insert("trunc".to_string(), builtin_trunc as BuiltinFunction);
    
    // Trigonometric functions
    functions.insert("sin".to_string(), builtin_sin as BuiltinFunction);
    functions.insert("cos".to_string(), builtin_cos as BuiltinFunction);
    functions.insert("tan".to_string(), builtin_tan as BuiltinFunction);
    functions.insert("asin".to_string(), builtin_asin as BuiltinFunction);
    functions.insert("acos".to_string(), builtin_acos as BuiltinFunction);
    functions.insert("atan".to_string(), builtin_atan as BuiltinFunction);
    functions.insert("atan2".to_string(), builtin_atan2 as BuiltinFunction);
    
    // Hyperbolic functions
    functions.insert("sinh".to_string(), builtin_sinh as BuiltinFunction);
    functions.insert("cosh".to_string(), builtin_cosh as BuiltinFunction);
    functions.insert("tanh".to_string(), builtin_tanh as BuiltinFunction);
    functions.insert("asinh".to_string(), builtin_asinh as BuiltinFunction);
    functions.insert("acosh".to_string(), builtin_acosh as BuiltinFunction);
    functions.insert("atanh".to_string(), builtin_atanh as BuiltinFunction);
    
    // Angle conversion
    functions.insert("degrees".to_string(), builtin_degrees as BuiltinFunction);
    functions.insert("radians".to_string(), builtin_radians as BuiltinFunction);
    
    // Advanced math
    functions.insert("hypot".to_string(), builtin_hypot as BuiltinFunction);
    functions.insert("factorial".to_string(), builtin_factorial as BuiltinFunction);
    functions.insert("gcd".to_string(), builtin_gcd as BuiltinFunction);
    
    // Random functions
    functions.insert("random".to_string(), builtin_random as BuiltinFunction);
    functions.insert("randint".to_string(), builtin_randint as BuiltinFunction);
    functions.insert("uniform".to_string(), builtin_uniform as BuiltinFunction);
    functions.insert("choice".to_string(), builtin_choice as BuiltinFunction);
    
    // Statistical functions
    functions.insert("mean".to_string(), builtin_mean as BuiltinFunction);
    functions.insert("median".to_string(), builtin_median as BuiltinFunction);
    functions.insert("stdev".to_string(), builtin_stdev as BuiltinFunction);
    functions.insert("variance".to_string(), builtin_variance as BuiltinFunction);
    
    // Constants
    functions.insert("pi".to_string(), builtin_pi as BuiltinFunction);
    functions.insert("e".to_string(), builtin_e as BuiltinFunction);
    
    // Utility functions
    functions.insert("exit".to_string(), builtin_exit as BuiltinFunction);
    functions.insert("time".to_string(), builtin_time as BuiltinFunction);
    functions.insert("sleep".to_string(), builtin_sleep as BuiltinFunction);
    
    functions
}

// I/O Functions
fn builtin_print(args: &[Value]) -> Result<Value> {
    let output = if args.is_empty() {
        String::new()
    } else {
        args.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    };
    println!("{}", output);
    Ok(Value::Null)
}

fn builtin_input(args: &[Value]) -> Result<Value> {
    if !args.is_empty() {
        print!("{}", args[0].to_string());
    }
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| {
        KyaroError::runtime_error(format!("Input error: {}", e), 0, 0)
    })?;
    
    Ok(Value::String(input.trim().to_string()))
}

// Type conversion functions
fn builtin_str(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("str() takes exactly one argument".to_string(), 0, 0));
    }
    Ok(Value::String(args[0].to_string()))
}

fn builtin_int(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("int() takes exactly one argument".to_string(), 0, 0));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n as i64 as f64)),
        Value::String(s) => {
            s.parse::<i64>()
                .map(|i| Value::Number(i as f64))
                .map_err(|_| KyaroError::runtime_error("Invalid integer".to_string(), 0, 0))
        }
        _ => Err(KyaroError::runtime_error("Cannot convert to int".to_string(), 0, 0)),
    }
}

fn builtin_float(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("float() takes exactly one argument".to_string(), 0, 0));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::String(s) => {
            s.parse::<f64>()
                .map(Value::Number)
                .map_err(|_| KyaroError::runtime_error("Invalid float".to_string(), 0, 0))
        }
        _ => Err(KyaroError::runtime_error("Cannot convert to float".to_string(), 0, 0)),
    }
}

fn builtin_type(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("type() takes exactly one argument".to_string(), 0, 0));
    }
    
    let type_name = match &args[0] {
        Value::Null => "null",
        Value::Boolean(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::List(_) => "list",
        Value::Function { .. } => "function",
    };
    
    Ok(Value::String(type_name.to_string()))
}

// Collection functions
fn builtin_len(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("len() takes exactly one argument".to_string(), 0, 0));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        Value::List(l) => Ok(Value::Number(l.len() as f64)),
        _ => Err(KyaroError::runtime_error("len() not supported for this type".to_string(), 0, 0)),
    }
}

fn builtin_range(args: &[Value]) -> Result<Value> {
    let (start, end, step) = match args.len() {
        1 => {
            if let Value::Number(n) = &args[0] {
                (0.0, *n, 1.0)
            } else {
                return Err(KyaroError::runtime_error("range() argument must be a number".to_string(), 0, 0));
            }
        }
        2 => {
            if let (Value::Number(start), Value::Number(end)) = (&args[0], &args[1]) {
                (*start, *end, 1.0)
            } else {
                return Err(KyaroError::runtime_error("range() arguments must be numbers".to_string(), 0, 0));
            }
        }
        3 => {
            if let (Value::Number(start), Value::Number(end), Value::Number(step)) = (&args[0], &args[1], &args[2]) {
                (*start, *end, *step)
            } else {
                return Err(KyaroError::runtime_error("range() arguments must be numbers".to_string(), 0, 0));
            }
        }
        _ => return Err(KyaroError::runtime_error("range() takes 1 to 3 arguments".to_string(), 0, 0)),
    };
    
    let mut result = Vec::new();
    let mut current = start;
    
    if step > 0.0 {
        while current < end {
            result.push(Value::Number(current));
            current += step;
        }
    } else if step < 0.0 {
        while current > end {
            result.push(Value::Number(current));
            current += step;
        }
    }
    
    Ok(Value::List(result))
}

fn builtin_append(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("append() takes exactly two arguments".to_string(), 0, 0));
    }
    
    // Note: This is a simplified implementation
    // In a real implementation, we'd need mutable references
    Err(KyaroError::runtime_error("append() requires mutable list reference".to_string(), 0, 0))
}

fn builtin_pop(args: &[Value]) -> Result<Value> {
    // Similar to append, requires mutable reference
    Err(KyaroError::runtime_error("pop() requires mutable list reference".to_string(), 0, 0))
}

fn builtin_push(args: &[Value]) -> Result<Value> {
    // Similar to append
    Err(KyaroError::runtime_error("push() requires mutable list reference".to_string(), 0, 0))
}

// Math functions
fn builtin_abs(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("abs() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.abs()))
    } else {
        Err(KyaroError::runtime_error("abs() requires a number".to_string(), 0, 0))
    }
}

fn builtin_min(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(KyaroError::runtime_error("min() requires at least one argument".to_string(), 0, 0));
    }
    
    if args.len() == 1 {
        if let Value::List(list) = &args[0] {
            if list.is_empty() {
                return Err(KyaroError::runtime_error("min() of empty list".to_string(), 0, 0));
            }
            
            let mut min_val = &list[0];
            for val in list.iter().skip(1) {
                if let (Value::Number(a), Value::Number(b)) = (val, min_val) {
                    if a < b {
                        min_val = val;
                    }
                }
            }
            return Ok(min_val.clone());
        }
    }
    
    let mut min_val = &args[0];
    for val in args.iter().skip(1) {
        if let (Value::Number(a), Value::Number(b)) = (val, min_val) {
            if a < b {
                min_val = val;
            }
        }
    }
    
    Ok(min_val.clone())
}

fn builtin_max(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(KyaroError::runtime_error("max() requires at least one argument".to_string(), 0, 0));
    }
    
    if args.len() == 1 {
        if let Value::List(list) = &args[0] {
            if list.is_empty() {
                return Err(KyaroError::runtime_error("max() of empty list".to_string(), 0, 0));
            }
            
            let mut max_val = &list[0];
            for val in list.iter().skip(1) {
                if let (Value::Number(a), Value::Number(b)) = (val, max_val) {
                    if a > b {
                        max_val = val;
                    }
                }
            }
            return Ok(max_val.clone());
        }
    }
    
    let mut max_val = &args[0];
    for val in args.iter().skip(1) {
        if let (Value::Number(a), Value::Number(b)) = (val, max_val) {
            if a > b {
                max_val = val;
            }
        }
    }
    
    Ok(max_val.clone())
}

fn builtin_sum(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("sum() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::List(list) = &args[0] {
        let mut total = 0.0;
        for val in list {
            if let Value::Number(n) = val {
                total += n;
            } else {
                return Err(KyaroError::runtime_error("sum() requires a list of numbers".to_string(), 0, 0));
            }
        }
        Ok(Value::Number(total))
    } else {
        Err(KyaroError::runtime_error("sum() requires a list".to_string(), 0, 0))
    }
}

fn builtin_sqrt(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("sqrt() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        if *n < 0.0 {
            return Err(KyaroError::runtime_error("sqrt() of negative number".to_string(), 0, 0));
        }
        Ok(Value::Number(n.sqrt()))
    } else {
        Err(KyaroError::runtime_error("sqrt() requires a number".to_string(), 0, 0))
    }
}

fn builtin_pow(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("pow() takes exactly two arguments".to_string(), 0, 0));
    }
    
    if let (Value::Number(base), Value::Number(exp)) = (&args[0], &args[1]) {
        Ok(Value::Number(base.powf(*exp)))
    } else {
        Err(KyaroError::runtime_error("pow() requires numbers".to_string(), 0, 0))
    }
}

fn builtin_exp(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("exp() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.exp()))
    } else {
        Err(KyaroError::runtime_error("exp() requires a number".to_string(), 0, 0))
    }
}

// Trigonometric functions
fn builtin_sin(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("sin() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.sin()))
    } else {
        Err(KyaroError::runtime_error("sin() requires a number".to_string(), 0, 0))
    }
}

fn builtin_cos(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("cos() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.cos()))
    } else {
        Err(KyaroError::runtime_error("cos() requires a number".to_string(), 0, 0))
    }
}

fn builtin_tan(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("tan() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.tan()))
    } else {
        Err(KyaroError::runtime_error("tan() requires a number".to_string(), 0, 0))
    }
}

// Utility functions
fn builtin_exit(args: &[Value]) -> Result<Value> {
    let code = if args.is_empty() {
        0
    } else if let Value::Number(n) = &args[0] {
        *n as i32
    } else {
        0
    };
    
    std::process::exit(code);
}

fn builtin_time(args: &[Value]) -> Result<Value> {
    if !args.is_empty() {
        return Err(KyaroError::runtime_error("time() takes no arguments".to_string(), 0, 0));
    }
    
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)
        .map_err(|_| KyaroError::runtime_error("Time error".to_string(), 0, 0))?;
    
    Ok(Value::Number(duration.as_secs_f64()))
}

fn builtin_sleep(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("sleep() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        std::thread::sleep(std::time::Duration::from_secs_f64(*n));
        Ok(Value::Null)
    } else {
        Err(KyaroError::runtime_error("sleep() requires a number".to_string(), 0, 0))
    }
}

// Logarithmic functions
fn builtin_log(args: &[Value]) -> Result<Value> {
    match args.len() {
        1 => {
            if let Value::Number(n) = &args[0] {
                if *n <= 0.0 {
                    return Err(KyaroError::runtime_error("log() of non-positive number".to_string(), 0, 0));
                }
                Ok(Value::Number(n.ln()))
            } else {
                Err(KyaroError::runtime_error("log() requires a number".to_string(), 0, 0))
            }
        }
        2 => {
            if let (Value::Number(n), Value::Number(base)) = (&args[0], &args[1]) {
                if *n <= 0.0 || *base <= 0.0 || *base == 1.0 {
                    return Err(KyaroError::runtime_error("Invalid log arguments".to_string(), 0, 0));
                }
                Ok(Value::Number(n.ln() / base.ln()))
            } else {
                Err(KyaroError::runtime_error("log() requires numbers".to_string(), 0, 0))
            }
        }
        _ => Err(KyaroError::runtime_error("log() takes 1 or 2 arguments".to_string(), 0, 0)),
    }
}

fn builtin_log10(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("log10() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        if *n <= 0.0 {
            return Err(KyaroError::runtime_error("log10() of non-positive number".to_string(), 0, 0));
        }
        Ok(Value::Number(n.log10()))
    } else {
        Err(KyaroError::runtime_error("log10() requires a number".to_string(), 0, 0))
    }
}

fn builtin_log2(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("log2() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        if *n <= 0.0 {
            return Err(KyaroError::runtime_error("log2() of non-positive number".to_string(), 0, 0));
        }
        Ok(Value::Number(n.log2()))
    } else {
        Err(KyaroError::runtime_error("log2() requires a number".to_string(), 0, 0))
    }
}

fn builtin_ln(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("ln() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        if *n <= 0.0 {
            return Err(KyaroError::runtime_error("ln() of non-positive number".to_string(), 0, 0));
        }
        Ok(Value::Number(n.ln()))
    } else {
        Err(KyaroError::runtime_error("ln() requires a number".to_string(), 0, 0))
    }
}

// Rounding functions
fn builtin_floor(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("floor() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.floor()))
    } else {
        Err(KyaroError::runtime_error("floor() requires a number".to_string(), 0, 0))
    }
}

fn builtin_ceil(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("ceil() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.ceil()))
    } else {
        Err(KyaroError::runtime_error("ceil() requires a number".to_string(), 0, 0))
    }
}

fn builtin_round(args: &[Value]) -> Result<Value> {
    match args.len() {
        1 => {
            if let Value::Number(n) = &args[0] {
                Ok(Value::Number(n.round()))
            } else {
                Err(KyaroError::runtime_error("round() requires a number".to_string(), 0, 0))
            }
        }
        2 => {
            if let (Value::Number(n), Value::Number(digits)) = (&args[0], &args[1]) {
                let factor = 10.0_f64.powf(*digits);
                Ok(Value::Number((n * factor).round() / factor))
            } else {
                Err(KyaroError::runtime_error("round() requires numbers".to_string(), 0, 0))
            }
        }
        _ => Err(KyaroError::runtime_error("round() takes 1 or 2 arguments".to_string(), 0, 0)),
    }
}

fn builtin_trunc(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("trunc() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.trunc()))
    } else {
        Err(KyaroError::runtime_error("trunc() requires a number".to_string(), 0, 0))
    }
}

// Additional trigonometric functions
fn builtin_asin(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("asin() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        if *n < -1.0 || *n > 1.0 {
            return Err(KyaroError::runtime_error("asin() domain error".to_string(), 0, 0));
        }
        Ok(Value::Number(n.asin()))
    } else {
        Err(KyaroError::runtime_error("asin() requires a number".to_string(), 0, 0))
    }
}

fn builtin_acos(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("acos() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        if *n < -1.0 || *n > 1.0 {
            return Err(KyaroError::runtime_error("acos() domain error".to_string(), 0, 0));
        }
        Ok(Value::Number(n.acos()))
    } else {
        Err(KyaroError::runtime_error("acos() requires a number".to_string(), 0, 0))
    }
}

fn builtin_atan(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("atan() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.atan()))
    } else {
        Err(KyaroError::runtime_error("atan() requires a number".to_string(), 0, 0))
    }
}

fn builtin_atan2(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("atan2() takes exactly two arguments".to_string(), 0, 0));
    }
    
    if let (Value::Number(y), Value::Number(x)) = (&args[0], &args[1]) {
        Ok(Value::Number(y.atan2(*x)))
    } else {
        Err(KyaroError::runtime_error("atan2() requires numbers".to_string(), 0, 0))
    }
}

// Hyperbolic functions
fn builtin_sinh(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("sinh() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.sinh()))
    } else {
        Err(KyaroError::runtime_error("sinh() requires a number".to_string(), 0, 0))
    }
}

fn builtin_cosh(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("cosh() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.cosh()))
    } else {
        Err(KyaroError::runtime_error("cosh() requires a number".to_string(), 0, 0))
    }
}

fn builtin_tanh(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("tanh() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.tanh()))
    } else {
        Err(KyaroError::runtime_error("tanh() requires a number".to_string(), 0, 0))
    }
}

fn builtin_asinh(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("asinh() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.asinh()))
    } else {
        Err(KyaroError::runtime_error("asinh() requires a number".to_string(), 0, 0))
    }
}

fn builtin_acosh(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("acosh() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        if *n < 1.0 {
            return Err(KyaroError::runtime_error("acosh() domain error".to_string(), 0, 0));
        }
        Ok(Value::Number(n.acosh()))
    } else {
        Err(KyaroError::runtime_error("acosh() requires a number".to_string(), 0, 0))
    }
}

fn builtin_atanh(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("atanh() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        if *n <= -1.0 || *n >= 1.0 {
            return Err(KyaroError::runtime_error("atanh() domain error".to_string(), 0, 0));
        }
        Ok(Value::Number(n.atanh()))
    } else {
        Err(KyaroError::runtime_error("atanh() requires a number".to_string(), 0, 0))
    }
}

// Angle conversion
fn builtin_degrees(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("degrees() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.to_degrees()))
    } else {
        Err(KyaroError::runtime_error("degrees() requires a number".to_string(), 0, 0))
    }
}

fn builtin_radians(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("radians() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        Ok(Value::Number(n.to_radians()))
    } else {
        Err(KyaroError::runtime_error("radians() requires a number".to_string(), 0, 0))
    }
}

// Advanced math functions
fn builtin_hypot(args: &[Value]) -> Result<Value> {
    if args.len() < 2 {
        return Err(KyaroError::runtime_error("hypot() requires at least 2 arguments".to_string(), 0, 0));
    }
    
    let mut sum_squares = 0.0;
    for arg in args {
        if let Value::Number(n) = arg {
            sum_squares += n * n;
        } else {
            return Err(KyaroError::runtime_error("hypot() requires numbers".to_string(), 0, 0));
        }
    }
    
    Ok(Value::Number(sum_squares.sqrt()))
}

fn builtin_factorial(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("factorial() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(n) = &args[0] {
        let n_int = *n as i64;
        if n_int < 0 || (*n - n_int as f64).abs() > f64::EPSILON {
            return Err(KyaroError::runtime_error("factorial() requires a non-negative integer".to_string(), 0, 0));
        }
        
        let mut result = 1.0;
        for i in 1..=n_int {
            result *= i as f64;
        }
        
        Ok(Value::Number(result))
    } else {
        Err(KyaroError::runtime_error("factorial() requires a number".to_string(), 0, 0))
    }
}

fn builtin_gcd(args: &[Value]) -> Result<Value> {
    if args.len() < 2 {
        return Err(KyaroError::runtime_error("gcd() requires at least 2 arguments".to_string(), 0, 0));
    }
    
    let mut result = if let Value::Number(n) = &args[0] {
        (*n as i64).abs()
    } else {
        return Err(KyaroError::runtime_error("gcd() requires integers".to_string(), 0, 0));
    };
    
    for arg in args.iter().skip(1) {
        if let Value::Number(n) = arg {
            let n_int = (*n as i64).abs();
            result = gcd_helper(result, n_int);
        } else {
            return Err(KyaroError::runtime_error("gcd() requires integers".to_string(), 0, 0));
        }
    }
    
    Ok(Value::Number(result as f64))
}

fn gcd_helper(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd_helper(b, a % b)
    }
}

// Random functions
fn builtin_random(args: &[Value]) -> Result<Value> {
    if !args.is_empty() {
        return Err(KyaroError::runtime_error("random() takes no arguments".to_string(), 0, 0));
    }
    
    let mut rng = thread_rng();
    Ok(Value::Number(rng.gen::<f64>()))
}

fn builtin_randint(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("randint() takes exactly two arguments".to_string(), 0, 0));
    }
    
    if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
        let min = (*a as i64).min(*b as i64);
        let max = (*a as i64).max(*b as i64);
        let mut rng = thread_rng();
        Ok(Value::Number(rng.gen_range(min..=max) as f64))
    } else {
        Err(KyaroError::runtime_error("randint() requires integers".to_string(), 0, 0))
    }
}

fn builtin_uniform(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("uniform() takes exactly two arguments".to_string(), 0, 0));
    }
    
    if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
        let mut rng = thread_rng();
        Ok(Value::Number(rng.gen_range(*a..*b)))
    } else {
        Err(KyaroError::runtime_error("uniform() requires numbers".to_string(), 0, 0))
    }
}

fn builtin_choice(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("choice() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::List(list) = &args[0] {
        if list.is_empty() {
            return Err(KyaroError::runtime_error("choice() from empty list".to_string(), 0, 0));
        }
        
        let mut rng = thread_rng();
        let index = rng.gen_range(0..list.len());
        Ok(list[index].clone())
    } else {
        Err(KyaroError::runtime_error("choice() requires a list".to_string(), 0, 0))
    }
}

// Statistical functions
fn builtin_mean(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("mean() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::List(list) = &args[0] {
        if list.is_empty() {
            return Err(KyaroError::runtime_error("mean() of empty list".to_string(), 0, 0));
        }
        
        let mut sum = 0.0;
        for val in list {
            if let Value::Number(n) = val {
                sum += n;
            } else {
                return Err(KyaroError::runtime_error("mean() requires a list of numbers".to_string(), 0, 0));
            }
        }
        
        Ok(Value::Number(sum / list.len() as f64))
    } else {
        Err(KyaroError::runtime_error("mean() requires a list".to_string(), 0, 0))
    }
}

fn builtin_median(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("median() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::List(list) = &args[0] {
        if list.is_empty() {
            return Err(KyaroError::runtime_error("median() of empty list".to_string(), 0, 0));
        }
        
        let mut numbers = Vec::new();
        for val in list {
            if let Value::Number(n) = val {
                numbers.push(*n);
            } else {
                return Err(KyaroError::runtime_error("median() requires a list of numbers".to_string(), 0, 0));
            }
        }
        
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = numbers.len();
        
        if len % 2 == 0 {
            Ok(Value::Number((numbers[len / 2 - 1] + numbers[len / 2]) / 2.0))
        } else {
            Ok(Value::Number(numbers[len / 2]))
        }
    } else {
        Err(KyaroError::runtime_error("median() requires a list".to_string(), 0, 0))
    }
}

fn builtin_stdev(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("stdev() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::List(list) = &args[0] {
        if list.len() < 2 {
            return Err(KyaroError::runtime_error("stdev() requires at least 2 values".to_string(), 0, 0));
        }
        
        let mut numbers = Vec::new();
        for val in list {
            if let Value::Number(n) = val {
                numbers.push(*n);
            } else {
                return Err(KyaroError::runtime_error("stdev() requires a list of numbers".to_string(), 0, 0));
            }
        }
        
        let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
        let variance = numbers.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (numbers.len() - 1) as f64;
        
        Ok(Value::Number(variance.sqrt()))
    } else {
        Err(KyaroError::runtime_error("stdev() requires a list".to_string(), 0, 0))
    }
}

fn builtin_variance(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("variance() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::List(list) = &args[0] {
        if list.len() < 2 {
            return Err(KyaroError::runtime_error("variance() requires at least 2 values".to_string(), 0, 0));
        }
        
        let mut numbers = Vec::new();
        for val in list {
            if let Value::Number(n) = val {
                numbers.push(*n);
            } else {
                return Err(KyaroError::runtime_error("variance() requires a list of numbers".to_string(), 0, 0));
            }
        }
        
        let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
        let variance = numbers.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (numbers.len() - 1) as f64;
        
        Ok(Value::Number(variance))
    } else {
        Err(KyaroError::runtime_error("variance() requires a list".to_string(), 0, 0))
    }
}

// Constants
fn builtin_pi(args: &[Value]) -> Result<Value> {
    if !args.is_empty() {
        return Err(KyaroError::runtime_error("pi() takes no arguments".to_string(), 0, 0));
    }
    Ok(Value::Number(std::f64::consts::PI))
}

fn builtin_e(args: &[Value]) -> Result<Value> {
    if !args.is_empty() {
        return Err(KyaroError::runtime_error("e() takes no arguments".to_string(), 0, 0));
    }
    Ok(Value::Number(std::f64::consts::E))
}
