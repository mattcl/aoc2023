#!/bin/bash
set -e

# This bit of crazy generates the new crate and adds the relevant imports and
# macro lines to the cli and benchmarking crates

cargo generate --path ./template --lib --name "$2" -d day="$1"

printf -v DAY "%03d" "$1"

EXPECTED="$2"
DESIRED="day-${DAY}-$2"
CRATE_NAME=$(cat "$EXPECTED/crate_ref")
STRUCT_NAME=$(cat "$EXPECTED/name_ref")

rm "$EXPECTED/crate_ref"
rm "$EXPECTED/name_ref"

echo "Renaming $EXPECTED to $DESIRED"
mv "$EXPECTED" "$DESIRED"

IMPORT_REPLACEMENT="use ${CRATE_NAME}::${STRUCT_NAME};\\n// import_marker"

# ====== cli

echo "Appending $EXPECTED to cli"
echo "$EXPECTED = { path = \"../$DESIRED\" }" >> aoc-cli/Cargo.toml

echo "Modifying cli.rs"
sed -i "s#// import_marker#$IMPORT_REPLACEMENT#" aoc-cli/src/cli.rs

COMMAND_REPLACEMENT="(${STRUCT_NAME}, $1),\\n    // command_marker"
sed -i "s#// command_marker#$COMMAND_REPLACEMENT#" aoc-cli/src/cli.rs

# ====== benchmarks

echo "Appending $EXPECTED to benchmarks"
echo "$EXPECTED = { path = \"../$DESIRED\" }" >> aoc-benchmarking/Cargo.toml

echo "Modifying bench_main.rs"
sed -i "s#// import_marker#$IMPORT_REPLACEMENT#" aoc-benchmarking/benches/bench_main.rs

# yeah, I'm definitely not proud of this insanity
BENCH_REPLACEMENT=$(cat <<EOF
(
        day_${DAY},
        "../${DESIRED}/input.txt",
        ${STRUCT_NAME},
        "Part 1",
        "Part 2"
    ),
    // bench_marker
EOF
)
BENCH_REPLACEMENT="${BENCH_REPLACEMENT//\\/\\\\}"
BENCH_REPLACEMENT="${BENCH_REPLACEMENT//\//\\/}"
BENCH_REPLACEMENT="${BENCH_REPLACEMENT//&/\\&}"
BENCH_REPLACEMENT="${BENCH_REPLACEMENT//$'\n'/\\n}"
sed -i "s#// bench_marker#$BENCH_REPLACEMENT#" aoc-benchmarking/benches/bench_main.rs
