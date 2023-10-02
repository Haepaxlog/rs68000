use crate::memory::{Memory, MemoryIter, MEMORY_CAPACITY};

pub trait Processor<M: Memory + ?Sized> {
    fn init(&mut self);
    fn run(&mut self);
    fn fetch(&mut self, mem_iter: &MemoryIter<'_, M>) -> u16;
    fn execute(&mut self);
}

enum CPUState {
    Fetching,
    Decoding,
    Executing,
    Halting,
}

pub struct CPU {
    pub registers: Registers,
    pub state: CPUState,
    pub memory_bus: MemoryBus<Box<[u8]>>,
}

struct MemoryBus<M: Memory + ?Sized> {
    memory: M,
}

#[allow(non_snake_case)]
pub struct Registers {
    /* Data Registers */
    D0: u32,
    D1: u32,
    D2: u32,
    D3: u32,
    D4: u32,
    D5: u32,
    D6: u32,
    D7: u32,
    /* Address Registers */
    A0: u32,
    A1: u32,
    A2: u32,
    A3: u32,
    A4: u32,
    A5: u32,
    A6: u32,
    // Stack Pointer (either SSP or USP)
    SP: u32,
    // Program Counter
    PC: u32,
    // Status Register (including CCR)
    SR: StatusRegister,
}

struct StatusRegister {
    trace_mode: bool,
    supervisor_state: bool,
    interrupt_mask: u8,
    extend: bool,
    negative: bool,
    zero: bool,
    overflow: bool,
    carry: bool,
}

impl CPU {
    fn new() -> Self {
        Self {
            registers: Registers::new(),
            state: CPUState::Halting,
            memory_bus: MemoryBus {
                memory: Box::new([0; MEMORY_CAPACITY]),
            },
        }
    }
}

impl Registers {
    fn new() -> Self {
        Self {
            D0: 0,
            D1: 0,
            D2: 0,
            D3: 0,
            D4: 0,
            D5: 0,
            D6: 0,
            D7: 0,
            A0: 0,
            A1: 0,
            A2: 0,
            A3: 0,
            A4: 0,
            A5: 0,
            A6: 0,
            SP: 0,
            PC: 0,
            SR: StatusRegister::new(),
        }
    }
}

impl StatusRegister {
    fn new() -> Self {
        Self {
            trace_mode: false,
            supervisor_state: false,
            interrupt_mask: 0,
            extend: false,
            negative: false,
            zero: false,
            overflow: false,
            carry: false,
        }
    }
}

impl<M: Memory + ?Sized> Processor<M> for CPU {
    fn init(&mut self) {
        self.state = CPUState::Fetching;
    }
    fn run(&mut self) {
        let mut ins: u16;
        //        let mut dec_ins: DecodedInstruction;
        let mut mem_iter = self.memory_bus.memory.iter(0);
        loop {
            match self.state {
                CPUState::Fetching => {
                    ins = self.fetch(&mem_iter);
                    self.state = CPUState::Decoding;
                }
                CPUState::Decoding => {
                    //                  dec_ins = self.decode(ins);
                }
                CPUState::Executing => {} //self.execute(dec_ins),
                CPUState::Halting => {}
            }
        }
    }
    fn fetch(&mut self, mem_iter: &MemoryIter<'_, M>) -> u16 {
        match mem_iter.next() {
            Some(value) => value,
            None => panic!("Iterator has hit memory limit"),
        }
    }
    fn execute(&mut self) {
        todo!();
    }
}
