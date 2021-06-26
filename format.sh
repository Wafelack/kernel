#!/usr/bin/env sh
clang-format -i --style GNU $(find ./src -name '*.c' -or -name '*.h' -type f) 
