use crate::environment::Value;
use crate::errors::{KyaroError, Result};
use std::collections::HashMap;
use rand::prelude::*;
use ndarray::prelude::*;

pub type AiMlFunction = fn(&[Value]) -> Result<Value>;

pub fn get_ai_ml_functions() -> HashMap<String, AiMlFunction> {
    let mut functions = HashMap::new();
    
    // Data preprocessing
    functions.insert("ml_train_test_split".to_string(), ml_train_test_split as AiMlFunction);
    functions.insert("ml_standardize".to_string(), ml_standardize as AiMlFunction);
    functions.insert("ml_min_max_scale".to_string(), ml_min_max_scale as AiMlFunction);
    functions.insert("ml_one_hot_encode".to_string(), ml_one_hot_encode as AiMlFunction);
    
    // Distance metrics
    functions.insert("ml_euclidean_distance".to_string(), ml_euclidean_distance as AiMlFunction);
    functions.insert("ml_manhattan_distance".to_string(), ml_manhattan_distance as AiMlFunction);
    functions.insert("ml_cosine_similarity".to_string(), ml_cosine_similarity as AiMlFunction);
    
    // Algorithms
    functions.insert("ml_knn_predict".to_string(), ml_knn_predict as AiMlFunction);
    functions.insert("ml_kmeans".to_string(), ml_kmeans as AiMlFunction);
    
    // Regression metrics
    functions.insert("ml_mse".to_string(), ml_mse as AiMlFunction);
    functions.insert("ml_mae".to_string(), ml_mae as AiMlFunction);
    functions.insert("ml_rmse".to_string(), ml_rmse as AiMlFunction);
    functions.insert("ml_r2_score".to_string(), ml_r2_score as AiMlFunction);
    
    // Classification metrics
    functions.insert("ml_accuracy".to_string(), ml_accuracy as AiMlFunction);
    functions.insert("ml_precision".to_string(), ml_precision as AiMlFunction);
    functions.insert("ml_recall".to_string(), ml_recall as AiMlFunction);
    functions.insert("ml_f1_score".to_string(), ml_f1_score as AiMlFunction);
    functions.insert("ml_confusion_matrix".to_string(), ml_confusion_matrix as AiMlFunction);
    
    // Neural network activations
    functions.insert("nn_tanh".to_string(), nn_tanh as AiMlFunction);
    functions.insert("nn_leaky_relu".to_string(), nn_leaky_relu as AiMlFunction);
    functions.insert("nn_elu".to_string(), nn_elu as AiMlFunction);
    functions.insert("nn_softplus".to_string(), nn_softplus as AiMlFunction);
    
    // Loss functions
    functions.insert("nn_mse_loss".to_string(), nn_mse_loss as AiMlFunction);
    functions.insert("nn_binary_crossentropy".to_string(), nn_binary_crossentropy as AiMlFunction);
    functions.insert("nn_categorical_crossentropy".to_string(), nn_categorical_crossentropy as AiMlFunction);
    
    // NN utilities
    functions.insert("nn_dropout".to_string(), nn_dropout as AiMlFunction);
    functions.insert("nn_batch_norm".to_string(), nn_batch_norm as AiMlFunction);
    
    // Matrix operations
    functions.insert("matrix_multiply".to_string(), matrix_multiply as AiMlFunction);
    functions.insert("matrix_transpose".to_string(), matrix_transpose as AiMlFunction);
    functions.insert("matrix_add".to_string(), matrix_add as AiMlFunction);
    functions.insert("matrix_subtract".to_string(), matrix_subtract as AiMlFunction);
    functions.insert("matrix_identity".to_string(), matrix_identity as AiMlFunction);
    functions.insert("matrix_determinant".to_string(), matrix_determinant as AiMlFunction);
    
    // Optimization
    functions.insert("gradient_descent_step".to_string(), gradient_descent_step as AiMlFunction);
    functions.insert("adam_step".to_string(), adam_step as AiMlFunction);
    
    functions
}

