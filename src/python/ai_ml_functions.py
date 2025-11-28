import math
import random
from functools import reduce

def ml_train_test_split(data, labels, test_size=0.2, random_state=None):
    if random_state is not None:
        random.seed(random_state)
    
    indices = list(range(len(data)))
    random.shuffle(indices)
    
    split_idx = int(len(data) * (1 - test_size))
    train_indices = indices[:split_idx]
    test_indices = indices[split_idx:]
    
    x_train = [data[i] for i in train_indices]
    x_test = [data[i] for i in test_indices]
    y_train = [labels[i] for i in train_indices]
    y_test = [labels[i] for i in test_indices]
    
    return [x_train, x_test, y_train, y_test]

def ml_standardize(data):
    mean = sum(data) / len(data)
    variance = sum((x - mean) ** 2 for x in data) / len(data)
    std = math.sqrt(variance)
    
    if std == 0:
        return data
    
    return [(x - mean) / std for x in data]

def ml_min_max_scale(data, feature_range=None):
    if feature_range is None:
        feature_range = [0, 1]
    
    min_val = min(data)
    max_val = max(data)
    
    if min_val == max_val:
        return data
    
    scaled = []
    for x in data:
        normalized = (x - min_val) / (max_val - min_val)
        scaled_val = normalized * (feature_range[1] - feature_range[0]) + feature_range[0]
        scaled.append(scaled_val)
    
    return scaled

def ml_one_hot_encode(labels):
    unique_labels = []
    for label in labels:
        if label not in unique_labels:
            unique_labels.append(label)
    
    encoded = []
    for label in labels:
        encoding = [0] * len(unique_labels)
        encoding[unique_labels.index(label)] = 1
        encoded.append(encoding)
    
    return encoded

def ml_euclidean_distance(a, b):
    return math.sqrt(sum((x - y) ** 2 for x, y in zip(a, b)))

def ml_manhattan_distance(a, b):
    return sum(abs(x - y) for x, y in zip(a, b))

def ml_cosine_similarity(a, b):
    dot_product = sum(x * y for x, y in zip(a, b))
    norm_a = math.sqrt(sum(x ** 2 for x in a))
    norm_b = math.sqrt(sum(y ** 2 for y in b))
    
    if norm_a == 0 or norm_b == 0:
        return 0
    
    return dot_product / (norm_a * norm_b)

def ml_knn_predict(x_train, y_train, x_test, k=3):
    distances = []
    for i, train_point in enumerate(x_train):
        dist = ml_euclidean_distance(x_test, train_point)
        distances.append([dist, y_train[i]])
    
    distances.sort(key=lambda x: x[0])
    k_nearest = distances[:k]
    
    labels = [item[1] for item in k_nearest]
    
    label_counts = {}
    for label in labels:
        label_counts[label] = label_counts.get(label, 0) + 1
    
    return max(label_counts.items(), key=lambda x: x[1])[0]

def ml_kmeans(data, k=3, max_iters=100, random_state=None):
    if random_state is not None:
        random.seed(random_state)
    
    centroids = random.sample(data, k)
    
    for _ in range(max_iters):
        clusters = [[] for _ in range(k)]
        
        for point in data:
            distances = [ml_euclidean_distance(point, centroid) for centroid in centroids]
            cluster_idx = distances.index(min(distances))
            clusters[cluster_idx].append(point)
        
        new_centroids = []
        for cluster in clusters:
            if cluster:
                centroid = [sum(coord) / len(cluster) for coord in zip(*cluster)]
                new_centroids.append(centroid)
            else:
                new_centroids.append(random.choice(data))
        
        if centroids == new_centroids:
            break
        
        centroids = new_centroids
    
    labels = []
    for point in data:
        distances = [ml_euclidean_distance(point, centroid) for centroid in centroids]
        labels.append(distances.index(min(distances)))
    
    return [labels, centroids]

def ml_mse(y_true, y_pred):
    return sum((t - p) ** 2 for t, p in zip(y_true, y_pred)) / len(y_true)

def ml_mae(y_true, y_pred):
    return sum(abs(t - p) for t, p in zip(y_true, y_pred)) / len(y_true)

