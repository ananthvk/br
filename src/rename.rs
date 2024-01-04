use crate::cli::Cli;
use colored::*;
use regex::Regex;
use std::{fs, path::PathBuf};

// TODO: Handle errors instead of unwrapping
// Add option to ignore or halt on error
pub fn rename(config: Cli) {
    let target_directory = match config.dir.as_deref() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    let paths = fs::read_dir(target_directory).unwrap();
    let re = Regex::new(&config.search_expr).unwrap();
    for entry in paths {
        let f = entry
            .unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap();
        if re.is_match(&f) {
            let regex_match = re.find(&f).unwrap();
            let prefix = &f[..regex_match.range().start];
            let suffix = &f[regex_match.range().end..];
            print!("{}{}{}", prefix, regex_match.as_str().red().bold(), suffix);
            let replaced = re.replace_all(&f, &config.replace_expr);
            println!(" => {}", replaced.white().bold());
            if config.execute {
                fs::rename(&f, &*replaced).unwrap();
            }
        }
    }
}
