#!/bin/bash

set -euxo pipefail

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

riscv64-unknown-elf-gcc -ggdb3 -fdebug-prefix-map=$(pwd)=/hifive1 -c -mabi=ilp32 -march=rv32imac flash.S -o bin/flash.o
riscv64-unknown-elf-ar crs bin/flash.a bin/flash.o

rm bin/flash.o