def ml_rmse(y_true, y_pred):
    return math.sqrt(ml_mse(y_true, y_pred))

def ml_r2_score(y_true, y_pred):
    mean_y = sum(y_true) / len(y_true)
    ss_tot = sum((y - mean_y) ** 2 for y in y_true)
    ss_res = sum((t - p) ** 2 for t, p in zip(y_true, y_pred))
    
    if ss_tot == 0:
        return 0
    
    return 1 - (ss_res / ss_tot)

def ml_accuracy(y_true, y_pred):
    correct = sum(1 for t, p in zip(y_true, y_pred) if t == p)
    return correct / len(y_true)

def ml_confusion_matrix(y_true, y_pred):
    labels = []
    for label in y_true + y_pred:
        if label not in labels:
            labels.append(label)
    labels.sort()
    
    n = len(labels)
    matrix = [[0] * n for _ in range(n)]
    
    for true, pred in zip(y_true, y_pred):
        true_idx = labels.index(true)
        pred_idx = labels.index(pred)
        matrix[true_idx][pred_idx] += 1
    
    return matrix

def ml_precision(y_true, y_pred, positive_label=1):
    tp = sum(1 for t, p in zip(y_true, y_pred) if t == positive_label and p == positive_label)
    fp = sum(1 for t, p in zip(y_true, y_pred) if t != positive_label and p == positive_label)
    
    if tp + fp == 0:
        return 0
    
    return tp / (tp + fp)

def ml_recall(y_true, y_pred, positive_label=1):
    tp = sum(1 for t, p in zip(y_true, y_pred) if t == positive_label and p == positive_label)
    fn = sum(1 for t, p in zip(y_true, y_pred) if t == positive_label and p != positive_label)
    
    if tp + fn == 0:
        return 0
    
    return tp / (tp + fn)

def ml_f1_score(y_true, y_pred, positive_label=1):
    prec = ml_precision(y_true, y_pred, positive_label)
    rec = ml_recall(y_true, y_pred, positive_label)
    
    if prec + rec == 0:
        return 0
    
    return 2 * (prec * rec) / (prec + rec)

def nn_tanh(x):
    return math.tanh(x)

def nn_leaky_relu(x, alpha=0.01):
    return x if x > 0 else alpha * x

def nn_elu(x, alpha=1.0):
    return x if x > 0 else alpha * (math.exp(x) - 1)

def nn_softplus(x):
    return math.log(1 + math.exp(x))

def nn_mse_loss(y_true, y_pred):
    return sum((t - p) ** 2 for t, p in zip(y_true, y_pred)) / len(y_true)

def nn_binary_crossentropy(y_true, y_pred):
    epsilon = 1e-15
    loss = 0
    for t, p in zip(y_true, y_pred):
        p = max(min(p, 1 - epsilon), epsilon)
        loss += -(t * math.log(p) + (1 - t) * math.log(1 - p))
    return loss / len(y_true)

def nn_categorical_crossentropy(y_true, y_pred):
    epsilon = 1e-15
    loss = 0
    for true_dist, pred_dist in zip(y_true, y_pred):
        for t, p in zip(true_dist, pred_dist):
            p = max(min(p, 1 - epsilon), epsilon)
            loss += -t * math.log(p)
    return loss / len(y_true)

def nn_dropout(x, rate=0.5, training=True):
    if not training or rate == 0:
        return x
    
    keep_prob = 1 - rate
    mask = [1 if random.random() < keep_prob else 0 for _ in x]
    return [val * m / keep_prob for val, m in zip(x, mask)]

def nn_batch_norm(x):
    mean = sum(x) / len(x)
    variance = sum((val - mean) ** 2 for val in x) / len(x)
    std = math.sqrt(variance + 1e-8)
    
    return [(val - mean) / std for val in x]

def matrix_multiply(a, b):
    if len(a[0]) != len(b):
        raise ValueError("Matrix dimensions incompatible for multiplication")
    
    result = []
    for i in range(len(a)):
        row = []
        for j in range(len(b[0])):
            sum_val = sum(a[i][k] * b[k][j] for k in range(len(b)))
            row.append(sum_val)
        result.append(row)
    
    return result

