use colored::*;
use regex::Regex;

pub struct RegexReplacement<'a> {
    filename: &'a str,
    m: regex::Match<'a>,
    replaced: String,
}

pub fn regex_replace<'a>(
    filename: &'a str,
    replacement: &'a str,
    re: &'a Regex,
) -> RegexReplacement<'a> {
    let regex_match = re.find(&filename).unwrap();
    let replaced = re.replace_all(&filename, replacement);
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
