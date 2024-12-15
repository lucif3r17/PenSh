use std::io::{self, Write, BufRead, BufReader};
use std::{env, fs};
use std::process::{Command, exit};
use std::fs::File;

fn string_arg(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut inside_quotes = false;
    let mut quote_char = '\0'; 
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                
                if let Some(next_char) = chars.next() {
                    if inside_quotes {
                        
                        match next_char {
                            'n' => current_arg.push('\n'),
                            '\\' => current_arg.push('\\'),
                            _ => {
                                current_arg.push('\\');
                                current_arg.push(next_char);
                            }
                        }
                    } else {
                       
                        match next_char {
                            'n' => current_arg.push('n'),
                            '\\' => current_arg.push('\\'),
                            _ => {
                                current_arg.push(next_char);
                            }
                        }
                    }
                } else {
                    current_arg.push('\\'); 
                }
            }
            '"' | '\'' if !inside_quotes => {
                inside_quotes = true;
                quote_char = c;
            }
            c if c == quote_char && inside_quotes => {
                inside_quotes = false;
            }
            ' ' if !inside_quotes => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

fn main() {
    let shell_builtins = ["echo", "type", "cd", "exit", "pwd"];

    loop {
        
        print!("{}$ ", env::current_dir().unwrap().display());        
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue; 
        }
    
        let args = string_arg(input);
        let cmd = args.first().map(String::as_str).unwrap_or("");
        let args_vec = &args[1..];

        match cmd {
            "exit" | "exit 0" => {
                
                exit(0);
            }

            "type" => {
                if args_vec.is_empty() {
                    eprintln!("type: missing argument");
                } else {
                    let arg = &args_vec[0];
                    if shell_builtins.contains(&arg.as_str()) {
                        println!("{} is a shell builtin", arg);
                    } else {
                        match env::var("PATH") {
                            Ok(path) => {
                                let directories: Vec<&str> = path.split(':').collect();
                                let mut found = false;

                                for dir in directories {
                                    let full_path = format!("{}/{}", dir, arg);
                                    if fs::metadata(&full_path).is_ok() {
                                        println!("{} is {}", arg, full_path);
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    println!("{}: not found", arg);
                                }
                            }
                            Err(_) => eprintln!("Failed to retrieve PATH environment variable"),
                        }
                    }
                }
            }

            "pwd" => {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(_) => eprintln!("pwd: error occurred"),
                }
            }

            "echo" => {
                let result = args[1..].join(" ");
                println!("{}", result);
            }

            "cd" => {
                if args_vec.is_empty() {
                    eprintln!("cd: missing argument");
                } else {
                    let mut path = args_vec[0].clone();
                    if path == "~" {
                        path = env::var("HOME").unwrap_or_else(|_| String::from("/"));
                    }
                    if let Err(_) = env::set_current_dir(&path) {
                        eprintln!("cd: {}: No such file or directory", path);
                    }
                }
            }

            "cat" => {
                let args = string_arg(input); 
                if args.len() < 2 {
                    eprintln!("cat: missing file operand");
                } else {
                    for file_path in &args[1..] {
                        match File::open(file_path) {
                            Ok(file) => {
                                let reader = BufReader::new(file);
                                for line in reader.lines() {
                                    match line {
                                        Ok(content) => print!("{}", content), 
                                        Err(e) => eprintln!("cat: error reading {}: {}", file_path, e),
                                    }
                                }
                            }
                            Err(e) => eprintln!("cat: {}: {}", file_path, e),
                        }
                    }
                    println!();
                }
            }

            _ => {
                
                let output = Command::new(cmd).args(args_vec).output();
                match output {
                    Ok(output) => {
                        if !output.stdout.is_empty() {
                            print!("{}", String::from_utf8_lossy(&output.stdout));
                        }
                        if !output.stderr.is_empty() {
                            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    Err(_) => eprintln!("{}: command not found", cmd),
                }
            }
        }
    }
}
