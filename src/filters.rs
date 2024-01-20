use regex::Error;
use regex::Regex;

pub fn generate_startswith_filter(starts_with_list: &Vec<&str>) -> Result<Regex, Error> {
    let opts: Vec<String> = starts_with_list.iter().map(|x| regex::escape(x)).collect();
    let regular_expression = format!("^({})", opts.join("|"));
    Regex::new(&regular_expression)
}

#[cfg(test)]
mod tests {
    use super::generate_startswith_filter;

    #[test]
    fn test_single_element(){
        let re = generate_startswith_filter(&vec!["abc"]).unwrap();
        println!("{}",re.as_str());
        assert!(re.as_str() == "^(abc)");
    }
    #[test]
    fn test_multiple_elements(){
        let re = generate_startswith_filter(&vec!["abc", "de8", "fgh", "123"]).unwrap();
        println!("{}",re.as_str());
        assert!(re.as_str() == "^(abc|de8|fgh|123)");
    }
    #[test]
    fn test_escaping(){
        let re = generate_startswith_filter(&vec!["^^^", ".-+"]).unwrap();
        println!("{}",re.as_str());
        assert!(re.as_str() == "^(\\^\\^\\^|\\.\\-\\+)");
    }
}
