pub fn usage(){
    println!("Usage: rgrep [OPTION]... PATTERN [FILE]...");
    println!("Search for PATTERN in each FILE or standard input.");
    println!("PATTERN is, by default, a basic regular expression (BRE).");
    println!("Example: rgrep -s words.txt pattern");
    println!("-s, --standard-search");
    println!("-i, --ignore-case");
    println!("-e, --explicit-search");
    println!("-ie, --explicit-search --ignore-case");
}
