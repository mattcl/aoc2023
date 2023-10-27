# generate the boilerplate for a new day's problem `just new 1 foo-bar-baz`
new DAY NAME:
    scripts/new.sh {{DAY}} {{NAME}}

# run all integration tests
test:
    cargo test --release -- --ignored

# run benchmarks for a given padded day `just bench 001`
bench DAY:
    cargo bench -p aoc-benchmarking -- {{DAY}}

# run all benchmarks
bench-all:
    cargo bench -p aoc-benchmarking

# makes a flamegraph for the given day
flame DAY:
    scripts/flame.sh {{DAY}}

# builds the cli
build-cli:
    cargo build -p aoc-cli --release
