#!/bin/bash

set -e

mkdir -p build

cargo run -- --input examples/test.su

nasm -f elf64 examples/test.s -o build/main.o

for file in stdlib/*.s; do
    filename=$(basename "$file" .s)

    nasm -f elf64 "$file" -o "build/$filename.o"
done

ld build/*.o -o build/main
