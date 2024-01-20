mod cli;
mod filters;
mod operations;
mod rename;
use clap::Parser;
fn main() {
    let cli = cli::Cli::parse();
    // println!("{:#?}", cli);
    let err = rename::rename(cli);
    if let Err(e) = err {
        eprintln!("{:#?}", e);
    }
}
