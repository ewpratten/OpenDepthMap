#! /bin/bash
# This is a small wrapper script to handle configuring your library path before executing ODM

set -e

echo "Enabling core dumps"
ulimit -c unlimited

echo "Starting cargo with new flags"
LD_LIBRARY_PATH="$(pwd)/libodm/dist:$LD_LIBRARY_PATH" cargo run $@