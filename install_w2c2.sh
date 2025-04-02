#!/bin/sh

set -e

BIN_PATH=$HOME/.cargo/bin
if [ ! -d $BIN_PATH ]; then
    echo "non-default cargo bin path"
    exit 1
fi
if [ -e $BIN_PATH/w2c2 ]; then
    echo "already installed"
    exit 0
fi

BUILD_DIR=$(mktemp)
BINARY_PATH=$BUILD_DIR/build/w2c2/w2c2

rm -rf $BUILD_DIR
git clone --recursive https://github.com/vivianjeng/w2c2 $BUILD_DIR
cd $BUILD_DIR

cmake -B build
cmake --build build

cp $BINARY_PATH $BIN_PATH

# w2c2 binary is at w2c2/build/w2c2/w2c2