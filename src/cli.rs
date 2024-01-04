use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="br")]
#[command(author="ananthvk https://github.com/ananthvk")]
#[command(about="A file utility to rename a lot of files")]
pub struct Cli{

    /// Expression for matching files
    #[arg(value_name = "SEARCH")]
    search_expr: String,

    /// Replacement expression (can use named groups with ${1})
    #[arg(value_name = "REPLACE")]
    replace_expr: String,

    /// Rename files in the specified directory (default: current directory)
    #[arg(value_name = "DIRECTORY")]
    dir: Option<PathBuf>
}