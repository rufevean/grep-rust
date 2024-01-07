

pub fn grep_explicit_insensitive(content: &mut String, pattern: &str) {
    let mut found = false;
    for (line_number, line) in content.lines().enumerate() {
        if line.to_lowercase() == pattern.to_lowercase() {
            println!("{}: {}", line_number + 1, line);
            found = true;
        }
    }

    if !found {
        println!("Pattern Not Found Explicitly");
        println!("{}", pattern);
    }
    if content.is_empty() {
        println!("File is empty ");
    }
}
