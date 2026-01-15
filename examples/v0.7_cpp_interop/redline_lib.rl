# examples/v0.7_cpp_interop/redline_lib.rl

# A simple public function exposed to C++
pub def add_numbers(a: int, b: int) -> int:
    return a + b

# A class exposed to C++
pub class Greeter:
    var prefix: string = "Hello"

    def init(p: string):
        this.prefix = p

    def greet(name: string):
        print(this.prefix + ", " + name + "!")
