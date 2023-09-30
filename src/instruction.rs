use core::fmt;

pub enum Instructions {
    // Add Decimal With Extend: Dst + Src + X -> Dst
    ABCD(Target, Target),
    // Add: Dst + Src -> Dst
    ADD(Target, Target),
    // Add Address: Dst + Src -> Dst
    ADDA(Target, AddressRegister),
    // Add Immediate: Dst + Immediate Data -> Dst
    ADDI(Immediate, Target),
    // Add Quick: Dst + Immediate Data -> Dst
    ADDQ(Immediate, Target),
    // Add with Extend: Dst + Src + X -> Dst
    ADDX(Target, Target),
    // And: Dst & Src -> Dst
    AND(Target, Target),
    // And Immediate: Dst & Immediate Data -> Dst
    ANDI(Immediate, Target),
    // Arithmetic Shift Left: Dst << n -> Dst
    ASL(Target, Target),
    // Arithmetic Shift Right: Dst >> n -> Dst
    ASR(Target, Target),
    // Branch Conditionally: If CC then PC + d -> PC
    BCC(Label),
    // Bit Test and Change: ~(number of Dst) -> Z, 0 -> (number of Dst)
    BCHG(Target, Target),
    // Bit Test and Clear: ~(number of Dst) -> Z, 1 -> (number of Dst)
    BCLR(Target, Target),
    // Branch Always: PC + d -> PC
    BRA(Label),
    // Bit Test and Set: ~(number of Dst) -> Z, 1 -> (number of Dst)
    BSET(Target, Target),
    // Branch to Subroutine: PC -> -(SP), PC + d -> PC
    BSR(Label),
    // Bit Test: ~(number of Dst) -> Z
    BTST(Target, Target),
    // Check Register Against Bounds: if Dn then Trap
    CHK(Target, Register),
    // Clear Operand: 0 -> Dst
    CLR(Target),
    // Compare: Dst - Src
    CMP(Target, DataRegister),
    // Compare Address: Dst - Src
    CMPA(Target, AddressRegister),
    // Compare Immediate: Dst - Immediate Data
    CMPI(Immediate, Target),
    // Compare Memory: Dst - Src
    CMPM(Target, Target),
    // Test Condition, Decrement and Branch: if ~CC then Dn - 1 -> Dn, if Dn != 1 then PC + d -> PC
    DBCC(Register, Label),
    // Signed Divide: Dst/Src -> Dst
    DIVS(Target, DataRegister),
    // Unsigned Divide: Dst/Src -> Dst
    DIVU(Target, DataRegister),
    // Exclusive Or: Dst^Src -> Dst
    EOR(DataRegister, Target),
    // Exclusive Or to CCR: Src^CCR -> CCR
    EORtoCCR(Immediate),
    // Exclusive Or to SR: if supervisor state then SRC^SR -> SR else TRAP
    EORtoSR(Immediate),
    // Exclusive Or Immediate: Dst^Immediate Data -> Dst
    EORI(Immediate, Target),
    // Exchange Registers: Rx <-> Ry
    EXG(Target, Target),
    // Sign Extend: Dst Sign-Extended -> Dst
    EXT(DataRegister),
    // Illegal
    ILLEGAL,
    // Jump: Dst -> PC
    JMP(Target),
    // Jump to Subroutine: PC -> -(SP), Dst -> PC
    JSR(Target),
    // Load Effective Address: <ea> -> An
    LEA(Target, AddressRegister),
    // Link and Allocate: An -> -(Sp), Sp -> An, Sp + Displacement -> Sp
    LINK(AddressRegister, Immediate),
    // Logical Shift Left: Dst << n -> Dst
    LSL(Target, Target),
    // Logical Shift Right: Dst >> n -> Dst
    LSR(Target, Target),
    // Move: Src -> Dst
    MOVE(Target, Target),
    // Move Address: Src -> Dst
    MOVEA(Target, AddressRegister),
    // Move from CCR: CCR -> Dst
    MOVEfromCCR(Target),
    // Move to CCR: Src -> CCR
    MOVEtoCCR(Target),
    // Move from SR: SR -> Dst, if supervisor state then Src -> SR else TRAP
    MOVEfromSR(Target),
    // Move to SR: if supervisor state then Src -> SR else TRAP
    MOVEtoSR(Target),
    // Move User Stack Pointer: if supervisor state then USP -> An or An -> USP else TRAP
    MOVEUSP(Target, Direction),
    // Move c: if supervisor state then Rc -> Rn or Rn -> Rc else TRAP
    MOVEC(Target, ControlRegister, Direction),
    // Move Multiple Registers: Registers -> Dst, Src -> Registers
    MOVEM(Target, Size, Label, Direction),
    // Move Peripheral Data: Src -> Dst
    MOVEP(Target, Target),
    // Move Quick: Immediate Data -> Dst
    MOVEQ(Immediate, DataRegister),
    // Signed Multiply: Dst*Src -> Dst
    MULS(Target, DataRegister),
    // Unsigned Multiply: Dst*Src -> Dst
    MULU(Target, DataRegister),
    // Negate Decimal with Extend: 0 - Dst - X -> Dst
    NBCD(Target),
    // Negate: 0 - Dst -> Dst
    NEG(Target),
    // Negate with Extend: 0 - Dst - X -> Dst
    NEGX(Target),
    // No Operation
    NOP,
    // 1's Complement: ~Dst -> Dst
    NOT(Target),
    // Logical Or: Dst | Src -> Dst
    OR(Target, Target),
    // Or Immediate: Dst | Immediate Data -> Dst
    ORI(Immediate, Target),
    // Or Immediate to CCR: Src | CCR -> CCR
    ORItoCCR(Immediate),
    // Or Immediate to SR: if supervisor state then Src | SR -> SR else TRAP
    ORItoSR(Immediate),
    // Push Effective Address: <ea> -> -(SP)
    PEA(Target),
    // Reset External Devices
    RESET,
    // Rotate Left without Extend: Dst rotated by n -> Dst
    ROL(Target, Target),
    // Rotate Right without Extend: Dst rotated by n -> Dst
    ROR(Target, Target),
    // Rotate Left with Extend: Dst rotated by n -> Dst
    ROXL(Target, Target),
    // Rotate Right with Extend: Dst rotated by n -> Dst
    ROXR(Target, Target),
    // Return with Displacement: (SP) -> PC, SP + 4 + d -> SP
    RTD(Immediate),
    // Return from Exception: if supervisor state then (SP) -> SR, SP + 2 -> SP, (SP) -> PC, SP + 4 -> SP else TRAP
    RTE,
    // Return and Restore: (SP) -> CCR, SP + 2 -> SP, (SP) -> PC, SP + 4 -> SP
    RTR,
    // Return from Subroutine: (SP) -> PC, SP + 4 -> SP
    RTS,
    // Subtract Decimal with Extend: Dst - Src - X -> Dst
    SBCD(Target, Target),
    // Set According to Condition: If CC then 1's -> Dst else 0's -> Dst
    SCC(Target),
    // Stop: Immediate Data -> SR, STOP
    STOP(Immediate),
    // Subtract: Dst - Src -> Dst
    SUB(Target, Target),
    // Subtract Address: Dst - Src -> Dst
    SUBA(Target, AddressRegister),
    // Subtract Immediate: Dst - Immediate Data -> Dst
    SUBI(Immediate, Target),
    // Subract Quick: Dst - Immediate Data -> Dst
    SUBQ(Immediate, Target),
    // Subtract with Extend: Dst - Src - X -> DSt
    SUBX(Target, Target),
    // Swap Data Register Halves: Register [31:16] <-> Register [15:0]
    SWAP(DataRegister),
    // Test and Set Operand: Dst Tested -> CC, 1 -> (7th bit of Dst)
    TAS(Target),
    // Trap: SSP - 2 -> SSP, Format/Offset -> (SSP), SPP - 4 -> SSP, PC -> (SSP), SSP -2 -> SSP, SR -> (SSP), Vector Address -> PC
    TRAP(Immediate),
    // Trap on Overflow: if V then TRAP
    TRAPV,
    // Test: Dst Tested -> CC
    TST(Target),
    // Unlink: An -> Sp, Sp -> An
    UNLK(AddressRegister),
    // Not Implemented
    NotImplemented,
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instructions::ABCD(src, st) => write!(f, "ABCD {}, {}", src, dst),
            Instructions::ADD(_, _) => todo!(),
            Instructions::ADDA(_, _) => todo!(),
            Instructions::ADDI(_, _) => todo!(),
            Instructions::ADDQ(_, _) => todo!(),
            Instructions::ADDX(_, _) => todo!(),
            Instructions::AND(_, _) => todo!(),
            Instructions::ANDI(_, _) => todo!(),
            Instructions::ASL(_, _) => todo!(),
            Instructions::ASR(_, _) => todo!(),
            Instructions::BCC(_) => todo!(),
            Instructions::BCHG(_, _) => todo!(),
            Instructions::BCLR(_, _) => todo!(),
            Instructions::BRA(_) => todo!(),
            Instructions::BSET(_, _) => todo!(),
            Instructions::BSR(_) => todo!(),
            Instructions::BTST(_, _) => todo!(),
            Instructions::CHK(_, _) => todo!(),
            Instructions::CLR(_) => todo!(),
            Instructions::CMP(_, _) => todo!(),
            Instructions::CMPA(_, _) => todo!(),
            Instructions::CMPI(_, _) => todo!(),
            Instructions::CMPM(_, _) => todo!(),
            Instructions::DBCC(_, _) => todo!(),
            Instructions::DIVS(_, _) => todo!(),
            Instructions::DIVU(_, _) => todo!(),
            Instructions::EOR(_, _) => todo!(),
            Instructions::EORtoCCR(_) => todo!(),
            Instructions::EORtoSR(_) => todo!(),
            Instructions::EORI(_, _) => todo!(),
            Instructions::EXG(_, _) => todo!(),
            Instructions::EXT(_) => todo!(),
            Instructions::ILLEGAL => todo!(),
            Instructions::JMP(_) => todo!(),
            Instructions::JSR(_) => todo!(),
            Instructions::LEA(_, _) => todo!(),
            Instructions::LINK(_, _) => todo!(),
            Instructions::LSL(_, _) => todo!(),
            Instructions::LSR(_, _) => todo!(),
            Instructions::MOVE(_, _) => todo!(),
            Instructions::MOVEA(_, _) => todo!(),
            Instructions::MOVEfromCCR(_) => todo!(),
            Instructions::MOVEtoCCR(_) => todo!(),
            Instructions::MOVEfromSR(_) => todo!(),
            Instructions::MOVEtoSR(_) => todo!(),
            Instructions::MOVEUSP(_, _) => todo!(),
            Instructions::MOVEC(_, _, _) => todo!(),
            Instructions::MOVEM(_, _, _, _) => todo!(),
            Instructions::MOVEP(_, _) => todo!(),
            Instructions::MOVEQ(_, _) => todo!(),
            Instructions::MULS(_, _) => todo!(),
            Instructions::MULU(_, _) => todo!(),
            Instructions::NBCD(_) => todo!(),
            Instructions::NEG(_) => todo!(),
            Instructions::NEGX(_) => todo!(),
            Instructions::NOP => todo!(),
            Instructions::NOT(_) => todo!(),
            Instructions::OR(_, _) => todo!(),
            Instructions::ORI(_, _) => todo!(),
            Instructions::ORItoCCR(_) => todo!(),
            Instructions::ORItoSR(_) => todo!(),
            Instructions::PEA(_) => todo!(),
            Instructions::RESET => todo!(),
            Instructions::ROL(_, _) => todo!(),
            Instructions::ROR(_, _) => todo!(),
            Instructions::ROXL(_, _) => todo!(),
            Instructions::ROXR(_, _) => todo!(),
            Instructions::RTD(_) => todo!(),
            Instructions::RTE => todo!(),
            Instructions::RTR => todo!(),
            Instructions::RTS => todo!(),
            Instructions::SBCD(_, _) => todo!(),
            Instructions::SCC(_) => todo!(),
            Instructions::STOP(_) => todo!(),
            Instructions::SUB(_, _) => todo!(),
            Instructions::SUBA(_, _) => todo!(),
            Instructions::SUBI(_, _) => todo!(),
            Instructions::SUBQ(_, _) => todo!(),
            Instructions::SUBX(_, _) => todo!(),
            Instructions::SWAP(_) => todo!(),
            Instructions::TAS(_) => todo!(),
            Instructions::TRAP(_) => todo!(),
            Instructions::TRAPV => todo!(),
            Instructions::TST(_) => todo!(),
            Instructions::UNLK(_) => todo!(),
            Instructions::NotImplemented => todo!(),
        }
    }
}