// Data preprocessing functions
fn ml_train_test_split(args: &[Value]) -> Result<Value> {
    if args.len() < 2 || args.len() > 4 {
        return Err(KyaroError::runtime_error("ml_train_test_split() takes 2-4 arguments".to_string(), 0, 0));
    }
    
    let (data, labels) = match (&args[0], &args[1]) {
        (Value::List(d), Value::List(l)) => (d, l),
        _ => return Err(KyaroError::runtime_error("ml_train_test_split() requires two lists".to_string(), 0, 0)),
    };
    
    if data.len() != labels.len() {
        return Err(KyaroError::runtime_error("Data and labels must have same length".to_string(), 0, 0));
    }
    
    let test_size = if args.len() > 2 {
        if let Value::Number(n) = &args[2] {
            *n
        } else {
            return Err(KyaroError::runtime_error("test_size must be a number".to_string(), 0, 0));
        }
    } else {
        0.2
    };
    
    let random_state = if args.len() > 3 {
        if let Value::Number(n) = &args[3] {
            Some(*n as u64)
        } else {
            None
        }
    } else {
        None
    };
    
    let mut rng = if let Some(seed) = random_state {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_entropy()
    };
    
    let mut indices: Vec<usize> = (0..data.len()).collect();
    indices.shuffle(&mut rng);
    
    let split_idx = ((data.len() as f64) * (1.0 - test_size)) as usize;
    let train_indices = &indices[..split_idx];
    let test_indices = &indices[split_idx..];
    
    let x_train: Vec<Value> = train_indices.iter().map(|&i| data[i].clone()).collect();
    let x_test: Vec<Value> = test_indices.iter().map(|&i| data[i].clone()).collect();
    let y_train: Vec<Value> = train_indices.iter().map(|&i| labels[i].clone()).collect();
    let y_test: Vec<Value> = test_indices.iter().map(|&i| labels[i].clone()).collect();
    
    Ok(Value::List(vec![
        Value::List(x_train),
        Value::List(x_test),
        Value::List(y_train),
        Value::List(y_test),
    ]))
}

fn ml_standardize(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("ml_standardize() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::List(data) = &args[0] {
        let mut numbers = Vec::new();
        for val in data {
            if let Value::Number(n) = val {
                numbers.push(*n);
            } else {
                return Err(KyaroError::runtime_error("ml_standardize() requires a list of numbers".to_string(), 0, 0));
            }
        }
        
        if numbers.is_empty() {
            return Ok(Value::List(vec![]));
        }
        
        let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
        let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;
        let std = variance.sqrt();
        
        if std == 0.0 {
            return Ok(Value::List(data.clone()));
        }
        
        let standardized: Vec<Value> = numbers.iter()
            .map(|x| Value::Number((x - mean) / std))
            .collect();
        
        Ok(Value::List(standardized))
    } else {
        Err(KyaroError::runtime_error("ml_standardize() requires a list".to_string(), 0, 0))
    }
}

fn ml_min_max_scale(args: &[Value]) -> Result<Value> {
    if args.len() < 1 || args.len() > 2 {
        return Err(KyaroError::runtime_error("ml_min_max_scale() takes 1-2 arguments".to_string(), 0, 0));
    }
    
    if let Value::List(data) = &args[0] {
        let mut numbers = Vec::new();
        for val in data {
            if let Value::Number(n) = val {
                numbers.push(*n);
            } else {
                return Err(KyaroError::runtime_error("ml_min_max_scale() requires a list of numbers".to_string(), 0, 0));
            }
        }
        
        if numbers.is_empty() {
            return Ok(Value::List(vec![]));
        }
        
        let feature_range = if args.len() > 1 {
            if let Value::List(range) = &args[1] {
                if range.len() != 2 {
                    return Err(KyaroError::runtime_error("feature_range must have 2 elements".to_string(), 0, 0));
                }
                if let (Value::Number(min), Value::Number(max)) = (&range[0], &range[1]) {
                    (*min, *max)
                } else {
                    return Err(KyaroError::runtime_error("feature_range must contain numbers".to_string(), 0, 0));
                }
            } else {
                return Err(KyaroError::runtime_error("feature_range must be a list".to_string(), 0, 0));
            }
        } else {
            (0.0, 1.0)
        };
        
        let min_val = numbers.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        if min_val == max_val {
            return Ok(Value::List(data.clone()));
        }
        
        let scaled: Vec<Value> = numbers.iter()
            .map(|x| {
                let normalized = (x - min_val) / (max_val - min_val);
                let scaled_val = normalized * (feature_range.1 - feature_range.0) + feature_range.0;
                Value::Number(scaled_val)
            })
            .collect();
        
        Ok(Value::List(scaled))
    } else {
        Err(KyaroError::runtime_error("ml_min_max_scale() requires a list".to_string(), 0, 0))
    }
}

fn ml_one_hot_encode(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("ml_one_hot_encode() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::List(data) = &args[0] {
        let mut unique_values = Vec::new();
        for val in data {
            if !unique_values.contains(val) {
                unique_values.push(val.clone());
            }
        }
        
        let mut encoded = Vec::new();
        for val in data {
            let mut one_hot = vec![Value::Number(0.0); unique_values.len()];
            if let Some(index) = unique_values.iter().position(|x| x == val) {
                one_hot[index] = Value::Number(1.0);
            }
            encoded.push(Value::List(one_hot));
        }
        
        Ok(Value::List(encoded))
    } else {
        Err(KyaroError::runtime_error("ml_one_hot_encode() requires a list".to_string(), 0, 0))
    }
}

