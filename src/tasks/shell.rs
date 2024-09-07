use core::str;

use crate::{console::console, println};

fn parse_command(command: &str) {
    println!();
    match command {
        "help" => {
            super::CMD_LIST.print_cmds();
        },
        "" => {},
        cmd_with_args => println!("Would run {}", cmd_with_args),//super::CMD_LIST.run_cmd(cmd_with_args),
    }
}

pub fn shell() {
    crate::print!("shell\n> ");

    loop {
        let c = console().read_char();
        console().write_char(c);

        if c == '\n' || c == '\r' {
            crate::print!("\n> ");
        }
    }
}
