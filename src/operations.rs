use std::io::{self, Write};

use colored::*;
use regex::Regex;

pub struct RegexReplacement<'a> {
    pub filename: &'a str,
    pub m: Vec<regex::Match<'a>>,
    pub replaced: String,
}

pub fn regex_replace<'a>(
    filename: &'a str,
    replacement: &'a str,
    re: &'a Regex,
    replace_all: bool,
) -> RegexReplacement<'a> {
    let regex_match = if replace_all {
        let matches: Vec<regex::Match> = re.find_iter(&filename).collect();
        matches
    } else {
        vec![re.find(&filename).unwrap()]
    };

    let replaced = if replace_all {
        re.replace_all(&filename, replacement)
    } else {
        re.replace(&filename, replacement)
    };
    RegexReplacement {
        filename,
        m: regex_match,
        replaced: replaced.into_owned(),
    }
}

pub fn match_display(r: &RegexReplacement) {
    let mut prev_idx = 0;
    for (i, rmatch) in (r.m).iter().enumerate() {
        let prefix = &r.filename[prev_idx..rmatch.range().start];
        print!("{}{}", prefix, rmatch.as_str().red().bold());
        if i == r.m.len() - 1 {
            let suffix = &r.filename[rmatch.range().end..];
            print!("{}", suffix);
        }
        prev_idx = rmatch.range().end;
    }
    println!(" {} {}", "=>".yellow().bold(), r.replaced.bold());
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

#[cfg(test)]
mod tests {
    use super::regex_replace;
    use regex::Regex;

    #[test]
    fn test_regex_replacement_only_first() {
        let re = Regex::new("te").unwrap();
        let replaced = regex_replace("testte.txt", "foo", &re, false);
        assert!(replaced.replaced == "foostte.txt");
    }

    #[test]
    fn test_regex_replacement_all() {
        let re = Regex::new("te").unwrap();
        let replaced = regex_replace("testte.txt", "foo", &re, true);
        assert!(replaced.replaced == "foostfoo.txt");
    }
}
