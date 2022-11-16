use crate::synchronization::SpinLock;

#[repr(align(16))]
#[derive(Copy, Clone)]
struct CPUContext {
    x19: u64,
    x20: u64,
    x21: u64,
    x22: u64,
    x23: u64,
    x24: u64,
    x25: u64,
    x26: u64,
    x27: u64,
    x28: u64,
    fp: u64,
    sp: u64,
    pc: u64,
}

#[derive(Copy, Clone)]
enum PState {
    TaskSleep,
    TaskRunning,
    TaskZombie,
}

#[repr(align(16))]
#[derive(Copy, Clone)]
struct Process<'a> {
    ctx: CPUContext,
    state: PState,
    name: &'a str,
    pid: u64,
    core_using: u8,
}

#[link_section = ".locks"]
static PTABLE: PTable = PTable::new();

struct PTable<'a> {
    inner: SpinLock<PTableInner<'a>>,
}

impl<'a> PTable<'a> {
    pub const fn new() -> Self {
        Self {
            inner: SpinLock::new(PTableInner::new()),
        }
    }

    
}

struct PTableInner<'a> {
    num_procs: usize,
    processes: [Option<Process<'a>>; 64],
    current: [Option<&'a Process<'a>>; 4],
}

impl<'a> PTableInner<'a> {
    const fn new() -> Self {
        Self {
            num_procs: 0,
            processes: [None; 64],
            current: [None, None, None, None],
        }
    }
}