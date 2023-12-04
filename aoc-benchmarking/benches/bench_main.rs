use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use cube_conundrum::CubeConundrum;
use gear_ratios::GearRatios;
use scratchcards::Scratchcards;
use trebuchet::Trebuchet;
// import_marker

criterion_main! {
    benches
}

aoc_benches! {
    5,
    (
        day_001,
        "../day-001-trebuchet/input.txt",
        Trebuchet,
        "Combined (including parsing)"
    ),
    (
        day_002,
        "../day-002-cube-conundrum/input.txt",
        CubeConundrum,
        "Combined (including parsing)"
    ),
    (
        day_003,
        "../day-003-gear-ratios/input.txt",
        GearRatios,
        "Combined (including parsing)"
    ),
    (
        day_004,
        "../day-004-scratchcards/input.txt",
        Scratchcards,
        "Combined (including parsing)"
    ),
    // bench_marker
}
