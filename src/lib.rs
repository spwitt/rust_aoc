
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_input_path_default() {
        assert_eq!(get_input_path(Vec::new()), "../input.txt");
    }

    #[test]
    fn get_input_path_value() {
        let args = vec!["./program".to_string(), "my_input.txt".to_string()];
        assert_eq!(get_input_path(args), "my_input.txt");
    }
}
