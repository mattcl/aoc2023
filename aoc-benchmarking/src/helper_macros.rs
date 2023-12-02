#[macro_export]
macro_rules! aoc_bench {
    // "standard" solution with two distinct parts
    ($name:ident, $input:literal, $problem:ty, $part1_desc:literal, $part2_desc:literal) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(<$problem>::problem_label());
            let input = std::fs::read_to_string($input).expect("Could not load input");

            group.bench_function($part1_desc, |b| {
                let mut problem = <$problem>::instance(&input).expect("Could not parse input");
                b.iter(|| problem.part_one().expect("Failed to solve part one"))
            });
            group.bench_function($part2_desc, |b| {
                let mut problem = <$problem>::instance(&input).expect("Could not parse input");
                b.iter(|| problem.part_two().expect("Failed to solve part two"))
            });
            group.bench_function("Combined (including parsing)", |b| {
                b.iter(|| <$problem>::solve(&input).expect("Failed to solve"))
            });
            group.finish();
        }
    };
    // combined solution
    ($name:ident, $input:literal, $problem:ty, $combined_desc:literal) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(<$problem>::problem_label());
            let input = std::fs::read_to_string($input).expect("Could not load input");

            group.bench_function($combined_desc, |b| {
                b.iter(|| <$problem>::solve(&input).expect("Failed to solve"))
            });
            group.finish();
        }
    };
}

#[macro_export]
macro_rules! aoc_benches {
    ($comb_seconds:literal, $(($name:ident, $input:literal, $problem:ty, $($description:literal),+)),* $(,)?) => {
        use std::time::Duration;

        use criterion::{criterion_group, Criterion};
        use aoc_plumbing::Problem;

        $(
            aoc_benchmarking::aoc_bench!($name, $input, $problem, $($description),+);
        )*

        pub fn aoc_combined(c: &mut Criterion) {
            let mut group = c.benchmark_group("Advent of Code");
            group.measurement_time(Duration::new($comb_seconds, 0));
            group.bench_function("Total runtime for all solutions, including parsing", |b| {
                b.iter(|| {
                    $(
                        let input = std::fs::read_to_string($input).expect("Failed to open file");
                        <$problem>::solve(&input).expect("Failed to solve");
                    )*
                })
            });
            group.finish();
        }

        criterion_group!(benches, $($name,)* aoc_combined);
    };
    ($(($name:ident, $input:literal, $problem:ty, $($description:literal),+)),* $(,)?) => {
        aoc_benches!{
            10, $( ($name, $input, $problem, $($description),+)),*
        }
    };
}
