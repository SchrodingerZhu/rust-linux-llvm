#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
rm -rf "$SCRIPT_DIR/libc" "$SCRIPT_DIR/cmake" "$SCRIPT_DIR/compiler-rt"
git clone https://github.com/llvm/llvm-project.git --depth=1
cp -r llvm-project/libc llvm-project/compiler-rt llvm-project/cmake $SCRIPT_DIR
cp llvm-project/llvm/cmake/modules/* "$SCRIPT_DIR/cmake/Modules/"

pushd llvm-project
git rev-parse HEAD > $SCRIPT_DIR/.git-hash
popd

rm -rf llvm-project