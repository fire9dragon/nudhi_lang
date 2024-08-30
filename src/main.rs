#![allow(warnings)]

use std::collections::btree_map::Values;
use std::env;
use std::fs;
use std::process::exit;
use std::process::Command;
use std::io;


fn nudhi_say(trimmed_line: &str) {
// Extract the argument to printl (the string to be printed)
if let Some(start) = trimmed_line.find('"') {
    if let Some(end) = trimmed_line.rfind('"') {
        if start != end {
            let message = &trimmed_line[start + 1..end];
            println!("{}", message);
        } else {
            // eprintln!("Error: Unterminated string in line: {}", line);
            eprintln!("Error: Unterminated string");
        }
    } else {
        eprintln!("Error: Missing closing quote");
    }
} else {
    // eprintln!("Error: Missing opening quote in line: {}", line);
    eprintln!("Error: Missing opening quote");
}
}

fn nudhi_do(trimmed_line: &str) {
    if let Some(start) = trimmed_line.find('"') {
        if let Some(end) = trimmed_line.rfind('"') {
            if start != end {
            let message = &trimmed_line[start + 1..end];
            
            Command::new("cmd")
                .args(["/c ", &message.replace(" ", "%space%")])
                .status()
                .expect("failed to execute process");
        }
        else {
            eprintln!("an errror");
        }
    

    }
    
}
    else {
        eprintln!("an errror");
}
}

fn nudhi_ask(trimmed_line: &str) {
    if let Some(start) = trimmed_line.find('"') {
        if let Some(end) = trimmed_line.rfind('"') {
            if start != end {
            let message = &trimmed_line[start + 1..end];
            println!("{}", message);
            
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("errot :[");
            let input: String = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => return,

            };


        }
        else {
            eprintln!("an errror");
        }
    

    }
    
}
    else {
        eprintln!("an errror");
}

}




fn nudhi_die() {
    exit(0);
}






fn main() {
    // Get the path to the source file from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        return;
    }
    let source_file = &args[1];

    // Read the source file
    let source_code = fs::read_to_string(source_file)
        .expect("Failed to read the source file");

    // Interpret the source code
    interpret(&source_code);
}

fn interpret(source_code: &str) {
    // Split the source code into lines
    let lines: Vec<&str> = source_code.lines().collect();

    for line in lines {
        // Trim whitespace from the line
        let trimmed_line = line.trim();

        // Check if the line starts with "printl"
        if trimmed_line.starts_with("nudhi_say") {
            nudhi_say(trimmed_line);
        } else if trimmed_line.starts_with("nudhi_do") {
                nudhi_do(trimmed_line);
            
        } else if trimmed_line.starts_with("nudhi_ask") {
            nudhi_ask(trimmed_line);
    } else if trimmed_line.starts_with("nudhi_die") {
        nudhi_die();
    }
        

        
        else {
        eprintln!("nudhi does not know that funtion: {}", line);

    }
}
}