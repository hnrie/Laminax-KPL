import sys
import math
import time
import statistics
import random

def builtin_print(*args):
    print(*args)
    return None

def builtin_input(prompt=""):
    return input(prompt)

def builtin_len(obj):
    if isinstance(obj, (list, str)):
        return len(obj)
    raise TypeError(f"len() not supported for type {type(obj).__name__}")

def builtin_str(obj):
    if obj is None:
        return "null"
    if isinstance(obj, bool):
        return "true" if obj else "false"
    return str(obj)

def builtin_int(obj):
    return int(obj)

def builtin_float(obj):
    return float(obj)

def builtin_type(obj):
    if obj is None:
        return "null"
    if isinstance(obj, bool):
        return "boolean"
    if isinstance(obj, int):
        return "integer"
    if isinstance(obj, float):
        return "float"
    if isinstance(obj, str):
        return "string"
    if isinstance(obj, list):
        return "list"
    if callable(obj):
        return "function"
    return "unknown"

def builtin_range(start, end=None, step=1):
    if end is None:
        return list(range(start))
    return list(range(start, end, step))

def builtin_append(lst, item):
    if not isinstance(lst, list):
        raise TypeError("append() requires a list")
    lst.append(item)
    return None

def builtin_pop(lst, index=-1):
    if not isinstance(lst, list):
        raise TypeError("pop() requires a list")
    return lst.pop(index)

def builtin_push(lst, item):
    if not isinstance(lst, list):
        raise TypeError("push() requires a list")
    lst.append(item)
    return None

def builtin_exit(code=0):
    sys.exit(code)

def builtin_time():
    return time.time()

def builtin_sleep(seconds):
    time.sleep(seconds)
    return None

def builtin_abs(num):
    return abs(num)

def builtin_min(*args):
    if len(args) == 1 and isinstance(args[0], list):
        return min(args[0])
    return min(args)

def builtin_max(*args):
    if len(args) == 1 and isinstance(args[0], list):
        return max(args[0])
    return max(args)

def builtin_sum(lst):
    if not isinstance(lst, list):
        raise TypeError("sum() requires a list")
    return sum(lst)

def builtin_sqrt(num):
    return math.sqrt(num)

def builtin_pow(base, exp):
    return pow(base, exp)

def builtin_exp(x):
    return math.exp(x)

def builtin_log(x, base=math.e):
    return math.log(x, base)

def builtin_log10(x):
    return math.log10(x)

def builtin_log2(x):
    return math.log2(x)

def builtin_ln(x):
    return math.log(x)

def builtin_floor(num):
    return math.floor(num)

def builtin_ceil(num):
    return math.ceil(num)

def builtin_round(num, digits=0):
    return round(num, digits)

def builtin_trunc(num):
    return math.trunc(num)

def builtin_sin(x):
    return math.sin(x)

def builtin_cos(x):
    return math.cos(x)

def builtin_tan(x):
    return math.tan(x)

def builtin_asin(x):
    return math.asin(x)

def builtin_acos(x):
    return math.acos(x)

def builtin_atan(x):
    return math.atan(x)

def builtin_atan2(y, x):
    return math.atan2(y, x)

def builtin_sinh(x):
    return math.sinh(x)

def builtin_cosh(x):
    return math.cosh(x)

def builtin_tanh(x):
    return math.tanh(x)

def builtin_asinh(x):
    return math.asinh(x)

def builtin_acosh(x):
    return math.acosh(x)

def builtin_atanh(x):
    return math.atanh(x)

def builtin_degrees(x):
    return math.degrees(x)

def builtin_radians(x):
    return math.radians(x)

def builtin_hypot(*args):
    return math.hypot(*args)

def builtin_factorial(n):
    return math.factorial(int(n))

def builtin_gcd(*args):
    return math.gcd(*[int(x) for x in args])

def builtin_lcm(*args):
    return math.lcm(*[int(x) for x in args])

def builtin_isnan(x):
    return math.isnan(x)

def builtin_isinf(x):
    return math.isinf(x)

def builtin_isfinite(x):
    return math.isfinite(x)

def builtin_copysign(x, y):
    return math.copysign(x, y)

def builtin_fmod(x, y):
    return math.fmod(x, y)

def builtin_remainder(x, y):
    return math.remainder(x, y)

def builtin_modf(x):
    fractional, integer = math.modf(x)
    return [fractional, integer]

def builtin_frexp(x):
    mantissa, exponent = math.frexp(x)
    return [mantissa, exponent]

def builtin_ldexp(x, i):
    return math.ldexp(x, int(i))

def builtin_erf(x):
    return math.erf(x)

def builtin_erfc(x):
    return math.erfc(x)

def builtin_gamma(x):
    return math.gamma(x)

def builtin_lgamma(x):
    return math.lgamma(x)

