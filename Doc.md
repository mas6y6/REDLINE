# 🟥 REDLINE v0.7.0 Documentation

REDLINE is a high-performance, transpiled systems language designed to be as readable as Python but as fast as C++.

## 1. Variables & Constants

REDLINE distinguishes between data that changes and data that stays the same. This allows the compiler to optimize memory.

| Keyword | Meaning | C++ Translation |
|---|---|---|
| `val` | Immutable (Constant). Cannot be changed once set. | `const` |
| `var` | Mutable (Variable). Can be updated later. | (Standard variable) |

Syntax:
```redline
val name: string = "Ace"
var health: int = 100
var pi: float = 3.14
var is_active: bool = true
```

## 2. Data Types

REDLINE is strictly typed, meaning the compiler ensures you don't accidentally treat a number like a word.

*   `int`: Whole numbers (e.g., `10`, `-5`).
*   `float`: Decimal numbers (e.g., `10.5`, `3.14`).
*   `string`: Text wrapped in double quotes (e.g., `"Redline"`).
*   `bool`: Logical values (`true` or `false`).
*   `void`: Represents the absence of a value (used for function return types).
*   `list[T]`: A dynamic array of elements of type `T`.

## 3. Functions

Functions in REDLINE use a modern "Arrow" syntax to show what they return.

Syntax:
```redline
def name(param: type) -> return_type:
    # Logic here
    return value
```

If a function does not return a value, the return type can be omitted (defaults to `void`).

```redline
def greet(name: string):
    print("Hello, " + name)
```

## 4. Control Flow (Decision Making)

REDLINE uses `if` and `else` for logic. It uses C-style comparison operators but Python-style structure.

Comparison Operators: `==`, `!=`, `>`, `<`, `>=`, `<=`

## 5. Loops

REDLINE supports `while` and `for` loops for repeating actions.

### While Loops
```redline
var i: int = 0
while i < 5:
    print(i)
    i = i + 1
```

### For Loops
```redline
for i in 0..5:
    print(i)
```

## 6. Lists

REDLINE has a built-in `list` type, which is a dynamic array.

### Declaration
```redline
var my_list: list[int] = [10, 20, 30]
```

### Indexing
Access and assign elements using square brackets.
```redline
val first_element: int = my_list[0]
my_list[1] = 99
```

### Built-in Functions
*   `len(list)`: Returns the number of elements in the list.
*   `append(list, value)`: Adds a new element to the end of the list.

Example:
```redline
var numbers: list[int] = []
append(numbers, 1)
append(numbers, 2)
print(len(numbers)) # Prints 2
```

## 7. Classes & Objects

REDLINE supports Object-Oriented Programming (OOP) with classes.

### Defining a Class
Use the `class` keyword to define a new type.

```redline
class Person:
    var name: string = ""
    var age: int = 0

    # Constructor
    def init(n: string, a: int):
        this.name = n
        this.age = a

    # Method
    def greet():
        print("Hello, I am " + this.name)
```

### Using a Class
```redline
var p: Person = Person("Alice", 30)
p.greet()
```

*   **`this`**: Use `this` inside methods to access member variables and other methods.
*   **`init`**: A special method that acts as the constructor.

## 8. Modules

You can split your code into multiple files using modules.

### Importing a Module
Use the `import` keyword to include another `.rl` file.

```redline
import "math_utils.rl"

val result: int = add(10, 5)
```

### Public Visibility
By default, functions and variables are private to the module. Use the `pub` keyword to make them accessible to other modules.

```redline
# math_utils.rl
pub def add(a: int, b: int) -> int:
    return a + b
```

## 9. C++ Interoperability

REDLINE is designed to work seamlessly with C++. You can compile REDLINE code into a library and use it in your C++ projects.

### Building a Library
```bash
python redline.py lib my_library.rl
```
This generates `my_library.hpp` and `my_library.o`.

### Using in C++
```cpp
#include "my_library.hpp"

int main() {
    int result = rl::add(10, 20); // Call REDLINE function
    return 0;
}
```

## 10. Input & Output

The built-in `print` and `input` commands handle communication with the console.

## 11. Type Conversion

REDLINE provides built-in functions to convert values between different types.

*   `to_int(value)`
*   `to_float(value)`
*   `to_string(value)`

## 12. The Compiler Pipeline

1.  **.rl File**: You write your logic here.
2.  **Lexer (`lexer.rs`)**: Breaks your code into "tokens".
3.  **Parser (`parser.rs`)**: Builds a structured representation (AST).
4.  **Code Generator (`codegen.rs`)**: Translates the AST into C++ code.
5.  **G++ Compiler**: Turns the C++ into a runnable executable.

## 13. Standard Library

### rl_io.hpp
- `print()`: Print values to stdout.
- `input()`: Read a string from stdin.

### rl_math.hpp
- Common math functions (`sqrt`, `pow`, `sin`, etc.) and constants (`PI`, `E`).

### rl_stdlib.hpp
- `len()`: Returns the size of a list.
- `append()`: Appends an element to a list.

This documentation is a work in progress. If you find any errors or want to improve it, please feel free to open an issue or pull request!
