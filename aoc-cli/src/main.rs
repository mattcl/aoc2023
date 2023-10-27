mod cli;

pub fn main() -> Result<(), anyhow::Error> {
    cli::Cli::run()
}
