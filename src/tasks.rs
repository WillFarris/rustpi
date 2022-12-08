pub mod shell;

use alloc::vec::Vec;

use crate::synchronization::{SpinLock, interface::Mutex};

const NUM_CMDS: usize = 10;

#[link_section = ".locks"]
static CMD_LIST: CommandList = CommandList::new();

pub fn register_cmd(name: &'static str, entry: fn()) {
    CMD_LIST.register_cmd(name, entry);
}

struct CommandList {
    inner: SpinLock<CommandListInner>,
}

impl CommandList {
    const fn new() -> Self {
        Self {
            inner: SpinLock::new(CommandListInner::new()),
        }
    }

    fn register_cmd(&self, name: &'static str, entry: fn()) {
        let mut inner = self.inner.lock().unwrap();
        inner.register_cmd(name, entry);
    }

    fn print_cmds(&self) {
        let inner = self.inner.lock().unwrap();
        inner.print_cmds();
    }

    fn run_cmd(&self, cmd_with_args: &str) {
        let inner = self.inner.lock().unwrap();
        inner.run_cmd(cmd_with_args);
    }
}

struct CommandListInner {
    next_idx: usize,
    cmds: [Option<Command>; NUM_CMDS],
}

impl CommandListInner {
    const fn new() -> Self {
        Self {
            next_idx: 0,
            cmds: [None; NUM_CMDS],
        }
    }

    fn register_cmd(&mut self, name: &'static str, entry: fn()) {
        assert!(self.next_idx < NUM_CMDS);

        let idx = self.next_idx;
        self.cmds[idx] = Some(Command::new(name, entry));
        self.next_idx += 1;
    }

    fn print_cmds(&self) {
        crate::println!("Here are the available commands:");
        for i in 0..self.next_idx {
            if let Some(cmd) = &self.cmds[i] {
                crate::println!("  {}", cmd.name);
            }
        }
    }

    fn run_cmd(&self, cmd_with_args: &str) {
        let tokens: Vec<&str> = cmd_with_args.split(' ').collect();
        let cmd_name = tokens[0];

        for i in 0..self.next_idx {
            if let Some(cmd) = &self.cmds[i] {
                if cmd.name.cmp(cmd_name) == core::cmp::Ordering::Equal {
                    crate::scheduler::PTABLE.new_process(cmd.name, cmd.entry);
                    return;
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Command {
    name: &'static str,
    entry: fn(),
}

impl Command {
    fn new(name: &'static str, entry: fn()) -> Self {
        Self {
            name,
            entry,
        }
    }
}