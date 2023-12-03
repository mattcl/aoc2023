use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use aoc_plumbing::Problem;
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, shells::Zsh};
use cube_conundrum::CubeConundrum;
use gear_ratios::GearRatios;
use trebuchet::Trebuchet;
// import_marker

// I'm not proud
macro_rules! generate_cli {
    ($(($name:ident, $day:literal)),* $(,)?) => {
        /// Advent of Code solutions for 2023
        #[derive(Parser)]
        #[command(name = "aoc", author, version)]
        #[command(help_template = "\
            {name} {version} by {author}
            {about-section}\n{usage-heading} {usage}\n\n{all-args}{tab}")]
        pub(crate) struct Cli {
            #[command(subcommand)]
            pub command: Commands,
        }

        impl Cli {
            pub fn run() -> Result<()> {
                let command = Self::parse().command;
                command.run()
            }
        }

        #[derive(Subcommand)]
        pub(crate) enum Commands {
            $(
            #[command(about = $name::problem_label(), long_about = $name::long_description(), display_order = $day)]
            $name(Solver<$name>),
            )*

            #[command(display_order = 30)]
            Run(Run),

            #[command(display_order = 31)]
            GenerateCompletions(GenerateCompletions),
        }

        impl Commands {
            pub fn run(&self) -> Result<()> {
                match self {
                    Self::GenerateCompletions(cmd) => cmd.run(),
                    Self::Run(cmd) => cmd.run(),
                    $(
                    Self::$name(cmd) => cmd.run(),
                    )*
                }
            }
        }

        /// Run the solution for a specified day with a specified input.
        ///
        /// The day must be implemented and the specified input must exist.
        #[derive(Args)]
        pub(crate) struct Run {
            /// The day to run.
            ///
            /// This may be specified instead by setting the `AOC_DAY` env var.
            /// An explicitly passed value will take precendence over the env
            /// var.
            #[clap(env = "AOC_DAY")]
            day: usize,

            /// The path to the input for this solution.
            ///
            /// This may be specified instead by setting the `AOC_INPUT` env
            /// var. An explicitly passed value will take precendence over the
            /// env var.
            #[clap(env = "AOC_INPUT")]
            input: PathBuf,

            /// Display the output as json.
            ///
            /// This may be specified instead by setting the `AOC_INPUT` env
            /// var to `true`. If the flag is passed, on the command line, it
            /// will take precendence over the env var.
            #[clap(short, long, env = "AOC_JSON")]
            json: bool,
        }

        impl Run {
            pub fn run(&self) -> Result<()> {
                match self.day {
                    $(
                    $day => _run::<$name>(&self.input, self.json),
                    )*
                    _ => {
                        if self.json {
                            println!("\"not implemented\"");
                        } else {
                            println!("not implemented");
                        }
                        Ok(())
                    }
                }
            }
        }
    };
}

#[derive(Args)]
pub(crate) struct Solver<T>
where
    T: Problem,
{
    /// The path to the input for this solution.
    input: PathBuf,

    /// Display the output as json.
    #[clap(short, long)]
    json: bool,

    #[clap(skip)]
    _phantom: PhantomData<T>,
}

impl<T> Solver<T>
where
    T: Problem,
    <T as Problem>::ProblemError: Into<anyhow::Error>,
{
    pub fn run(&self) -> Result<()> {
        _run::<T>(&self.input, self.json)
    }
}

fn _run<T>(input_file: &Path, json: bool) -> Result<()>
where
    T: Problem,
    <T as Problem>::ProblemError: Into<anyhow::Error>,
{
    let input = std::fs::read_to_string(input_file).context("Could not read input file")?;

    let solution = T::solve(&input)
        .map_err(Into::<anyhow::Error>::into)
        .context("Failed to solve")?;

    if json {
        println!("{}", serde_json::to_string(&solution)?);
    } else {
        println!("{}", solution);
    }

    Ok(())
}

/// Generate zsh completions
#[derive(Debug, Args)]
pub struct GenerateCompletions;

impl GenerateCompletions {
    fn run(&self) -> Result<()> {
        generate(Zsh, &mut Cli::command(), "aoc", &mut std::io::stdout());
        Ok(())
    }
}

generate_cli! {
    (Trebuchet, 1),
    (CubeConundrum, 2),
    (GearRatios, 3),
    // command_marker
}
