# 2023 Advent of Code Solutions

This workspace provides an executable named `aoc` that produces solutions for a
given day and input. This is done either through the `run` subcommand or via a
day-specific subcommand.

## Developing

### Prerequisite

1. git
2. rust >=1.66 (1.73 preferred)
3. [cargo-generate](https://crates.io/crates/cargo-generate)
4. [just](https://github.com/casey/just#packages)
5. [cargo-flamegraph](https://crates.io/crates/flamegraph) (optional)
5. [cargo-watch](https://crates.io/crates/cargo-watch) (optional)


### Working on a new day's problem

This workspace utilizes a separate crate for each day's solution, with an
additional set of glue crates providing cli functionality, benchmarking, and
utility interfaces.

There is a `just` command for creating a new crate for a given day. The title
MUST be hyphenated (a problem titled "Calorie Counting" would be specified as
"calorie-counting").

Example:

```
just new 1 calorie-counting
```

This will produce a directory named `day-001-calorie-counting`, exposing a
workspace crate named `calorie-counting`. The `new.sh` script will make the
necessary modifications to include this day's solution in the CLI, as well as
adding the benchmark macro calls to the benchmarking crate.

The real input is stored in each day's workspace crate. Example inputs are
embedded in the source files.


### Building the cli

```
cargo build -p aoc-cli --release

# or, if you have just installed:
just build-cli
```


### Running tests against real inputs

The tests with real inputs are marked as `#[ignore]`, so they will not run by
default. You can run these by running

```
# tests against real inputs should be run in release mode
cargo test --release -- --ignored

# or, if you have just installed:
just test
```


### Running benchmarks against a given day

The benchmarks are defined in the `aoc-benchmarking` workspace crate, and
contain the three-digit zero-padded day, but you can match on any part of the
benchmark name.

To run benchmarks for a particular day:

```
cargo bench -p aoc-benchmarking -- DAY

# or, if you have just installed:
just bench DAY

# Example
just bench 001
```


### Running all benchmarks

The entire benchmark suite (which includes an overall runtime) can be run via:

```
cargo bench -p aoc-benchmarking

# or, if you have just installed:
just bench-all
```

### Additional

See the `justfile` for additional functionality like flamegraphs.
