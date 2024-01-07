use regex::Regex;


pub fn grep_insensitive(content: &mut String, pattern: &str) {
    let mut found: bool = false;
    let lower_pattern = pattern.to_lowercase();
    let lower_pattern_regex = Regex::new(&lower_pattern).unwrap();
    for (line_number, line) in content.lines().enumerate() {
        if lower_pattern_regex.is_match(line.to_lowercase().as_str()) {
            println!("{}: {}", line_number + 1, line);
            found = true;
        }
    }
    if !found {
        println!("Pattern Not Found Insensitive");
        println!("{}", pattern);
    }
    if content.is_empty() {
        println!("File is empty ");
    }
}