def builtin_mean(lst):
    if not isinstance(lst, list):
        raise TypeError("mean() requires a list")
    return statistics.mean(lst)

def builtin_median(lst):
    if not isinstance(lst, list):
        raise TypeError("median() requires a list")
    return statistics.median(lst)

def builtin_median_low(lst):
    if not isinstance(lst, list):
        raise TypeError("median_low() requires a list")
    return statistics.median_low(lst)

def builtin_median_high(lst):
    if not isinstance(lst, list):
        raise TypeError("median_high() requires a list")
    return statistics.median_high(lst)

def builtin_mode(lst):
    if not isinstance(lst, list):
        raise TypeError("mode() requires a list")
    return statistics.mode(lst)

def builtin_stdev(lst):
    if not isinstance(lst, list):
        raise TypeError("stdev() requires a list")
    return statistics.stdev(lst)

def builtin_variance(lst):
    if not isinstance(lst, list):
        raise TypeError("variance() requires a list")
    return statistics.variance(lst)

def builtin_pstdev(lst):
    if not isinstance(lst, list):
        raise TypeError("pstdev() requires a list")
    return statistics.pstdev(lst)

def builtin_pvariance(lst):
    if not isinstance(lst, list):
        raise TypeError("pvariance() requires a list")
    return statistics.pvariance(lst)

def builtin_quantiles(lst, n=4):
    if not isinstance(lst, list):
        raise TypeError("quantiles() requires a list")
    return statistics.quantiles(lst, n=int(n))

def builtin_covariance(x, y):
    if not isinstance(x, list) or not isinstance(y, list):
        raise TypeError("covariance() requires two lists")
    return statistics.covariance(x, y)

def builtin_correlation(x, y):
    if not isinstance(x, list) or not isinstance(y, list):
        raise TypeError("correlation() requires two lists")
    return statistics.correlation(x, y)

def builtin_linear_regression(x, y):
    if not isinstance(x, list) or not isinstance(y, list):
        raise TypeError("linear_regression() requires two lists")
    result = statistics.linear_regression(x, y)
    return [result.slope, result.intercept]

def builtin_harmonic_mean(lst):
    if not isinstance(lst, list):
        raise TypeError("harmonic_mean() requires a list")
    return statistics.harmonic_mean(lst)

def builtin_geometric_mean(lst):
    if not isinstance(lst, list):
        raise TypeError("geometric_mean() requires a list")
    return statistics.geometric_mean(lst)

def builtin_fmean(lst):
    if not isinstance(lst, list):
        raise TypeError("fmean() requires a list")
    return statistics.fmean(lst)

def builtin_random():
    return random.random()

def builtin_randint(a, b):
    return random.randint(int(a), int(b))

def builtin_uniform(a, b):
    return random.uniform(a, b)

def builtin_choice(lst):
    if not isinstance(lst, list):
        raise TypeError("choice() requires a list")
    return random.choice(lst)

def builtin_shuffle(lst):
    if not isinstance(lst, list):
        raise TypeError("shuffle() requires a list")
    random.shuffle(lst)
    return None

def builtin_sample(lst, k):
    if not isinstance(lst, list):
        raise TypeError("sample() requires a list")
    return random.sample(lst, int(k))

def builtin_gauss(mu, sigma):
    return random.gauss(mu, sigma)

def builtin_normalvariate(mu, sigma):
    return random.normalvariate(mu, sigma)

def builtin_lognormvariate(mu, sigma):
    return random.lognormvariate(mu, sigma)

def builtin_expovariate(lambd):
    return random.expovariate(lambd)

def builtin_vonmisesvariate(mu, kappa):
    return random.vonmisesvariate(mu, kappa)

def builtin_gammavariate(alpha, beta):
    return random.gammavariate(alpha, beta)

def builtin_betavariate(alpha, beta):
    return random.betavariate(alpha, beta)

def builtin_paretovariate(alpha):
    return random.paretovariate(alpha)

def builtin_weibullvariate(alpha, beta):
    return random.weibullvariate(alpha, beta)

def builtin_seed(a=None):
    random.seed(a)
    return None

def builtin_reverse(lst):
    if not isinstance(lst, list):
        raise TypeError("reverse() requires a list")
    lst.reverse()
    return None

def builtin_sort(lst):
    if not isinstance(lst, list):
        raise TypeError("sort() requires a list")
    lst.sort()
    return None

def builtin_sorted(lst):
    if not isinstance(lst, list):
        raise TypeError("sorted() requires a list")
    return sorted(lst)

def builtin_all(lst):
    if not isinstance(lst, list):
        raise TypeError("all() requires a list")
    return all(lst)

def builtin_any(lst):
    if not isinstance(lst, list):
        raise TypeError("any() requires a list")
    return any(lst)

