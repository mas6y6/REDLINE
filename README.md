# 🟥 REDLINE

**C++ but Simplified.**

REDLINE is a high-performance, transpiled systems programming language designed to combine the **readability of Python** with the **raw speed of C++**. It compiles your REDLINE code (`.rl`) into optimized C++, which is then compiled into a native executable.

## 🚀 Key Features

*   **Python-like Syntax**: Clean, indentation-based structure. No semicolons or curly braces required.
*   **C++ Performance**: Transpiles directly to C++, leveraging the full power of the G++ compiler.
*   **Strict Typing**: Catch errors at compile time with a strong type system (`int`, `float`, `string`, `bool`).
*   **Modern Tooling**: Built-in Lexer, Parser, and Code Generator written in Rust for speed and safety.
*   **Standard Library**: Includes fast I/O and math functions out of the box.

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
2.  Build the core compiler (Rust):
    ```bash
    cd redline-core
    cargo build --release
    cd ..
    ```

## 🛠️ Usage

### Compiling and Running a File
The easiest way to compile and run a REDLINE file is using the `redline.py` wrapper script.

```bash
python redline.py <path_to_file.rl>
```

**Example:**
```bash
python redline.py examples/hello_world.rl
```

### Manual Compilation
If you prefer to run the steps manually:
1.  **Transpile**: Run the Rust core to generate `output.cpp`.
    ```bash
    ./redline-core/target/release/redline-core <file.rl> > output.cpp
    ```
2.  **Compile**: Use G++ to create the executable.
    ```bash
    g++ output.cpp -o program
    ```
3.  **Run**: Execute your program.
    ```bash
    ./program
    ```

## 📝 Example Code

**Hello World:**
```redline
print("Hello, Redline!")
```

**Fibonacci Sequence:**
```redline
def fib(n: int) -> int:
    if n <= 1:
        return n
    else:
        return fib(n - 1) + fib(n - 2)

for i in 0..10:
    print(fib(i))
```

**User Input:**
```redline
val name: string = input("What is your name? ")
print("Welcome, " + name)
```

## 🗺️ Roadmap

*   [x] Variables & Types (`int`, `float`, `string`, `bool`)
*   [x] Functions & Recursion
*   [x] Control Flow (`if`, `else`, `while`, `for`)
*   [x] Standard I/O (`print`, `input`)
*   [ ] Arrays & Lists
*   [ ] Classes & Structs
*   [ ] Module System (`import`)

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
