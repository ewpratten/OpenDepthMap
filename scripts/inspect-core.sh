#! /bin/bash

set -e

LD_LIBRARY_PATH="$(pwd)/libodm/lib:$LD_LIBRARY_PATH" gdb ./target/debug/odm