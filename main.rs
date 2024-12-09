use std::io::{self, Write};
use std::{env, fs};
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn string_arg(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut inside_quotes = false;
    let mut quote_char = '\0'; // Track if inside single or double quotes

    for c in input.chars() {
        match c {
            '"' | '\'' if inside_quotes => {
                // End of quote, push current argument and reset
                args.push(current_arg.trim().to_string());
                current_arg.clear();
                inside_quotes = false;
            }
            '"' | '\'' if !inside_quotes => {
                // Start of quote
                inside_quotes = true;
                quote_char = c; // Store which quote is used (single or double)
            }
            ' ' if !inside_quotes => {
                // Space outside quotes indicates a new argument
                if !current_arg.is_empty() {
                    args.push(current_arg.trim().to_string());
                    current_arg.clear();
                }
            }
            _ => {
                // Add character to current argument
                current_arg.push(c);
            }
        }
    }

    // If we have a leftover argument, push it to the result
    if !current_arg.is_empty() {
        args.push(current_arg.trim().to_string());
    }

    args
}




fn main(){



    let shell_builtins = ["echo","type", "cd", "exit", "pwd"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap(); // print it on the screen

        let stdin = io::stdin(); // make a stdin handle
        let mut input = String::new();  // make a muatble var input (string)
        stdin.read_line(&mut input).unwrap();  // use the handle to grab the input
        let input = input.trim(); // trim the input for spaces
        
        if input == "exit 0"{
            break;
        }
        
        let mut parts = input.trim().split_whitespace();
        let cmd = parts.next().unwrap();
        let args_vec : Vec<&str> = parts.collect();

        
        match cmd {

            // type cmd starts here

            "type" =>{
                if args_vec.is_empty(){
                    eprintln!("type: missing arg");
                }
                
                else {
                    let arg = args_vec[0];
                    let mut found =false;
                    let mut i=0;
                    while i < shell_builtins.len(){
                        if arg == shell_builtins[i] {
                            found = true;
                            break;
                        }
                        i +=1;
                        
                    }
                    if found == true{
                        println!("{} is a shell builtin", arg);
                    }
                    else{
                        match env::var("PATH"){
                            Ok(path)=>{
                                let directories : Vec<String> = path.split(':').map(|s| s.to_string()).collect();
                                let mut found =false;

                                for dir in directories{
                                    let full_path = format!("{}/{}", dir, arg);
                                    
                                    match fs::metadata(full_path.clone()){
                                        Ok(metadata)=>{
                                            if metadata.is_file(){
                                                println!("{} is {}",arg, full_path);
                                                found = true;
                                                break;
                                            }
                                        }
                                        _=> {
                                            continue;
                    
                                        }

                                    }
                                }
                                if !found{
                                    println!("{}: not found", arg);
                                }
                            }
                            Err(_)=>{
                                eprintln!("Failed to retrive PATH env");                            }
                        }
                    }

                }
            }

            // type cmd ends here
            // pwd starts
            "pwd" =>{
                match env::current_dir() {
                    Ok(path)=>{
                        println!("{}",path.display());
                    }
                    Err(_) =>{
                        eprintln!("pwd: error occured");
                    }
                }
            }

            //pwd ends

            //echo starts

            "echo" => {
                if args_vec.is_empty() {
                    println!();
                } else {
                    let output = string_arg(input);
                    println!("{}",output);
            }
            }

            //echo ends
            //cd starts
            
            "cd" => {
                if args_vec.is_empty(){
                    eprintln!("cd: missing argument");
                }
                else {
                let mut  path = args_vec[0].to_string();
                
                if path == "~" {
                    match env::var("HOME") {
                        Ok(home)=>{
                            path = home;
                        }
                        Err(_)=>{
                            continue;
                        }
                    }
                }

                match env::set_current_dir(path.clone()){
                    Ok(()) =>{
                        continue;
                    }
                    Err(_) => {
                        eprintln!("cd: {}: No such file or directory", path );
                    }
                }
                
                }
            }
            
            //cd ends

            //cat starts
        "cat" => {
            if args_vec.is_empty() {
                eprintln!("cat: missing file operand");
            } else {
                println!("args_vec: {:?}", args_vec); // Debug args_vec

                let paths: Vec<String> = args_vec.iter()
                    .filter(|&&path| !path.trim().is_empty()) // Filter out empty paths
                    .map(|&path| {
                        let processed_path = string_arg(path); // Call string_arg only on the path
                        println!("Raw path: '{}', Processed path: '{}'", path, processed_path); // Debug paths
                        processed_path
                    })
                    .collect();

                for path in paths {
                    if path.is_empty() {
                        eprintln!("cat: skipping empty path");
                        continue;
                    }

                    println!("Processing file: {}", path); // Debugging output

                    let file = File::open(&path);
                    match file {
                        Ok(file) => {
                            let reader = BufReader::new(file);
                            for line in reader.lines() {
                                match line {
                                    Ok(content) => println!("{}", content),
                                    Err(e) => eprintln!("cat: error reading line in {}: {}", path, e),
                                }
                            }
                        }
                        Err(e) => eprintln!("cat: {}: {}", path, e),
                    }
                }
            }
        }

            
	    // Updated `cat` section
        

	    //cat ends
            _ => {
                let output = Command::new(cmd).args(&args_vec).output();
        match output{
            Ok(output) =>{
                if !output.stdout.is_empty(){
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }

                if !output.stderr.is_empty(){
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(_) => {
                eprintln!("{}: command not found", input);
            }
        }
            }

        }


    }
}

