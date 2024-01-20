use crate::cli::Cli;
use crate::filters::*;
use crate::operations::*;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// Add option to ignore or halt on error
// Also if a file exists, give an option to overwrite or not
// By default do not overwrite

fn get_dir_contents(target_directory: &PathBuf) -> Result<Box<dyn Iterator<Item = String>>, Box<dyn Error>> {
    let entries = fs::read_dir(target_directory)?
        .map(|entry| entry.unwrap().file_name().into_string().unwrap());
    Ok(Box::new(entries))
}

fn rename_entry(from: &str, to: &str) -> Result<(), Box<dyn Error>>{
    fs::rename(from, to)?;
    Ok(())
}

pub fn rename(config: Cli) -> Result<(), Box<dyn Error>> {
    let target_directory = match config.dir.as_deref() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };

    let target_directory = fs::canonicalize(&target_directory)?;

    std::env::set_current_dir(&target_directory)?;
    let files = get_dir_contents(&target_directory)?;
    let mut filters: Vec<Regex> = Vec::new();
    let re = Regex::new(&config.search_expr)?;

    if config.starts_with_list.len() > 0 {
        let startswith: Vec<&str> = config.starts_with_list.iter().map(AsRef::as_ref).collect();
        filters.push(generate_startswith_filter(&startswith)?)
    }

    'outer: for file in files {
        for filter in &filters {
            if !filter.is_match(&file) {
                continue 'outer;
            }
        }
        if re.is_match(&file) {
            let replacement = regex_replace(&file, &config.replace_expr, &re, config.replace_all);
            match_display(&replacement);
            if config.execute {
                if !config.noconfirm {
                    if ask_confirmation() {
                        rename_entry(&file, &replacement.replaced)?;
                    }
                } else {
                    rename_entry(&file, &replacement.replaced)?;
                }
            }
        }
    }
    Ok(())
}
