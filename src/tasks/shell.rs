use alloc::{string::String, vec::Vec};

use crate::{console, println};

fn parse_command(command: String) {
    println!();
    let mut tokens: Vec<&str> = command.split(' ').collect();
    
    let cmd = tokens[0];
    match cmd {
        "help" => {
            println!("here are the available commands: ");
        },
        _ => println!("invalid command!"),
    }
}

pub fn shell() {
    crate::print!("shell > ");

    let mut buffer: String = String::new();

    loop {
        loop {
            let c = console::console().read_char();
            console::console().write_char(c);

            if c == '\n' || c == '\r' {
                parse_command(buffer);
                buffer = String::new();
                crate::print!("shell > ");
            } else {
                buffer.push(c);
            }
        }
    }
}