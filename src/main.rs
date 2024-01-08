use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
mod explicit_pattern;
mod insensitive_explicit_pattern;
mod insensitive_pattern;
mod recursive_search;
mod standard_search;
mod usage;

#[derive(PartialEq)]
enum PathType {
    File,
    Folder,
}
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
        return;
    }
    let path = &args[2];
    let path_type = is_file_or_folder(&path);
    let path_as_path = std::path::Path::new(path);
    if path_type == PathType::File {
        read_contents_in_a_file(&path, &mut contents,);
    } else {
        let files = get_files_in_directory(&path_as_path).unwrap();
        for file in files {
            read_contents(&file, &mut contents, &path_as_path);
        }
    }
    let input_pattern = &args[3];
    let pattern_type: &str = &args[1];

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

fn read_contents(filename: &str, contents: &mut String, directory: &Path) {
    let filename = directory.join(filename);
    //changing program to that directory
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            contents.push_str(line);
            contents.push_str("\n");
        });
}

fn read_contents_in_a_file(filename:&str,contents:&mut String){
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            contents.push_str(line);
            contents.push_str("\n");
        });
}

fn get_files_in_directory(path: &Path) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;
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

fn is_file_or_folder(path: &str) -> PathType {
    let path = Path::new(path);
    if path.is_file() {
        return PathType::File;
    } else if path.is_dir() {
        return PathType::Folder;
    }
    panic!("Invalid Path");
}
