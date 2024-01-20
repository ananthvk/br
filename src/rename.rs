use crate::cli::Cli;
use crate::filters::*;
use crate::operations::*;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

// TODO: Handle errors instead of unwrapping
// Add option to ignore or halt on error
// Also if a file exists, give an option to overwrite or not
// By default do not overwrite

fn get_dir_contents(target_directory: &PathBuf) -> Box<dyn Iterator<Item = String>> {
    let entries = fs::read_dir(target_directory)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap());
    Box::new(entries)
}

fn rename_entry(from: &str, to: &str) {
    fs::rename(from, to).unwrap();
}

pub fn rename(config: Cli) {
    let target_directory = match config.dir.as_deref() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    let target_directory = fs::canonicalize(&target_directory).unwrap();

    std::env::set_current_dir(&target_directory).unwrap();
    let files = get_dir_contents(&target_directory);
    let mut filters: Vec<Regex> = Vec::new();
    let re = Regex::new(&config.search_expr).unwrap();

    if config.starts_with_list.len() > 0 {
        let startswith: Vec<&str> = config.starts_with_list.iter().map(AsRef::as_ref).collect();
        filters.push(generate_startswith_filter(&startswith).unwrap())
    }

    'outer: for file in files {
        for filter in &filters {
            if !filter.is_match(&file) {
                continue 'outer;
            }
        }
        if re.is_match(&file) {
            let replacement = regex_replace(&file, &config.replace_expr, &re);
            match_display(&replacement);
            if config.execute {
                if !config.noconfirm {
                    if ask_confirmation() {
                        rename_entry(&file, &replacement.replaced);
                    }
                } else {
                    rename_entry(&file, &replacement.replaced);
                }
            }
        }
    }
}
