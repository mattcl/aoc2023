# 2023 Advent of Code Solutions

This workspace provides an executable named `aoc` that produces solutions for a
given day and input. This is done either through the `run` subcommand or via a
day-specific subcommand.

## Current runtime ~25.5 ms

```
+-------------------------------------------------------------+
| Problem                            Time (ms)   % Total Time |
+=============================================================+
| 001 trebuchet                        0.06723          0.263 |
| 002 cube conundrum                   0.01480          0.058 |
| 003 gear ratios                      0.08415          0.330 |
| 004 scratchcards                     0.03774          0.148 |
| 005 you give a seed a fertilizer     0.01162          0.046 |
| 006 wait for it                      0.00027          0.001 |
| 007 camel cards                      0.10829          0.424 |
| 008 haunted wasteland                0.32761          1.284 |
| 009 mirage maintenance               0.04608          0.181 |
| 010 pipe maze                        0.22459          0.880 |
| 011 cosmic expansion                 0.01197          0.047 |
| 012 hot springs                      0.56546          2.216 |
| 013 point of incidence               0.03004          0.118 |
| 014 parabolic reflector dish         2.48077          9.722 |
| 015 lens library                     0.13207          0.518 |
| 016 the floor will be lava           2.86935         11.245 |
| 017 clumsy crucible                  7.12009         27.904 |
| 018 lavaduct lagoon                  0.02418          0.095 |
| 019 aplenty                          0.11363          0.445 |
| 020 pulse propagation                1.66637          6.531 |
| 021 step counter                     0.55691          2.183 |
| 022 sand slabs                       1.33472          5.231 |
| 023 a long walk                      4.09091         16.033 |
| 024 never tell me the odds           0.25839          1.013 |
| 025 snowverload                      3.33897         13.086 |
| Total                               25.51623        100.000 |
+-------------------------------------------------------------+
```


## Developing

**A note about compiling this yourself:** I have my
[`aoc-std`](https://github.com/mattcl/aoc-std.git) crate published to a private
registry that will likely be inaccessible for you. You can swap that out for the
version specified by a direct link to the git repo. There's a comment in the
top-level (workspace) `Cargo.toml` explaining how to do this, but it will
currently not work because of a bug with cargo that does not fall back to git
sources for subcrates if a registry is specified.


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
