# 🟥 REDLINE

**C++ but Simplified.**

REDLINE is a high-performance, transpiled systems programming language designed to combine the **readability of Python** with the **raw speed of C++**. It compiles your REDLINE code (`.rl`) into optimized C++, which is then compiled into a native executable.

## 🚀 Key Features

*   **Python-like Syntax**: Clean, indentation-based structure. No semicolons or curly braces required.
*   **C++ Performance**: Transpiles directly to C++, leveraging the full power of the G++ compiler.
*   **Object-Oriented**: Full support for classes, methods, and constructors.
*   **Modular**: Organize your code with `import` and `pub`.
*   **C++ Interop**: Easily use REDLINE code in your existing C++ projects.
*   **Rich Type System**: Supports `int`, `float`, `string`, `bool`, `void`, and `list[T]`.
*   **Modern Tooling**: Built-in Lexer, Parser, and Code Generator written in Rust for speed and safety.

## 📦 Installation

### Prerequisites
*   **Rust**: To build the REDLINE compiler core.
*   **G++**: To compile the generated C++ code.
*   **Python 3**: To run the build script wrapper.

### Building the Compiler
1.  Clone the repository:
    ```bash
    git clone https://github.com/yourusername/REDLINE.git
    cd REDLINE
    ```
2.  Initialize the compiler core:
    ```bash
    python redline.py init
    ```

## 🛠️ Usage

### Compiling and Running a File
The easiest way to compile and run a REDLINE file is using the `redline.py` wrapper script.

```bash
python redline.py build <path_to_file.rl>
```

**Example:**
```bash
python redline.py build examples/v0.6_tests/class_test.rl
```

### C++ Interoperability (Auto-Discovery)
You can write a C++ `main.cpp` that includes REDLINE headers, and the build script will automatically find and compile the dependencies.

**main.cpp:**
```cpp
#include "my_lib.hpp" // Corresponds to my_lib.rl

int main() {
    rl::my_function();
    return 0;
}
```

**Build Command:**
```bash
python redline.py build main.cpp
```
The script will detect the `#include "my_lib.hpp"`, find `my_lib.rl`, compile it, and link everything together!

### Building a Library Manually
If you prefer manual control, you can compile a REDLINE module into a static library (`.o`) and header (`.hpp`).

```bash
python redline.py lib my_library.rl
```

## 📝 Example Code

**Classes & Objects:**
```redline
class Person:
    var name: string = ""
    var age: int = 0

    # Constructor
    def init(n: string, a: int):
        this.name = n
        this.age = a

    def greet():
        print("Hello, I am " + this.name)

var p: Person = Person("Alice", 30)
p.greet()
```

**List Manipulation:**
```redline
# Declare a list of integers
var my_list: list[int] = [10, 20, 30]

# Add an element
append(my_list, 40)

# Change an element
my_list[1] = 99

# Print the list
for i in 0..len(my_list):
    print(my_list[i])
```

## 🗺️ Roadmap

See [ROADMAP.md](ROADMAP.md) for the full list of planned features and future goals.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