// Distance metrics
fn ml_euclidean_distance(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("ml_euclidean_distance() takes exactly two arguments".to_string(), 0, 0));
    }
    
    let (vec1, vec2) = match (&args[0], &args[1]) {
        (Value::List(v1), Value::List(v2)) => (v1, v2),
        _ => return Err(KyaroError::runtime_error("ml_euclidean_distance() requires two lists".to_string(), 0, 0)),
    };
    
    if vec1.len() != vec2.len() {
        return Err(KyaroError::runtime_error("Vectors must have same length".to_string(), 0, 0));
    }
    
    let mut sum_squares = 0.0;
    for (v1, v2) in vec1.iter().zip(vec2.iter()) {
        if let (Value::Number(n1), Value::Number(n2)) = (v1, v2) {
            sum_squares += (n1 - n2).powi(2);
        } else {
            return Err(KyaroError::runtime_error("Vectors must contain numbers".to_string(), 0, 0));
        }
    }
    
    Ok(Value::Number(sum_squares.sqrt()))
}

fn ml_manhattan_distance(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("ml_manhattan_distance() takes exactly two arguments".to_string(), 0, 0));
    }
    
    let (vec1, vec2) = match (&args[0], &args[1]) {
        (Value::List(v1), Value::List(v2)) => (v1, v2),
        _ => return Err(KyaroError::runtime_error("ml_manhattan_distance() requires two lists".to_string(), 0, 0)),
    };
    
    if vec1.len() != vec2.len() {
        return Err(KyaroError::runtime_error("Vectors must have same length".to_string(), 0, 0));
    }
    
    let mut sum_abs = 0.0;
    for (v1, v2) in vec1.iter().zip(vec2.iter()) {
        if let (Value::Number(n1), Value::Number(n2)) = (v1, v2) {
            sum_abs += (n1 - n2).abs();
        } else {
            return Err(KyaroError::runtime_error("Vectors must contain numbers".to_string(), 0, 0));
        }
    }
    
    Ok(Value::Number(sum_abs))
}

fn ml_cosine_similarity(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("ml_cosine_similarity() takes exactly two arguments".to_string(), 0, 0));
    }
    
    let (vec1, vec2) = match (&args[0], &args[1]) {
        (Value::List(v1), Value::List(v2)) => (v1, v2),
        _ => return Err(KyaroError::runtime_error("ml_cosine_similarity() requires two lists".to_string(), 0, 0)),
    };
    
    if vec1.len() != vec2.len() {
        return Err(KyaroError::runtime_error("Vectors must have same length".to_string(), 0, 0));
    }
    
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;
    
    for (v1, v2) in vec1.iter().zip(vec2.iter()) {
        if let (Value::Number(n1), Value::Number(n2)) = (v1, v2) {
            dot_product += n1 * n2;
            norm1 += n1 * n1;
            norm2 += n2 * n2;
        } else {
            return Err(KyaroError::runtime_error("Vectors must contain numbers".to_string(), 0, 0));
        }
    }
    
    let magnitude = (norm1.sqrt() * norm2.sqrt());
    if magnitude == 0.0 {
        return Ok(Value::Number(0.0));
    }
    
    Ok(Value::Number(dot_product / magnitude))
}

// Placeholder implementations for remaining functions
fn ml_knn_predict(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("ml_knn_predict() not yet implemented".to_string(), 0, 0))
}

fn ml_kmeans(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("ml_kmeans() not yet implemented".to_string(), 0, 0))
}

fn ml_mse(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("ml_mse() takes exactly two arguments".to_string(), 0, 0));
    }
    
    let (y_true, y_pred) = match (&args[0], &args[1]) {
        (Value::List(t), Value::List(p)) => (t, p),
        _ => return Err(KyaroError::runtime_error("ml_mse() requires two lists".to_string(), 0, 0)),
    };
    
    if y_true.len() != y_pred.len() {
        return Err(KyaroError::runtime_error("Arrays must have same length".to_string(), 0, 0));
    }
    
    let mut sum_squares = 0.0;
    for (true_val, pred_val) in y_true.iter().zip(y_pred.iter()) {
        if let (Value::Number(t), Value::Number(p)) = (true_val, pred_val) {
            sum_squares += (t - p).powi(2);
        } else {
            return Err(KyaroError::runtime_error("Arrays must contain numbers".to_string(), 0, 0));
        }
    }
    
    Ok(Value::Number(sum_squares / y_true.len() as f64))
}

