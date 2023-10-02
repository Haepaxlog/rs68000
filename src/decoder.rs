use crate::cpu::CPU;
use crate::instruction::Instructions;
use crate::memory::Memory;

pub enum OpCodeType {
    BitManipulation,
    MovByte,
    MovLong,
    MovWord,
    Misc,
    AddqSubq,
    Branch,
    Moveq,
    OrDiv,
    Sub,
    Reserved,
    CmpEor,
    AndMul,
    Add,
    Shift,
    Extension,
}

pub const OPCM: [OpCodeType; 16] = [
    OpCodeType::BitManipulation,
    OpCodeType::MovByte,
    OpCodeType::MovLong,
    OpCodeType::MovWord,
    OpCodeType::Misc,
    OpCodeType::AddqSubq,
    OpCodeType::Branch,
    OpCodeType::Moveq,
    OpCodeType::OrDiv,
    OpCodeType::Sub,
    OpCodeType::Reserved,
    OpCodeType::CmpEor,
    OpCodeType::AndMul,
    OpCodeType::Add,
    OpCodeType::Shift,
    OpCodeType::Extension,
];

pub enum AddressingMode {
    DataRegisterDirect,
    AddressRegisterDirect,
    AbsoluteShort,
    AbsoluteLong,
    RelativeWithOffset,
    RelativeWithOffsetAndIndex,
    RegisterIndirect,
    PostincrementRegisterIndirect,
    PredecrementRegisterIndirect,
    RegisterIndirectWithOffset,
    IndexedRegisterIndirectWithOffset,
    Immediate,
    QuickImmediate,
    ImpliedRegister,
}

pub trait Decoder<M: Memory + ?Sized> {
    fn decode(&self, ins: u16) -> Instructions;
    fn decode_bit_manipulation(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_mov_byte(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_mov_long(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_mov_word(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_misc(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_addq_subq(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_branch(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_moveq(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_or_div(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_sub(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_reserved(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_cmp_eor(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_and_mul(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_add(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_shift(&mut self, mem: &mut M, ins: u16) -> Instructions;
    fn decode_extension(&mut self, mem: &mut M, ins: u16) -> Instructions;
}

impl<M: Memory + ?Sized> Decoder<M> for CPU {
    fn decode(&self, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_bit_manipulation(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_mov_byte(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_mov_long(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_mov_word(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_misc(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_addq_subq(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_branch(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_moveq(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_or_div(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_sub(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_reserved(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_cmp_eor(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_and_mul(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_add(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_shift(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }

    fn decode_extension(&mut self, mem: &mut M, ins: u16) -> Instructions {
        todo!()
    }
}