def builtin_count(lst, item):
    if isinstance(lst, list):
        return lst.count(item)
    elif isinstance(lst, str):
        return lst.count(item)
    raise TypeError("count() requires a list or string")

def builtin_index(lst, item):
    if isinstance(lst, list):
        return lst.index(item)
    elif isinstance(lst, str):
        return lst.index(item)
    raise TypeError("index() requires a list or string")

def builtin_insert(lst, index, item):
    if not isinstance(lst, list):
        raise TypeError("insert() requires a list")
    lst.insert(int(index), item)
    return None

def builtin_remove(lst, item):
    if not isinstance(lst, list):
        raise TypeError("remove() requires a list")
    lst.remove(item)
    return None

def builtin_clear(lst):
    if not isinstance(lst, list):
        raise TypeError("clear() requires a list")
    lst.clear()
    return None

def builtin_copy(lst):
    if isinstance(lst, list):
        return lst.copy()
    raise TypeError("copy() requires a list")

def builtin_extend(lst1, lst2):
    if not isinstance(lst1, list) or not isinstance(lst2, list):
        raise TypeError("extend() requires two lists")
    lst1.extend(lst2)
    return None

def builtin_zip(*args):
    return list(zip(*args))

def builtin_enumerate(lst, start=0):
    if not isinstance(lst, list):
        raise TypeError("enumerate() requires a list")
    return [[i, item] for i, item in enumerate(lst, int(start))]

def builtin_filter(func, lst):
    if not isinstance(lst, list):
        raise TypeError("filter() requires a list")
    return [item for item in lst if func(item)]

def builtin_map(func, lst):
    if not isinstance(lst, list):
        raise TypeError("map() requires a list")
    return [func(item) for item in lst]

def builtin_reduce(func, lst, initial=None):
    if not isinstance(lst, list):
        raise TypeError("reduce() requires a list")
    from functools import reduce
    if initial is None:
        return reduce(func, lst)
    return reduce(func, lst, initial)

def builtin_product(lst):
    if not isinstance(lst, list):
        raise TypeError("product() requires a list")
    result = 1
    for item in lst:
        result *= item
    return result

def builtin_cumsum(lst):
    if not isinstance(lst, list):
        raise TypeError("cumsum() requires a list")
    result = []
    total = 0
    for item in lst:
        total += item
        result.append(total)
    return result

def builtin_cumprod(lst):
    if not isinstance(lst, list):
        raise TypeError("cumprod() requires a list")
    result = []
    total = 1
    for item in lst:
        total *= item
        result.append(total)
    return result

def builtin_diff(lst):
    if not isinstance(lst, list):
        raise TypeError("diff() requires a list")
    if len(lst) < 2:
        return []
    return [lst[i+1] - lst[i] for i in range(len(lst)-1)]

def builtin_unique(lst):
    if not isinstance(lst, list):
        raise TypeError("unique() requires a list")
    seen = []
    for item in lst:
        if item not in seen:
            seen.append(item)
    return seen

def builtin_flatten(lst):
    if not isinstance(lst, list):
        raise TypeError("flatten() requires a list")
    result = []
    for item in lst:
        if isinstance(item, list):
            result.extend(builtin_flatten(item))
        else:
            result.append(item)
    return result

def builtin_transpose(matrix):
    if not isinstance(matrix, list):
        raise TypeError("transpose() requires a list")
    return list(map(list, zip(*matrix)))

def builtin_dot(a, b):
    if not isinstance(a, list) or not isinstance(b, list):
        raise TypeError("dot() requires two lists")
    if len(a) != len(b):
        raise ValueError("dot() requires lists of same length")
    return sum(x * y for x, y in zip(a, b))

def builtin_norm(lst, p=2):
    if not isinstance(lst, list):
        raise TypeError("norm() requires a list")
    if p == float('inf'):
        return max(abs(x) for x in lst)
    return sum(abs(x)**p for x in lst) ** (1/p)

def builtin_normalize(lst):
    if not isinstance(lst, list):
        raise TypeError("normalize() requires a list")
    magnitude = builtin_norm(lst)
    if magnitude == 0:
        return lst
    return [x / magnitude for x in lst]

def builtin_clamp(value, min_val, max_val):
    return max(min_val, min(value, max_val))

def builtin_lerp(a, b, t):
    return a + (b - a) * t

def builtin_sigmoid(x):
    return 1 / (1 + math.exp(-x))

def builtin_relu(x):
    return max(0, x)

def builtin_softmax(lst):
    if not isinstance(lst, list):
        raise TypeError("softmax() requires a list")
    exp_values = [math.exp(x) for x in lst]
    exp_sum = sum(exp_values)
    return [x / exp_sum for x in exp_values]

def builtin_pi():
    return math.pi

def builtin_e():
    return math.e

def builtin_tau():
    return math.tau

