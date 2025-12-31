// stdlib/rl_io.hpp
#ifndef RL_IO_H
#define RL_IO_H

#include <iostream>
#include <string>

namespace rl {
    // Using fast I/O
    inline void print(const std::string& msg) {
        std::ios_base::sync_with_stdio(false); // Speed boost
        std::cout << msg << "\n";
    }

    // Overload for string literals to prevent implicit bool conversion
    inline void print(const char* msg) {
        std::ios_base::sync_with_stdio(false);
        std::cout << msg << "\n";
    }

    inline void print(int val) {
        std::cout << val << "\n";
    }

    inline void print(double val) {
        std::cout << val << "\n";
    }

    inline void print(bool val) {
        std::cout << (val ? "true" : "false") << "\n";
    }

    inline std::string input(const std::string& prompt = "") {
        if (!prompt.empty()) {
            std::cout << prompt;
        }
        std::string line;
        std::getline(std::cin, line);
        return line;
    }
}
#endif
