use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::fs::metadata;
mod explicit_pattern;
mod insensitive_explicit_pattern;
mod insensitive_pattern;
mod recursive_search;
mod standard_search;
mod usage;
fn main() {
    let mut contents = String::new();
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let input = &args[1];
        if input == "-h" || input == "--help" {
            usage::usage();
        } else {
            println!("Invalid Input");
        }
        //end the program
        return;
    }
    let path = &args[2];
    let input_pattern = &args[3];
    let pattern_type: &str = &args[1];
    verify_path(path);
    println!("Path : {}", path);
    let file_names = get_files_in_directory(path).unwrap();
    for file in file_names{
        read_contents(&file, &mut contents);
    }

    // matching the arugements with respective functions
    /*
        -s : Standard search
        -e : Explicit Pattern
        -i : Case Insensitive
        -ie : Explicit and Case Insensitive
    */
    match pattern_type {
        "-e" => {
            explicit_pattern::grep_explicit(&mut contents, input_pattern);
        }
        "-i" => {
            insensitive_pattern::grep_insensitive(&mut contents, input_pattern);
        }
        "-ie" => {
            insensitive_explicit_pattern::grep_explicit_insensitive(&mut contents, input_pattern);
        }
        "-s" => {
            let pattern = Regex::new(input_pattern).unwrap();
            standard_search::grep(&mut contents, pattern);
        }
        "-r" => {
            recursive_search::grep_recursive(&mut contents, input_pattern);
        }
        _ => {
            println!("Invalid Input");
        }
    }
}

fn read_contents(filename: &str, contents: &mut String) {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            contents.push_str(line);
            contents.push_str("\n");
        });
}
fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
    // Get a list of all entries in the folder
    println!("Path: {}", path);
    let entries = fs::read_dir(path)?;

    // Extract the filenames from the directory entries and store them in a vector
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(file_names)
}


fn verify_path(path: &str) {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_dir() {
            if fs::read_dir(path).count() == 0 {
                println!("Path is empty");
            }
        } else {
            println!("Path is not a directory");
        }
    } else {
        println!("Path does not exist");
    }
}

