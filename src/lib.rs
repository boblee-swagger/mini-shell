pub mod command;
pub use command::cat;


use std::io::{self, Write};

pub fn start() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap(); //flush the output so the prompt shows up

        let input = read_input();
        if input.is_empty() {
            continue; 
        }

        let mut parts  = input.split_whitespace();
        let command = parts.next().unwrap();
        let mut args  = parts;

        match command {
            "cat" => {
                match cat(command, &mut args) {
                    Ok(output) => print!("{}", output), 
                    Err(e) => eprint!("Error: {}", e),    
                }
            }
            _ => println!("Command not found"), 
        }
    }
}

//read from the stdin an return it as String
pub fn read_input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim(); 
    return input.to_string();
}