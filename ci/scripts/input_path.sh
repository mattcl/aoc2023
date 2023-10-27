#!/bin/sh
set -e

# This script is used by the ci pipeline to extract our inputs for use in the
# benchmarking and checking of solutions.

# The specification says that AOC_DAY will be set from 1-25, so make sure that
# var is set.
if [ -z ${AOC_DAY+x} ]; then
    echo "env var AOC_DAY must be set"
    exit 1
fi

# We need to zero-pad the day to 3 digits to properly match our inputs.
padded=$(printf "%03d" "$AOC_DAY")
search=$(find . -type d -name "day-${padded}*" -print -quit)

# The specification says that if an input does not exist for a given day, we
# need to exit with a nonzero code.
if [ -z "$search" ]; then
    echo "no input for day ${AOC_DAY}"
    exit 1
fi

expected="${search}/input.txt"

# The specification says that if an input does not exist for a given day, we
# need to exit with a nonzero code.
if [ -f "$expected" ]; then
    echo "$expected"
else
    echo "no input for day ${AOC_DAY}"
    exit 1
fi
