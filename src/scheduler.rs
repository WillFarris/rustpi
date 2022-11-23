use crate::{utils::get_core, synchronization::{SpinLock, interface::Mutex}};
use alloc::boxed::Box;

#[link_section = ".locks"]
pub static PTABLE: PTable = PTable::new();

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
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
    
    fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    fn set_sp(&mut self, sp: u64) {
        self.sp = sp;
    }

}

#[derive(Copy, Clone, Debug)]
enum PState {
    TaskUnused,
    TaskSleep,
    TaskRunning,
    TaskZombie,
}

#[repr(align(16))]
//#[derive(Copy, Clone)]
#[derive(Debug)]
struct Process {
    ctx: CPUContext,
    state: PState,
    name: &'static str,
    pid: usize,
    core_using: u8,
    next: Option<Box<Process>>,
}

impl Process {
    const fn empty() -> Self {
        Self {
            ctx: CPUContext::empty(),
            state: PState::TaskUnused,
            name: "",
            pid: !0,
            core_using: !0,
            next: None
        }
    }
}

trait LinkedList<T> {
  fn add(&mut self, item: T);
}

impl LinkedList<Box<Process>> for Option<Box<Process>> {
  fn add(&mut self, item: Box<Process>) {
    match *self {
      Some(ref mut proc) => proc.next.add(item),
      None => *self = Some(item),
    }
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
        let mut table = self.inner.lock().unwrap();
        table.new_process_inner(name, f);
    }

    pub fn print(&self) {
        let table = self.inner.lock().unwrap();
        table.print();
    }
    
    pub fn unlock(& self) {
      self.inner.unlock();
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
    
    fn schedule_tail() {
      /*
      __asm volatile ("dsb sy");
    release(ptable.lock);
    irq_enable();
    enable_preempt();
      */
      //PTABLE.unlock();
      crate::exception::irq_enable();
      
      
    }
    
    fn ret_from_fork(&self, prev: &Process, next: &Process) {
      /*
        bl schedule_tail
        mov x0, x20
        mov x1, x21
        blr x19
        bl exit
      */
      //self.schedule_tail();
      unsafe {
        core::arch::asm!("
          mov x0, x0
        ");
      }
      
    }

    fn init_core_inner(&mut self, core: u8) {
        let init_proc = Box::new(Process {
            ctx: CPUContext::empty(),
            state: PState::TaskRunning,
            name: "kthread",
            pid: self.num_procs + 1,
            core_using: core,
            next: None,
        });
        self.running[core as usize] = Some(init_proc);
        self.num_procs += 1;
    }

    fn new_process_inner(&mut self, name: &'static str, f: fn()) {
        let mut new_proc = Box::new(Process {
            ctx: CPUContext::empty(),
            state: PState::TaskRunning,
            name,
            pid: self.num_procs + 1,
            core_using: 0xF,
            next: None,
        });
        let sp = (&new_proc.ctx.sp as *const u64 as u64& !0xFFFF) + 4096;
        //new_proc.ctx.set_entry(f as u64);
        //new_proc.ctx.set_pc(self.ret_from_fork);
        new_proc.ctx.set_sp(sp);
        crate::println!("Process {} created at address {:x}, pc={:x} sp={:x}", name, &new_proc as *const Box<Process> as u64, new_proc.ctx.pc, new_proc.ctx.sp);
        //new_proc.ctx.set_sp();

        self.num_procs += 1;
        self.head.add(new_proc);
    }

    fn print(&self) {
        crate::println!("Currently running:");
        for i in 0..4 {
            if let Some(curproc) = &self.running[i] {
                crate::println!("{:#?}", curproc);
            }
        }
        crate::println!("Waiting to run:");
        let mut cur = &self.head;
        while let Some(curproc) = cur {
            crate::println!("{:#?}", curproc);
            cur = &curproc.next;
        }
    }
}