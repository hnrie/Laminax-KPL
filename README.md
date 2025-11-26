# Laminax Kyaro Programming Language (KPL)

**By [Laminax](https://laminax.org)**

A dynamically-typed, interpreted programming language written in pure Python with clean, readable syntax.

## Features

- **Dynamic typing** - Variables don't need type declarations
- **First-class functions** - Functions are values that can be passed around
- **Control flow** - if/elif/else, while loops, for loops
- **Built-in functions** - Rich standard library for common operations
- **Lists** - Dynamic arrays with indexing and iteration
- **String operations** - String manipulation and formatting
- **REPL** - Interactive shell for testing code

## Installation

No installation required! Just Python 3.6+ is needed.

```bash
git clone <repository-url>
cd Laminax-KPL
python main.py
```

## Usage

### Interactive REPL

Run without arguments to start the interactive shell:

```bash
python main.py
```

### Execute a File

Run a Kyaro program file:

```bash
python main.py program.kyaro
```

## Language Syntax

### Variables

```kyaro
let x = 10
let name = "Kyaro"
let is_active = true
```

### Functions

```kyaro
func greet(name) {
    print("Hello, " + name + "!")
}

greet("World")
```

### Control Flow

```kyaro
let age = 18

if age >= 18 {
    print("Adult")
} elif age >= 13 {
    print("Teenager")
} else {
    print("Child")
}
```

### Loops

```kyaro
let i = 0
while i < 5 {
    print(i)
    i += 1
}

for item in [1, 2, 3, 4, 5] {
    print(item)
}

for char in "Kyaro" {
    print(char)
}
```

### Lists

```kyaro
let numbers = [1, 2, 3, 4, 5]
print(numbers[0])
append(numbers, 6)
print(len(numbers))
```

### Operators

**Arithmetic:** `+`, `-`, `*`, `/`, `%`, `**` (power)

**Comparison:** `==`, `!=`, `<`, `>`, `<=`, `>=`

**Logical:** `and`, `or`, `not`

**Assignment:** `=`, `+=`, `-=`, `*=`, `/=`

### Built-in Functions

**136 built-in functions** across multiple categories:

- **I/O**: `print()`, `input()`
- **Type conversion**: `str()`, `int()`, `float()`, `type()`
- **Collections**: `len()`, `range()`, `append()`, `pop()`, `push()`, `reverse()`, `sort()`, `sorted()`, `count()`, `index()`, `insert()`, `remove()`, `clear()`, `copy()`, `extend()`, `unique()`, `flatten()`
- **Basic math**: `abs()`, `min()`, `max()`, `sum()`, `sqrt()`, `pow()`, `exp()`, `floor()`, `ceil()`, `round()`, `trunc()`, `factorial()`, `gcd()`, `lcm()`
- **Logarithms**: `log()`, `log10()`, `log2()`, `ln()`
- **Trigonometry**: `sin()`, `cos()`, `tan()`, `asin()`, `acos()`, `atan()`, `atan2()`, `sinh()`, `cosh()`, `tanh()`, `asinh()`, `acosh()`, `atanh()`, `degrees()`, `radians()`, `hypot()`
- **Special functions**: `isnan()`, `isinf()`, `isfinite()`, `copysign()`, `fmod()`, `remainder()`, `modf()`, `frexp()`, `ldexp()`, `erf()`, `erfc()`, `gamma()`, `lgamma()`
- **Statistics**: `mean()`, `median()`, `median_low()`, `median_high()`, `mode()`, `stdev()`, `variance()`, `pstdev()`, `pvariance()`, `quantiles()`, `covariance()`, `correlation()`, `linear_regression()`, `harmonic_mean()`, `geometric_mean()`, `fmean()`
- **Random**: `random()`, `randint()`, `uniform()`, `choice()`, `shuffle()`, `sample()`, `gauss()`, `normalvariate()`, `lognormvariate()`, `expovariate()`, `vonmisesvariate()`, `gammavariate()`, `betavariate()`, `paretovariate()`, `weibullvariate()`, `seed()`
- **Functional**: `zip()`, `enumerate()`, `filter()`, `map()`, `reduce()`, `all()`, `any()`
- **Data analysis**: `product()`, `cumsum()`, `cumprod()`, `diff()`, `transpose()`, `dot()`, `norm()`, `normalize()`
- **Machine learning**: `sigmoid()`, `relu()`, `softmax()`, `clamp()`, `lerp()`
- **Constants**: `pi()`, `e()`, `tau()`, `inf()`, `nan()`
- **Utility**: `exit()`, `time()`, `sleep()`

See [FUNCTIONS.md](FUNCTIONS.md) for complete reference.

### Image Processing Functions

**70 image manipulation functions** powered by Pillow:

- **I/O**: `image_load()`, `image_save()`, `image_new()`
- **Transformations**: `image_resize()`, `image_crop()`, `image_rotate()`, `image_flip_horizontal()`, `image_flip_vertical()`
- **Filters**: `image_blur()`, `image_sharpen()`, `image_edge_enhance()`, `image_find_edges()`, `image_emboss()`, `image_contour()`
- **Enhancements**: `image_brightness()`, `image_contrast()`, `image_color()`, `image_sharpness()`
- **Effects**: `image_grayscale()`, `image_invert()`, `image_posterize()`, `image_solarize()`, `image_equalize()`
- **Composition**: `image_blend()`, `image_add()`, `image_subtract()`, `image_multiply()`, `image_composite()`
- **Drawing**: `image_draw()`, `draw_line()`, `draw_rectangle()`, `draw_circle()`, `draw_ellipse()`, `draw_polygon()`, `draw_text()`
- **Pixel ops**: `image_get_pixel()`, `image_put_pixel()`
- **Channels**: `image_split()`, `image_merge()`, `image_convert()`

See [IMAGE_FUNCTIONS.md](IMAGE_FUNCTIONS.md) for complete reference.

### AI and Machine Learning Functions

**37 AI/ML functions** for data science and neural networks:

- **Data preprocessing**: `ml_train_test_split()`, `ml_standardize()`, `ml_min_max_scale()`, `ml_one_hot_encode()`
- **Algorithms**: `ml_knn_predict()`, `ml_kmeans()`
- **Distance metrics**: `ml_euclidean_distance()`, `ml_manhattan_distance()`, `ml_cosine_similarity()`
- **Regression metrics**: `ml_mse()`, `ml_mae()`, `ml_rmse()`, `ml_r2_score()`
- **Classification metrics**: `ml_accuracy()`, `ml_precision()`, `ml_recall()`, `ml_f1_score()`, `ml_confusion_matrix()`
- **Neural network activations**: `nn_tanh()`, `nn_leaky_relu()`, `nn_elu()`, `nn_softplus()`
- **Loss functions**: `nn_mse_loss()`, `nn_binary_crossentropy()`, `nn_categorical_crossentropy()`
- **NN utilities**: `nn_dropout()`, `nn_batch_norm()`
- **Matrix operations**: `matrix_multiply()`, `matrix_transpose()`, `matrix_add()`, `matrix_subtract()`, `matrix_identity()`, `matrix_determinant()`
- **Optimization**: `gradient_descent_step()`, `adam_step()`

See [AI_ML_FUNCTIONS.md](AI_ML_FUNCTIONS.md) for complete reference.

### File System Functions

**52 file system functions** for comprehensive file and directory operations:

- **File I/O**: `fs_read_file()`, `fs_write_file()`, `fs_read_lines()`, `fs_write_lines()`, `fs_read_bytes()`, `fs_write_bytes()`, `fs_append_file()`
- **File/Dir checks**: `fs_exists()`, `fs_is_file()`, `fs_is_dir()`, `fs_is_link()`
- **File/Dir operations**: `fs_delete_file()`, `fs_delete_dir()`, `fs_create_dir()`, `fs_copy_file()`, `fs_copy_dir()`, `fs_move()`, `fs_rename()`, `fs_touch()`
- **Directory listing**: `fs_list_dir()`, `fs_walk()`, `fs_glob()`, `fs_find_files()`
- **File metadata**: `fs_get_size()`, `fs_get_mtime()`, `fs_get_ctime()`, `fs_get_atime()`, `fs_stat()`
- **Path manipulation**: `path_join()`, `path_split()`, `path_dirname()`, `path_basename()`, `path_splitext()`, `path_abspath()`, `path_realpath()`, `path_normpath()`
- **Working directory**: `fs_get_cwd()`, `fs_change_dir()`, `fs_get_home()`, `fs_get_temp()`
- **Advanced**: `fs_symlink()`, `fs_readlink()`, `fs_chmod()`, `fs_get_extension()`, `fs_get_stem()`, `fs_with_suffix()`

See [FILESYSTEM_FUNCTIONS.md](FILESYSTEM_FUNCTIONS.md) for complete reference.

## Example Programs

### Hello World

```kyaro
print("Hello, World!")
```

### Fibonacci Sequence

```kyaro
func fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

for i in range(10) {
    print(fibonacci(i))
}
```

### Factorial

```kyaro
func factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print(factorial(5))
```

### FizzBuzz

```kyaro
for i in range(1, 101) {
    if i % 15 == 0 {
        print("FizzBuzz")
    } elif i % 3 == 0 {
        print("Fizz")
    } elif i % 5 == 0 {
        print("Buzz")
    } else {
        print(i)
    }
}
```

## Project Structure

```
Laminax-KPL/
├── main.py           # Entry point and REPL
├── lexer.py          # Tokenizer
├── parser.py         # Parser (builds AST)
├── interpreter.py    # Interpreter (executes AST)
├── ast_nodes.py      # AST node definitions
├── environment.py    # Variable scoping
├── builtins.py       # Standard library
├── token_types.py    # Token type definitions
├── errors.py         # Error handling
├── examples/         # Example programs
└── README.md         # This file
```

## License

Created by Laminax - https://laminax.org

## About Laminax

Laminax is committed to building high-quality, stable, and performant software solutions. The Kyaro Programming Language represents our dedication to creating accessible and powerful development tools.