fn ml_mae(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(KyaroError::runtime_error("ml_mae() takes exactly two arguments".to_string(), 0, 0));
    }
    
    let (y_true, y_pred) = match (&args[0], &args[1]) {
        (Value::List(t), Value::List(p)) => (t, p),
        _ => return Err(KyaroError::runtime_error("ml_mae() requires two lists".to_string(), 0, 0)),
    };
    
    if y_true.len() != y_pred.len() {
        return Err(KyaroError::runtime_error("Arrays must have same length".to_string(), 0, 0));
    }
    
    let mut sum_abs = 0.0;
    for (true_val, pred_val) in y_true.iter().zip(y_pred.iter()) {
        if let (Value::Number(t), Value::Number(p)) = (true_val, pred_val) {
            sum_abs += (t - p).abs();
        } else {
            return Err(KyaroError::runtime_error("Arrays must contain numbers".to_string(), 0, 0));
        }
    }
    
    Ok(Value::Number(sum_abs / y_true.len() as f64))
}

fn ml_rmse(args: &[Value]) -> Result<Value> {
    let mse_result = ml_mse(args)?;
    if let Value::Number(mse) = mse_result {
        Ok(Value::Number(mse.sqrt()))
    } else {
        Err(KyaroError::runtime_error("RMSE calculation error".to_string(), 0, 0))
    }
}

fn ml_r2_score(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("ml_r2_score() not yet implemented".to_string(), 0, 0))
}

fn ml_accuracy(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("ml_accuracy() not yet implemented".to_string(), 0, 0))
}

fn ml_precision(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("ml_precision() not yet implemented".to_string(), 0, 0))
}

fn ml_recall(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("ml_recall() not yet implemented".to_string(), 0, 0))
}

fn ml_f1_score(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("ml_f1_score() not yet implemented".to_string(), 0, 0))
}

fn ml_confusion_matrix(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("ml_confusion_matrix() not yet implemented".to_string(), 0, 0))
}

// Neural network functions
fn nn_tanh(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("nn_tanh() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(x) = &args[0] {
        Ok(Value::Number(x.tanh()))
    } else {
        Err(KyaroError::runtime_error("nn_tanh() requires a number".to_string(), 0, 0))
    }
}

fn nn_leaky_relu(args: &[Value]) -> Result<Value> {
    let alpha = if args.len() > 1 {
        if let Value::Number(a) = &args[1] {
            *a
        } else {
            return Err(KyaroError::runtime_error("alpha must be a number".to_string(), 0, 0));
        }
    } else {
        0.01
    };
    
    if let Value::Number(x) = &args[0] {
        Ok(Value::Number(if *x > 0.0 { *x } else { alpha * x }))
    } else {
        Err(KyaroError::runtime_error("nn_leaky_relu() requires a number".to_string(), 0, 0))
    }
}

fn nn_elu(args: &[Value]) -> Result<Value> {
    let alpha = if args.len() > 1 {
        if let Value::Number(a) = &args[1] {
            *a
        } else {
            return Err(KyaroError::runtime_error("alpha must be a number".to_string(), 0, 0));
        }
    } else {
        1.0
    };
    
    if let Value::Number(x) = &args[0] {
        Ok(Value::Number(if *x > 0.0 { *x } else { alpha * (x.exp() - 1.0) }))
    } else {
        Err(KyaroError::runtime_error("nn_elu() requires a number".to_string(), 0, 0))
    }
}

fn nn_softplus(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(KyaroError::runtime_error("nn_softplus() takes exactly one argument".to_string(), 0, 0));
    }
    
    if let Value::Number(x) = &args[0] {
        Ok(Value::Number((1.0 + x.exp()).ln()))
    } else {
        Err(KyaroError::runtime_error("nn_softplus() requires a number".to_string(), 0, 0))
    }
}

// Loss functions
fn nn_mse_loss(args: &[Value]) -> Result<Value> {
    ml_mse(args)
}

fn nn_binary_crossentropy(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("nn_binary_crossentropy() not yet implemented".to_string(), 0, 0))
}

fn nn_categorical_crossentropy(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("nn_categorical_crossentropy() not yet implemented".to_string(), 0, 0))
}

fn nn_dropout(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("nn_dropout() not yet implemented".to_string(), 0, 0))
}

fn nn_batch_norm(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("nn_batch_norm() not yet implemented".to_string(), 0, 0))
}

// Matrix operations
fn matrix_multiply(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("matrix_multiply() not yet implemented".to_string(), 0, 0))
}

fn matrix_transpose(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("matrix_transpose() not yet implemented".to_string(), 0, 0))
}

fn matrix_add(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("matrix_add() not yet implemented".to_string(), 0, 0))
}

fn matrix_subtract(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("matrix_subtract() not yet implemented".to_string(), 0, 0))
}

fn matrix_identity(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("matrix_identity() not yet implemented".to_string(), 0, 0))
}

fn matrix_determinant(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("matrix_determinant() not yet implemented".to_string(), 0, 0))
}

fn gradient_descent_step(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("gradient_descent_step() not yet implemented".to_string(), 0, 0))
}

fn adam_step(_args: &[Value]) -> Result<Value> {
    Err(KyaroError::runtime_error("adam_step() not yet implemented".to_string(), 0, 0))
}
