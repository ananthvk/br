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

fn get_dir_contents(config: &Cli) -> Box<dyn Iterator<Item = String>> {
    let target_directory = match config.dir.as_deref() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    let entries = fs::read_dir(target_directory)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap());
    Box::new(entries)
}

pub fn rename(config: Cli) {
    let files = get_dir_contents(&config);
    let mut filters: Vec<Regex> = Vec::new();
    let re = Regex::new(&config.search_expr).unwrap();

    if config.starts_with_list.len() > 0 {
        let startswith: Vec<&str> = config.starts_with_list.iter().map(AsRef::as_ref).collect();
        filters.push(generate_startswith_filter(&startswith).unwrap())
    }

    for file in files {
        if re.is_match(&file) {
            let replacement = regex_replace(&file, &config.replace_expr, &re);
            match_display(&replacement);
        }
    }
    /*
    for entry in paths {
        let f = entry.unwrap().file_name().into_string().unwrap();
        if re.is_match(&f) && start.is_match(&f) {
            let regex_match = re.find(&f).unwrap();
            let prefix = &f[..regex_match.range().start];
            let suffix = &f[regex_match.range().end..];
            print!("{}{}{}", prefix, regex_match.as_str().red().bold(), suffix);
            let replaced = re.replace_all(&f, &config.replace_expr);
            println!(" {} {}", "=>".white().bold(), replaced);
            if config.execute {
                fs::rename(&f, &*replaced).unwrap();
            }
        }
    }
    */
}
