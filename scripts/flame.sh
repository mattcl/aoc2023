#!/bin/bash
set -e

printf -v DAY "%03d" $1
INPUT_DIR=$(find . -name "day-${DAY}-*")

cargo build -p aoc-cli
cargo flamegraph -o "day-${DAY}_flamegraph.svg" -b aoc --dev -- run $1 "${INPUT_DIR}/input.txt"
