#!/bin/sh

set -e

# OUT_DIR is specified by the rust build environment
if [ -z $OUT_DIR ]; then
    echo "OUT_DIR not specified"
    exit 1
fi
BUILD_DIR=$OUT_DIR/w2c2
BINARY_PATH=$BUILD_DIR/build/w2c2/w2c2

if [ -e $BINARY_PATH ]; then
    exit 0
fi

rm -rf $BUILD_DIR
git clone --recursive https://github.com/vivianjeng/w2c2 $BUILD_DIR

# if any argument is supplied just clone (to access the headers)
if [ ! -z $1 ]; then
    exit 0
fi

cd $BUILD_DIR

cmake -B build
cmake --build build

# w2c2 binary is at w2c2/build/w2c2/w2c2