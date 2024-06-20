#!/bin/sh

set -e

BUILD_DIR=./wabt

rm -rf $BUILD_DIR
git clone --recursive https://github.com/WebAssembly/wabt $BUILD_DIR
cd $BUILD_DIR

mkdir build
cd build
cmake ..
cmake --build .

# wasm2c binary is at wabt/build/wasm2c