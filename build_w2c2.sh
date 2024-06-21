#!/bin/sh

set -e

BUILD_DIR=./w2c2
BINARY_PATH=$BUILD_DIR/build/w2c2/w2c2

if [ -e $BINARY_PATH ]; then
    exit 0
fi

rm -rf $BUILD_DIR
git clone --recursive https://github.com/turbolent/w2c2 $BUILD_DIR
cd $BUILD_DIR

cmake -B build
cmake --build build

# w2c2 binary is at w2c2/build/w2c2/w2c2