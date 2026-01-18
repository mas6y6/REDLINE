# 🟥 REDLINE v1.0.0 Documentation

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
```

## 2. Data Types

REDLINE is strictly typed, meaning the compiler ensures you don't accidentally treat a number like a word.

*   `int`: Whole numbers (e.g., `10`, `-5`).
*   `float`: Decimal numbers (e.g., `10.5`, `3.14`).
*   `string`: Text wrapped in double quotes (e.g., `"Redline"`).
*   `bool`: Logical values (`true` or `false`).
*   `void`: Represents the absence of a value (used for function return types).
*   `list[T]`: A dynamic array of elements of type `T`.
*   `dict[K, V]`: A dictionary (hash map) with keys of type `K` and values of type `V`.

## 3. Functions

Functions in REDLINE use a modern "Arrow" syntax. If a function does not return a value, the return type can be omitted (defaults to `void`).

```redline
def greet(name: string):
    print("Hello, " + name)
```

### Function Overloading
You can define multiple functions with the same name, as long as they have different parameter types. The compiler will choose the correct one based on the arguments you provide.

```redline
def add(a: int, b: int) -> int:
    return a + b

def add(a: string, b: string) -> string:
    return a + b

val r1: int = add(10, 20)
val r2: string = add("Hello, ", "World!")
```

## 4. Control Flow

REDLINE uses `if`/`else` for logic and `while`/`for` for loops.

### If/Else
```redline
if health <= 0:
    print("Game Over")
else:
    print("Still kicking!")
```

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

### Loop Control
You can control loop execution with `break` and `continue`.
*   `continue`: Skips the rest of the current iteration and proceeds to the next one.
*   `break`: Exits the loop entirely.

```redline
for i in 0..10:
    if i == 2:
        continue # Skip printing 2
    if i == 5:
        break # Stop the loop at 5
    print(i)
# Output: 0, 1, 3, 4
```

## 5. Data Structures

### Lists
A dynamic array of elements.
```redline
var my_list: list[int] = [10, 20, 30]
val first: int = my_list[0]
my_list[1] = 99
append(my_list, 40)
```

### Dictionaries
A collection of key-value pairs.
```redline
var scores: dict[string, int] = {
    "Alice": 100,
    "Bob": 85
}
val alice_score: int = scores["Alice"]
scores["Bob"] = 90
```

## 6. Strings & F-Strings

REDLINE supports standard string concatenation. For more complex formatting, you can use f-strings.

```redline
val name: string = "Redline"
val version: float = 1.0

# f-string interpolation
val message: string = f"Welcome to {name} v{version}!"
print(message) # Welcome to Redline v1.0!
```

## 7. Classes & Objects

REDLINE supports Object-Oriented Programming (OOP) with classes and automatic memory management.

### Defining a Class
```redline
class Person:
    var name: string = ""
    var age: int = 0

    def init(n: string, a: int):
        this.name = n
        this.age = a

    def greet():
        print(f"Hello, I am {this.name}")
```

### Using a Class
Use the `new` keyword to create an object instance on the heap. Memory is managed automatically.
```redline
var p: Person = new Person("Alice", 30)
p.greet()
```

## 8. Error Handling

REDLINE uses `try` and `catch` blocks to handle runtime errors.
```redline
try:
    val content: string = read_file("missing.txt")
catch e:
    print("An error occurred!")
```

## 9. Modules & Projects

### Modules
Split your code into multiple files using `import`. Use the `pub` keyword to make functions and classes accessible from other modules.
```redline
# utils.rl
pub def my_util():
    print("Utility function!")

# main.rl
import "utils.rl"
my_util()
```

### Projects (`RedConfig.toml`)
For larger projects, create a `RedConfig.toml` file. This allows you to define your project's entry point and output directory, and build with a simple `redline build` command.
```toml
[project]
name = "MyAwesomeProject"
entry_point = "src/main.rl"
output_dir = "bin"
```

## 10. Standard Library

### System (`rl_stdlib.hpp`)
*   `args: list[string]`: A global list containing command-line arguments.
*   `len(list)`: Returns the number of elements in a list.
*   `append(list, value)`: Adds an element to the end of a list.
*   `sort(list)` / `reverse(list)` / `find(list, value)`
*   `to_string(value)` / `to_int(value)` / `to_float(value)`

### I/O (`rl_io.hpp`)
*   `print(value)`: Print to stdout.
*   `input(prompt)`: Read a string from stdin.

### File System (`rl_file.hpp`)
*   `read_file(path) -> string`: Reads a file's content. Throws on error.
*   `write_file(path, content)`: Writes content to a file. Throws on error.
*   `exists(path) -> bool`: Checks if a file or directory exists.
*   `mkdir(path)`: Creates a new directory.
*   `remove(path)`: Deletes a file or directory.
*   `list_dir(path) -> list[string]`: Returns a list of names in a directory.

### Time (`rl_time.hpp`)
*   `time() -> float`: Returns the current Unix timestamp.
*   `sleep(seconds: float)`: Pauses the program.

### Random (`rl_random.hpp`)
*   `random_int(min: int, max: int) -> int`: Returns a random integer in the specified range.
*   `random_float() -> float`: Returns a random float between 0.0 and 1.0.
