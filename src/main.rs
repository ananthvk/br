mod cli;
mod rename;
mod filters;
mod operations;
use clap::Parser;
fn main() {
    let cli = cli::Cli::parse();
    println!("{:#?}", cli);
    rename::rename(cli);
}
