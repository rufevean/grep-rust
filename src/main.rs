use std::env;
use std::fs;
use regex::Regex; 



fn main() {
    let mut contents = String::new();
    let args : Vec<String> = env::args().collect();
    let filename = &args[1];
    let input_pattern  = &args[2];
    let mut explicit_pattern = "";
    if args.len() == 4 {
        explicit_pattern = &args[3];
    }
    read_contents(&filename, &mut contents);
    if explicit_pattern == "-e"{
        grep_explicit(&mut contents,input_pattern);
    }else{        
        let gen_regex = format!(r"\w*{}\b",input_pattern);
        let pattern = Regex::new(&gen_regex).unwrap();
        grep(&mut contents,pattern);
    }

}

fn read_contents(filename: &str,contents : &mut String){
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            contents.push_str(line);
            contents.push_str("\n");
        });
}

fn grep(contents : &mut String,pattern: Regex){
    let mut found = false;
    for (line_number,line) in contents.lines().enumerate(){
            if pattern.is_match(line){
                println!("{}: {}",line_number+1,line);
                found = true;
            } 
    }
    if !found {
        println!("Pattern Not Found");
    }
    if contents.is_empty(){
        println!("File is empty ");
    }
}

fn grep_explicit(content : &mut String,pattern :&str){
    let mut found = false;
    for (line_number,line) in content.lines().enumerate(){
        if line ==pattern{
            println!("{}: {}",line_number+1,line);
            found = true;
        }
    }

    if !found {
        println!("Pattern Not Found Explicitly");
        println!("{}",pattern);
    }
    if content.is_empty(){
        println!("File is empty ");
    }

}
