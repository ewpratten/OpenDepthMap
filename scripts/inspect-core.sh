#! /bin/bash

set -e
cargo build
LD_LIBRARY_PATH="$(pwd)/libodm/dist:$LD_LIBRARY_PATH" gdb ./target/debug/odm