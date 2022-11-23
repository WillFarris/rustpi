use crate::{utils::get_core, synchronization::{SpinLock, interface::Mutex}, exception};
use alloc::boxed::Box;

#[link_section = ".locks"]
pub static PTABLE: PTable = PTable::new();


fn schedule_tail() {
    /*
    __asm volatile ("dsb sy");
    release(ptable.lock);
    irq_enable();
    enable_preempt();
    */


    PTABLE.unlock();
    crate::exception::irq_enable();


}

fn ret_from_fork() {
    /*
        bl schedule_tail
        mov x0, x20
        mov x1, x21
        blr x19
        bl exit
    */
    schedule_tail();
    unsafe {
        core::arch::asm!("blr x19");
    }
    loop {}
}

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
    
    fn set_entry(&mut self, entry: u64) {
        self.x19 = entry;
    }
    
    fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    fn set_sp(&mut self, sp: u64) {
        self.sp = sp;
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    core_using: Option<u8>,
    next: Option<Box<Process>>,
}

impl Process {
    const fn empty() -> Self {
        Self {
            ctx: CPUContext::empty(),
            state: PState::TaskUnused,
            name: "",
            pid: !0,
            core_using: None,
            next: None
        }
    }
}

trait ProcessList<T> {
  fn add_proc(&mut self, item: T);
}

impl ProcessList<Box<Process>> for Option<Box<Process>> {

  fn add_proc(&mut self, item: Box<Process>) {
    match *self {
      Some(ref mut proc) => proc.next.add_proc(item),
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

    pub fn schedule(&self) {
        exception::irq_disable();
        let mut table = self.inner.lock().unwrap();
        table.schedule_inner();
    }

    pub fn print(&self) {
        let table = self.inner.lock().unwrap();
        table.print();
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
            state: PState::TaskRunning,
            name: "kthread",
            pid: self.num_procs + 1,
            core_using: Some(core),
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
            core_using: None,
            next: None,
        });
        let sp = (&new_proc.ctx as *const CPUContext as u64);
        new_proc.ctx.set_entry(f as u64);
        new_proc.ctx.set_pc(ret_from_fork as u64);
        new_proc.ctx.set_sp(sp);
        crate::println!("Process {} created at address {:x}, pc={:x} sp={:x}", name, &new_proc as *const Box<Process> as u64, new_proc.ctx.pc, new_proc.ctx.sp);

        self.num_procs += 1;
        self.head.add_proc(new_proc);
    }

    fn schedule_inner(&mut self) {
        let core = crate::utils::get_core();
        
        /*while let Some(mut proc) = self.head.take() {
            if proc.state == PState::TaskZombie {
                self.head = proc.next.take();
                drop(proc);
            }
        }*/

        

    }

    fn print(&self) {
        crate::println!("Currently running:");
        for i in 0..4 {
            if let Some(curproc) = &self.running[i] {
                crate::println!("{:#x?}", curproc);
            }
        }
        crate::println!("Waiting to run:");
        let mut cur = &self.head;
        while let Some(curproc) = cur {
            crate::println!("{:#x?}", curproc);
            cur = &curproc.next;
        }
    }
}


