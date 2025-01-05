use alloc::string::String;
use core::str;

use crate::{console::console, println};

fn parse_command(command: &str) {
    println!();
    match command {
        "help" => {
            super::CMD_LIST.print_cmds();
        },
        "" => {},
        cmd_with_args => super::CMD_LIST.run_cmd(cmd_with_args),
    }
}

pub fn shell() {
    crate::print!("shell\n> ");

    let mut buffer: String = String::with_capacity(65536);

    loop {
        let c = console().read_char();
        console().write_char(c);

        if c == '\n' || c == '\r' {
            parse_command(buffer.as_str());
            buffer.clear();
            crate::print!("> ");
        } else {
            buffer.push(c);
        }
    }
}