def builtin_inf():
    return math.inf

def builtin_nan():
    return math.nan

BUILTIN_FUNCTIONS = {
    'print': builtin_print,
    'input': builtin_input,
    'len': builtin_len,
    'str': builtin_str,
    'int': builtin_int,
    'float': builtin_float,
    'type': builtin_type,
    'range': builtin_range,
    'append': builtin_append,
    'pop': builtin_pop,
    'push': builtin_push,
    'exit': builtin_exit,
    'time': builtin_time,
    'sleep': builtin_sleep,
    'abs': builtin_abs,
    'min': builtin_min,
    'max': builtin_max,
    'sum': builtin_sum,
    'sqrt': builtin_sqrt,
    'pow': builtin_pow,
    'exp': builtin_exp,
    'log': builtin_log,
    'log10': builtin_log10,
    'log2': builtin_log2,
    'ln': builtin_ln,
    'floor': builtin_floor,
    'ceil': builtin_ceil,
    'round': builtin_round,
    'trunc': builtin_trunc,
    'sin': builtin_sin,
    'cos': builtin_cos,
    'tan': builtin_tan,
    'asin': builtin_asin,
    'acos': builtin_acos,
    'atan': builtin_atan,
    'atan2': builtin_atan2,
    'sinh': builtin_sinh,
    'cosh': builtin_cosh,
    'tanh': builtin_tanh,
    'asinh': builtin_asinh,
    'acosh': builtin_acosh,
    'atanh': builtin_atanh,
    'degrees': builtin_degrees,
    'radians': builtin_radians,
    'hypot': builtin_hypot,
    'factorial': builtin_factorial,
    'gcd': builtin_gcd,
    'lcm': builtin_lcm,
    'isnan': builtin_isnan,
    'isinf': builtin_isinf,
    'isfinite': builtin_isfinite,
    'copysign': builtin_copysign,
    'fmod': builtin_fmod,
    'remainder': builtin_remainder,
    'modf': builtin_modf,
    'frexp': builtin_frexp,
    'ldexp': builtin_ldexp,
    'erf': builtin_erf,
    'erfc': builtin_erfc,
    'gamma': builtin_gamma,
    'lgamma': builtin_lgamma,
    'mean': builtin_mean,
    'median': builtin_median,
    'median_low': builtin_median_low,
    'median_high': builtin_median_high,
    'mode': builtin_mode,
    'stdev': builtin_stdev,
    'variance': builtin_variance,
    'pstdev': builtin_pstdev,
    'pvariance': builtin_pvariance,
    'quantiles': builtin_quantiles,
    'covariance': builtin_covariance,
    'correlation': builtin_correlation,
    'linear_regression': builtin_linear_regression,
    'harmonic_mean': builtin_harmonic_mean,
    'geometric_mean': builtin_geometric_mean,
    'fmean': builtin_fmean,
    'random': builtin_random,
    'randint': builtin_randint,
    'uniform': builtin_uniform,
    'choice': builtin_choice,
    'shuffle': builtin_shuffle,
    'sample': builtin_sample,
    'gauss': builtin_gauss,
    'normalvariate': builtin_normalvariate,
    'lognormvariate': builtin_lognormvariate,
    'expovariate': builtin_expovariate,
    'vonmisesvariate': builtin_vonmisesvariate,
    'gammavariate': builtin_gammavariate,
    'betavariate': builtin_betavariate,
    'paretovariate': builtin_paretovariate,
    'weibullvariate': builtin_weibullvariate,
    'seed': builtin_seed,
    'reverse': builtin_reverse,
    'sort': builtin_sort,
    'sorted': builtin_sorted,
    'all': builtin_all,
    'any': builtin_any,
    'count': builtin_count,
    'index': builtin_index,
    'insert': builtin_insert,
    'remove': builtin_remove,
    'clear': builtin_clear,
    'copy': builtin_copy,
    'extend': builtin_extend,
    'zip': builtin_zip,
    'enumerate': builtin_enumerate,
    'filter': builtin_filter,
    'map': builtin_map,
    'reduce': builtin_reduce,
    'product': builtin_product,
    'cumsum': builtin_cumsum,
    'cumprod': builtin_cumprod,
    'diff': builtin_diff,
    'unique': builtin_unique,
    'flatten': builtin_flatten,
    'transpose': builtin_transpose,
    'dot': builtin_dot,
    'norm': builtin_norm,
    'normalize': builtin_normalize,
    'clamp': builtin_clamp,
    'lerp': builtin_lerp,
    'sigmoid': builtin_sigmoid,
    'relu': builtin_relu,
    'softmax': builtin_softmax,
    'pi': builtin_pi,
    'e': builtin_e,
    'tau': builtin_tau,
    'inf': builtin_inf,
    'nan': builtin_nan,
}
