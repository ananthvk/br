mod cli;
mod rename;
use clap::Parser;
fn main() {
    let cli = cli::Cli::parse();
    rename::rename(cli);
}
