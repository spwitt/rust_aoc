use std::fs;

/// Gets input path from the first program argument or default to `../input.txt`
/// 
/// The function assumes that input path is the first argument to the program
/// 
/// # Arguments
/// 
/// * `args` - a vector containing command line arguments
/// 
/// ```
/// use std::env;
/// use rust_aoc;
/// let path = rust_aoc::get_input_path(env::args().collect());
/// assert_eq!(path, "../input.txt");
/// ```
pub fn get_input_path(args: Vec<String>) -> String {
    if args.len() < 2 {
        return "../input.txt".to_string();
    } else {
        return args[1].to_string();
    }
}

/// Converts puzzle input String to vector with String entry for each line
pub fn get_lines_from_input_string(input: String) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

/// Helper to get input from default location as vector of Strings, one String per line
/// **NOTE**: this function does not perform _any_ error handling
pub fn get_default_input_lines() -> Vec<String> {
    get_lines_from_input_string(fs::read_to_string(get_input_path(Vec::new())).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_input_path_default() {
        assert_eq!(get_input_path(vec!["./program".to_string()]), "../input.txt");
    }

    #[test]
    fn get_input_path_value() {
        let args = vec!["./program".to_string(), "my_input.txt".to_string()];
        assert_eq!(get_input_path(args), "my_input.txt");
    }

    #[test]
    fn get_lines_empty() {
        assert!(get_lines_from_input_string(String::new()).is_empty());
    }

    #[test]
    fn get_lines() {
        assert_eq!(get_lines_from_input_string("input for\nthis\r\npuzzle\n".to_string()),
            vec!["input for".to_string(), "this".to_string(), "puzzle".to_string()]
        );
    }
}
