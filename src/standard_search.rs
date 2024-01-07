use regex::Regex;
pub fn grep(contents: &mut String, pattern: Regex) {
    let mut found = false;
    for (line_number, line) in contents.lines().enumerate() {
        if pattern.is_match(line) {
            println!("{}: {}", line_number + 1, line);
            found = true;
        }
    }
    if !found {
        println!("Pattern Not Found");
    }
    if contents.is_empty() {
        println!("File is empty ");
    }
}
