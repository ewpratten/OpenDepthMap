#! /bin/bash
# This is a small wrapper script to handle configuring your library path before executing ODM

set -e

echo "Enabling core dumps"
ulimit -c unlimited

echo "Building deps"
cargo build

echo "Starting python with new flags"
LD_LIBRARY_PATH="$(pwd)/libodm/dist:$LD_LIBRARY_PATH" python3 odm $@