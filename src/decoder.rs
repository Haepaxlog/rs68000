use crate::instruction::Instructions;
use crate::memory::Memory;

enum AddressingMode {
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

trait Decoder<M: Memory> {
    fn decode(&self, memory: &M);
    fn decode_add_ins(&self, memory: &M, instruction: );
}
