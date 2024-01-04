use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "br")]
#[command(author = "ananthvk https://github.com/ananthvk")]
#[command(about = "A file utility to rename a lot of files")]
pub struct Cli {
    /// Expression for matching files
    #[arg(value_name = "SEARCH")]
    pub search_expr: String,

    /// Replacement expression (can use named groups with ${1})
    #[arg(value_name = "REPLACE")]
    pub replace_expr: String,

    /// Rename files in the specified directory (default: current directory)
    #[arg(value_name = "DIRECTORY")]
    pub dir: Option<PathBuf>,

    /// Execute the operations, without this flag only a dry run is produced
    #[arg(short = 'x', long = "execute", default_value_t=false)]
    pub execute: bool,
    
    /// Does not ask any "are you sure?" confirmation questions
    #[arg(long="noconfirm", default_value_t=false)]
    pub noconfirm: bool
}