def matrix_transpose(matrix):
    return list(map(list, zip(*matrix)))

def matrix_add(a, b):
    if len(a) != len(b) or len(a[0]) != len(b[0]):
        raise ValueError("Matrix dimensions must match for addition")
    
    return [[a[i][j] + b[i][j] for j in range(len(a[0]))] for i in range(len(a))]

def matrix_subtract(a, b):
    if len(a) != len(b) or len(a[0]) != len(b[0]):
        raise ValueError("Matrix dimensions must match for subtraction")
    
    return [[a[i][j] - b[i][j] for j in range(len(a[0]))] for i in range(len(a))]

def matrix_scalar_multiply(matrix, scalar):
    return [[val * scalar for val in row] for row in matrix]

def matrix_identity(n):
    return [[1 if i == j else 0 for j in range(n)] for i in range(n)]

def matrix_determinant(matrix):
    n = len(matrix)
    
    if n == 1:
        return matrix[0][0]
    
    if n == 2:
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
    
    det = 0
    for j in range(n):
        minor = [row[:j] + row[j+1:] for row in matrix[1:]]
        det += ((-1) ** j) * matrix[0][j] * matrix_determinant(minor)
    
    return det

def gradient_descent_step(weights, gradients, learning_rate=0.01):
    return [w - learning_rate * g for w, g in zip(weights, gradients)]

def adam_step(weights, gradients, m, v, t, learning_rate=0.001, beta1=0.9, beta2=0.999, epsilon=1e-8):
    m_new = [beta1 * m_i + (1 - beta1) * g for m_i, g in zip(m, gradients)]
    v_new = [beta2 * v_i + (1 - beta2) * (g ** 2) for v_i, g in zip(v, gradients)]
    
    m_hat = [m_i / (1 - beta1 ** t) for m_i in m_new]
    v_hat = [v_i / (1 - beta2 ** t) for v_i in v_new]
    
    weights_new = [w - learning_rate * m_h / (math.sqrt(v_h) + epsilon) 
                   for w, m_h, v_h in zip(weights, m_hat, v_hat)]
    
    return [weights_new, m_new, v_new]

AI_ML_FUNCTIONS = {
    'ml_train_test_split': ml_train_test_split,
    'ml_standardize': ml_standardize,
    'ml_min_max_scale': ml_min_max_scale,
    'ml_one_hot_encode': ml_one_hot_encode,
    'ml_euclidean_distance': ml_euclidean_distance,
    'ml_manhattan_distance': ml_manhattan_distance,
    'ml_cosine_similarity': ml_cosine_similarity,
    'ml_knn_predict': ml_knn_predict,
    'ml_kmeans': ml_kmeans,
    'ml_mse': ml_mse,
    'ml_mae': ml_mae,
    'ml_rmse': ml_rmse,
    'ml_r2_score': ml_r2_score,
    'ml_accuracy': ml_accuracy,
    'ml_confusion_matrix': ml_confusion_matrix,
    'ml_precision': ml_precision,
    'ml_recall': ml_recall,
    'ml_f1_score': ml_f1_score,
    'nn_tanh': nn_tanh,
    'nn_leaky_relu': nn_leaky_relu,
    'nn_elu': nn_elu,
    'nn_softplus': nn_softplus,
    'nn_mse_loss': nn_mse_loss,
    'nn_binary_crossentropy': nn_binary_crossentropy,
    'nn_categorical_crossentropy': nn_categorical_crossentropy,
    'nn_dropout': nn_dropout,
    'nn_batch_norm': nn_batch_norm,
    'matrix_multiply': matrix_multiply,
    'matrix_transpose': matrix_transpose,
    'matrix_add': matrix_add,
    'matrix_subtract': matrix_subtract,
    'matrix_scalar_multiply': matrix_scalar_multiply,
    'matrix_identity': matrix_identity,
    'matrix_determinant': matrix_determinant,
    'gradient_descent_step': gradient_descent_step,
    'adam_step': adam_step,
}
