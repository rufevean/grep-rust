
pub fn grep_explicit(content: &mut String, pattern: &str) {
    let mut found = false;
    for (line_number, line) in content.lines().enumerate() {
        if line == pattern {
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
