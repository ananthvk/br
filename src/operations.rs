
use std::io::{self, Write};

use colored::*;

use regex::Regex;

pub struct RegexReplacement<'a> {
    pub filename: &'a str,
    pub m: regex::Match<'a>,
    pub replaced: String,
}

pub fn regex_replace<'a>(
    filename: &'a str,
    replacement: &'a str,
    re: &'a Regex,
) -> RegexReplacement<'a> {
    let regex_match = re.find(&filename).unwrap();
    let replaced = re.replace(&filename, replacement);
    RegexReplacement {
        filename,
        m: regex_match,
        replaced: replaced.into_owned(),
    }
}

pub fn match_display(r: &RegexReplacement) {
    let prefix = &r.filename[..r.m.range().start];
    let suffix = &r.filename[r.m.range().end..];
    print!("{}{}{}", prefix, r.m.as_str().red().bold(), suffix);
    println!(" {} {}", "=>".white().bold(), r.replaced);
}

pub fn ask_confirmation() -> bool {
    let mut line: String = String::new();
    print!("Do you want to continue? (y/N): ");
    io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut line) {
        Ok(n) if n > 0 && line.chars().next().unwrap() == 'y' => true,
        Ok(n) if n > 0 && line.chars().next().unwrap() == 'n' => false,
        Ok(n) if n > 0 && line.chars().next().unwrap() == 'Y' => true,
        Ok(n) if n > 0 && line.chars().next().unwrap() == 'N' => false,
        Ok(_) => false,
        Err(err) => panic!("{:#?}", err),
    }
}
