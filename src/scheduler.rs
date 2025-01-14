use crate::{utils::get_core, synchronization::{SpinLock, interface::Mutex}, exception};
use alloc::boxed::Box;

pub static PTABLE: PTable = PTable::new();

extern "C" {
    fn cpu_switch_to(prev: usize, next: usize);
}

fn ret_from_fork() {
    PTABLE.unlock();
    crate::exception::irq_enable();
    let mut ptr: usize = 0;
    unsafe {
        core::arch::asm!("
        mov {p}, x23
        ", p = out(reg) ptr);
    }
    let f = unsafe { core::mem::transmute::<usize, fn()>(ptr) };
    f();

    PTABLE.exit();
}

#[repr(C, align(16))]
struct CPUContext {
    x19: usize,
    x20: usize,
    x21: usize,
    x22: usize,
    x23: usize,
    x24: usize,
    x25: usize,
    x26: usize,
    x27: usize,
    x28: usize,
    fp: usize,
    sp: usize,
    pc: usize,
}

impl CPUContext {
    const fn empty() -> Self {
        Self {
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            fp: 0,
            sp: 0,
            pc: 0,
        }
    }
    
    fn set_entry(&mut self, entry: usize) {
        self.x23 = entry;
    }
    
    fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    fn set_sp(&mut self, sp: usize) {
        self.sp = sp;
    }

}

#[derive(Copy, Clone, PartialEq)]
enum TaskState {
    Unused,
    Sleeping,
    Running,
    Zombie,
}

#[repr(C, align(16))]
struct Process {
    ctx: CPUContext,
    state: TaskState,
    name: &'static str,
    pid: usize,
    stack: Box<[u8; 65536]>,
    next: Option<Box<Process>>,
}

impl Process {
    fn empty() -> Self {
        Self {
            ctx: CPUContext::empty(),
            state: TaskState::Unused,
            name: "",
            pid: 0,
            stack: Box::new([0; 65536]),
            next: None
        }
    }
}

trait ProcessList<T> {
  fn add_proc(&mut self, item: T);
  fn remove_zombies(&mut self) -> usize;
  fn get_first(&mut self) -> Self;
}

impl ProcessList<Box<Process>> for Option<Box<Process>> {

  fn add_proc(&mut self, item: Box<Process>) {
    match *self {
      Some(ref mut proc) => proc.next.add_proc(item),
      None => *self = Some(item),
    }
  }

  fn remove_zombies(&mut self) -> usize {
    let mut removed_count = 0;
    let mut current = self;
    loop {
        match current {
            None => return removed_count,
            Some(proc) if proc.state == TaskState::Zombie => {
                *current = proc.next.take();
                removed_count += 1;
            },
            Some(proc) => {
                current = &mut proc.next;
            }
        }
    }
  }

  fn get_first(&mut self) -> Option<Box<Process>> {
    let mut first = self.take();
    if let Some(proc) = &mut first {
      *self = proc.next.take();
    }
    first
  }
}

pub struct PTable {
    inner: SpinLock<PTableInner>,
}

impl PTable {
    pub const fn new() -> Self {
        Self {
            inner: SpinLock::new(PTableInner::new()),
        }
    }

    pub fn init_core(&self) {
        let mut table = self.inner.lock().unwrap();
        table.init_core_inner(get_core());
    }

    pub fn new_process(&self, name: &'static str, f: fn()) {
        crate::exception::irq_disable();
        let mut table = self.inner.lock().unwrap();
        table.new_process_inner(name, f);
        crate::exception::irq_enable();
    }

    pub fn schedule(&self) {
        exception::irq_disable();
        let mut table = self.inner.lock().unwrap();
        table.schedule_inner();
    }
    
    fn exit(&self) {
      crate::exception::irq_disable();
      {
        let mut table = self.inner.lock().unwrap();
        table.exit_current_process();
      }
      self.schedule();
    }

    pub fn print(&self) {
        crate::exception::irq_disable();
        {
            let table = self.inner.lock().unwrap();
            table.print();
        }
        crate::exception::irq_enable();
    }
    
    fn unlock(& self) {
      self.inner.unlock().unwrap();
    }
}

struct PTableInner {
    num_procs: usize,
    head: Option<Box<Process>>,
    running: [Option<Box<Process>>; 4],
}

impl PTableInner {
    const fn new() -> Self {
        Self {
            num_procs: 0,
            head: None,
            running: [None, None, None, None],
        }
    }

    fn init_core_inner(&mut self, core: u8) {
        let init_proc = Box::new(Process {
            ctx: CPUContext::empty(),
            state: TaskState::Running,
            name: "kthread",
            pid: self.num_procs + 1,
            stack: Box::new([0; 65536]),
            next: None,
        });
        self.running[core as usize] = Some(init_proc);
        self.num_procs += 1;
    }

    fn new_process_inner(&mut self, name: &'static str, f: fn()) {
        let mut new_proc = Box::new(Process {
            ctx: CPUContext::empty(),
            state: TaskState::Running,
            name,
            pid: self.num_procs + 1,
            stack: Box::new([0; 65536]),
            next: None,
        });
        let sp = &new_proc.stack[65535] as *const u8 as usize + 1;
        new_proc.ctx.set_entry(f as usize);
        new_proc.ctx.set_pc(ret_from_fork as usize);
        new_proc.ctx.set_sp(sp);

        self.num_procs += 1;
        self.head.add_proc(new_proc);
    }

    fn schedule_inner(&mut self) {
        let core = crate::utils::get_core();
        
        self.head.remove_zombies();

        if self.head.is_none() {
            return;
        }

        let next = self.head.get_first().unwrap();
        let prev = self.running[core as usize].take().unwrap();
        
        let prev_ptr = &prev.ctx as *const CPUContext as usize;
        let next_ptr = &next.ctx as *const CPUContext as usize;
        
        self.running[core as usize] = Some(next);
        self.head.add_proc(prev);
        

        unsafe {
            cpu_switch_to(prev_ptr, next_ptr);
        }
    }
    
    fn exit_current_process(&mut self) {
      if let Some(proc) = &mut self.running[crate::utils::get_core() as usize] {
        proc.state = TaskState::Zombie;
      }
    }

    fn kill(&mut self, pid: usize) {

    }

    fn print(&self) {
        crate::println!("\nProcess Table");
        for i in 0..4 {
            if let Some(curproc) = &self.running[i] {
                let page = &curproc.ctx as *const CPUContext as usize;
                let name = curproc.name;
                let pid = curproc.pid;

                crate::println!("  [core {}] pid {}, context: 0x{:X}, sp: 0x{:X}, {}", i, pid, page, curproc.ctx.sp, name);
            }
        }
        crate::println!("\nWaiting to run:");
        let mut cur = &self.head;
        while let Some(curproc) = cur {
            let page = &curproc.ctx as *const CPUContext as usize;
            let name = curproc.name;
            let pid = curproc.pid;

            crate::println!("  pid {}, context: 0x{:X}, sp: 0x{:X}, {}", pid, page, curproc.ctx.sp, name);
            cur = &curproc.next;
        }
        crate::println!("\n> ");
    }
}


