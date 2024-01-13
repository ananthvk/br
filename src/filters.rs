use regex::Error;
use regex::Regex;

pub fn generate_startswith_filter(starts_with_list: &Vec<&str>) -> Result<Regex, Error> {
    let opts: Vec<String> = starts_with_list.iter().map(|x| regex::escape(x)).collect();
    let regular_expression = format!("^({})", opts.join("|"));
    Regex::new(&regular_expression)
}
