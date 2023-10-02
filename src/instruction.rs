use core::fmt;
use std::fmt::write;

type LongLabel = i32;
type WordLabel = i16;

pub enum Size {
    Byte,
    Word,
    Long,
}

pub enum Direction {
    RegisterToMemory,
    MemoryToRegister,
}

pub struct IndexRegister {
    register: Registers,
    size: Size,
    scale: u8,
}

#[allow(non_snake_case)]
pub enum Registers {
    /* Data Registers */
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    /* Address Registers */
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    // Stack Pointer (either SSP or USP)
    SP,
    // Program Counter
    PC,
    // Status Register (including CCR)
    SR,
}

pub enum Target {
    DnDirect(Registers),
    AnDirect(Registers),
    AnIndirect(Registers),
    AnIndirectPostInc(Registers),
    AnIndirectPreDec(Registers),
    AnIndirectDisplacement(Registers, i32),
    AnIndirectIndex(i32, Registers, IndexRegister),
    AnIndirectIndexBase(Option<i32>, Registers, IndexRegister),
    MemoryIndirectPostIndex(
        Option<i32>,
        Option<Registers>,
        Option<IndexRegister>,
        Option<i32>,
    ),
    MemoryIndirectPreIndex(
        Option<i32>,
        Option<Registers>,
        Option<IndexRegister>,
        Option<i32>,
    ),
    PCIndirectDisplacement(i32, Registers),
    PCIndirectIndex(i32, Registers, IndexRegister),
    PCIndirectIndexBase(Option<i32>, Option<Registers>, Option<IndexRegister>),
    PCMemoryIndirectPostIndex(
        Option<i32>,
        Option<Registers>,
        Option<IndexRegister>,
        Option<i32>,
    ),
    PCMemoryIndirectPreIndex(
        Option<i32>,
        Option<Registers>,
        Option<IndexRegister>,
        Option<i32>,
    ),
    AbsoluteShortAddress(i32),
    AbsoluteLongAddress(u32, u32),
    Immediate(u32),
}

pub enum Condition {
    CarryClear,
    CarrySet,
    Equal,
    Geq,
    Gt,
    High,
    Leq,
    Low,
    Lt,
    Minus,
    Neq,
    Plus,
    OverflowClear,
    OverflowSet,
}

pub enum ControlRegister {
    VBR,
}

pub enum Instructions {
    // Add Decimal With Extend: Dst + Src + X -> Dst
    ABCD(Target, Target),
    // Add: Dst + Src -> Dst
    ADD(Target, Target, Size),
    // Add Address: Dst + Src -> Dst
    ADDA(Target, Registers, Size),
    // Add Immediate: Dst + Immediate Data -> Dst
    ADDI(u32, Target, Size),
    // Add Quick: Dst + Immediate Data -> Dst
    ADDQ(u32, Target, Size),
    // Add with Extend: Dst + Src + X -> Dst
    ADDX(Target, Target, Size),
    // And: Dst & Src -> Dst
    AND(Target, Target, Size),
    // And Immediate: Dst & Immediate Data -> Dst
    ANDI(u32, Target, Size),
    // Arithmetic Shift Left: Dst << n -> Dst
    ASL(Target, Target, Size),
    // Arithmetic Shift Right: Dst >> n -> Dst
    ASR(Target, Target, Size),
    // Branch Conditionally: If CC then PC + d -> PC
    Bcc(Condition, LongLabel),
    // Bit Test and Change: ~(number of Dst) -> Z, 0 -> (number of Dst)
    BCHG(Target, Target, Size),
    // Bit Test and Clear: ~(number of Dst) -> Z, 1 -> (number of Dst)
    BCLR(Target, Target, Size),
    // Branch Always: PC + d -> PC
    BRA(LongLabel),
    // Bit Test and Set: ~(number of Dst) -> Z, 1 -> (number of Dst)
    BSET(Target, Target, Size),
    // Branch to Subroutine: PC -> -(SP), PC + d -> PC
    BSR(LongLabel),
    // Bit Test: ~(number of Dst) -> Z
    BTST(Target, Target, Size),
    // Check Register Against Bounds: if Dn then Trap
    CHK(Target, Registers, Size),
    // Clear Operand: 0 -> Dst
    CLR(Target, Size),
    // Compare: Dst - Src
    CMP(Target, Target, Size),
    // Compare Address: Dst - Src
    CMPA(Target, Registers, Size),
    // Compare Immediate: Dst - Immediate Data
    CMPI(u32, Target, Size),
    // Compare Memory: Dst - Src
    CMPM(Target, Target, Size),
    // Test Condition, Decrement and Branch: if ~CC then Dn - 1 -> Dn, if Dn != 1 then PC + d -> PC
    DBcc(Condition, Registers, WordLabel),
    // Signed Divide: Dst/Src -> Dst
    DIVSW(Target, Registers, Size),
    // Unsigned Divide: Dst/Src -> Dst
    DIVUW(Target, Registers, Size),
    // Exclusive Or: Dst^Src -> Dst
    EOR(Target, Target, Size),
    // Exclusive Or to CCR: Src^CCR -> CCR
    EORtoCCR(u8),
    // Exclusive Or to SR: if supervisor state then SRC^SR -> SR else TRAP
    EORtoSR(u16),
    // Exclusive Or Immediate: Dst^Immediate Data -> Dst
    EORI(u32, Target, Size),
    // Exchange Registers: Rx <-> Ry
    EXG(Target, Target),
    // Sign Extend: Dst Sign-Extended -> Dst
    EXT(Registers, Size, Size),
    // Illegal
    ILLEGAL,
    // Jump: Dst -> PC
    JMP(Target),
    // Jump to Subroutine: PC -> -(SP), Dst -> PC
    JSR(Target),
    // Load Effective Address: <ea> -> An
    LEA(Target, Registers),
    // Link and Allocate: An -> -(Sp), Sp -> An, Sp + Displacement -> Sp
    LINK(Registers, i32),
    // Logical Shift Left: Dst << n -> Dst
    LSL(Target, Target, Size),
    // Logical Shift Right: Dst >> n -> Dst
    LSR(Target, Target, Size),
    // Move: Src -> Dst
    MOVE(Target, Target, Size),
    // Move Address: Src -> Dst
    MOVEA(Target, Registers, Size),
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
    // Move Control Register: if supervisor state then Rc -> Rn or Rn -> Rc else TRAP
    MOVEC(Target, ControlRegister, Direction),
    // Move Multiple Registers: Registers -> Dst, Src -> Registers
    MOVEM(Target, Size, WordLabel, Direction),
    // Move Peripheral Data: Src -> Dst
    MOVEP(Registers, Registers, WordLabel, Size, Direction),
    // Move Quick: Immediate Data -> Dst
    MOVEQ(u8, Registers),
    // Signed Multiply: Dst*Src -> Dst
    MULSW(Target, Registers),
    // Unsigned Multiply: Dst*Src -> Dst
    MULUW(Target, Registers),
    // Negate Decimal with Extend: 0 - Dst - X -> Dst
    NBCD(Target),
    // Negate: 0 - Dst -> Dst
    NEG(Target, Size),
    // Negate with Extend: 0 - Dst - X -> Dst
    NEGX(Target, Size),
    // No Operation
    NOP,
    // 1's Complement: ~Dst -> Dst
    NOT(Target, Size),
    // Logical Or: Dst | Src -> Dst
    OR(Target, Target, Size),
    // Or Immediate: Dst | Immediate Data -> Dst
    ORI(u32, Target, Size),
    // Or Immediate to CCR: Src | CCR -> CCR
    ORItoCCR(u32),
    // Or Immediate to SR: if supervisor state then Src | SR -> SR else TRAP
    ORItoSR(u32),
    // Push Effective Address: <ea> -> -(SP)
    PEA(Target),
    // Reset External Devices
    RESET,
    // Rotate Left without Extend: Dst rotated by n -> Dst
    ROL(Target, Target, Size),
    // Rotate Right without Extend: Dst rotated by n -> Dst
    ROR(Target, Target, Size),
    // Rotate Left with Extend: Dst rotated by n -> Dst
    ROXL(Target, Target, Size),
    // Rotate Right with Extend: Dst rotated by n -> Dst
    ROXR(Target, Target, Size),
    // Return with Displacement: (SP) -> PC, SP + 4 + d -> SP
    RTD(WordLabel),
    // Return from Exception: if supervisor state then (SP) -> SR, SP + 2 -> SP, (SP) -> PC, SP + 4 -> SP else TRAP
    RTE,
    // Return and Restore: (SP) -> CCR, SP + 2 -> SP, (SP) -> PC, SP + 4 -> SP
    RTR,
    // Return from Subroutine: (SP) -> PC, SP + 4 -> SP
    RTS,
    // Subtract Decimal with Extend: Dst - Src - X -> Dst
    SBCD(Target, Target),
    // Set According to Condition: If CC then 1's -> Dst else 0's -> Dst
    Scc(Condition, Target),
    // Stop: Immediate Data -> SR, STOP
    STOP(u16),
    // Subtract: Dst - Src -> Dst
    SUB(Target, Target, Size),
    // Subtract Address: Dst - Src -> Dst
    SUBA(Target, Registers, Size),
    // Subtract Immediate: Dst - Immediate Data -> Dst
    SUBI(u32, Target, Size),
    // Subract Quick: Dst - Immediate Data -> Dst
    SUBQ(u32, Target, Size),
    // Subtract with Extend: Dst - Src - X -> DSt
    SUBX(Target, Target, Size),
    // Swap Data Register Halves: Register [31:16] <-> Register [15:0]
    SWAP(Registers),
    // Test and Set Operand: Dst Tested -> CC, 1 -> (7th bit of Dst)
    TAS(Target),
    // Trap: SSP - 2 -> SSP, Format/Offset -> (SSP), SPP - 4 -> SSP, PC -> (SSP), SSP -2 -> SSP, SR -> (SSP), Vector Address -> PC
    TRAP(u8),
    // Trap on Overflow: if V then TRAP
    TRAPV,
    // Test: Dst Tested -> CC
    TST(Target, Size),
    // Unlink: An -> Sp, Sp -> An
    UNLK(Registers),
    // Not Implemented
    NotImplemented,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::DnDirect(reg) => write!(f, "%d{}", reg),
            Target::AnDirect(reg) => write!(f, "%a{}", reg),
            Target::AnIndirect(reg) => write!(f, "(%a{})", reg),
            Target::AnIndirectPostInc(reg) => write!(f, "(%a{}) +", reg),
            Target::AnIndirectPreDec(reg) => write!(f, "- (%a{})", reg),
            Target::AnIndirectDisplacement(disp, reg) => write!(f, "({}_16, %a{})", disp, reg),
            Target::AnIndirectIndex(disp, rega, regidx) => write!(
                f,
                "({}_8, %a{}, %x{}.{}*{})",
                disp, rega, regidx.register, regidx.size, regidx.scale
            ),
            Target::AnIndirectIndexBase(bdisp, rega, regidx) => write!(
                f,
                "({}, %a{}, %x{}.{}*{})",
                bdisp, rega, regidx.register, regidx.size, regidx.scale
            ),
            Target::MemoryIndirectPostIndex(bdisp, rega, regidx, odisp) => write!(
                f,
                "([{}, %a{}], %x{}.{}*{}, {})",
                bdisp, rega, regidx.register, regidx.size, regidx.scale, odisp
            ),
            Target::MemoryIndirectPreIndex(bdisp, rega, regidx, odisp) => write!(
                f,
                "([{}, %a{}, %x{}.{}*{}], {})",
                bdisp, rega, regidx.register, regidx.size, regidx.scale, odisp
            ),
            Target::PCIndirectDisplacement(disp, pc) => write!(f, "({}_16, %pc{})", disp, pc),
            Target::PCIndirectIndex(disp, pc, regidx) => write!(
                f,
                "({}_8, %pc{}, %x{}.{}*{})",
                disp, pc, regidx.register, regidx.size, regidx.scale
            ),
            Target::PCIndirectIndexBase(bdisp, pc, regidx) => write!(
                f,
                "({}, %pc{}, %x{},{}*{})",
                bdisp, pc, regidx.register, regidx.size, regidx.scale
            ),
            Target::PCMemoryIndirectPostIndex(bdisp, pc, regidx, odisp) => write!(
                f,
                "([{}, %pc{}], %x{}.{}*{}, {})",
                bdisp, pc, regidx.register, regidx.size, regidx.scale, odisp
            ),
            Target::PCMemoryIndirectPreIndex(bdisp, pc, regidx, odisp) => write!(
                f,
                "([{}, %pc{}, %x{}.{}*{}], {})",
                bdisp, pc, regidx.register, regidx.size, regidx.scale, odisp
            ),
            Target::AbsoluteShortAddress(extw) => write!(f, "({}).W", extw),
            Target::AbsoluteLongAddress(addrh, addrl) => {
                write!(f, "({}).L", (addrh as u32) << 16 | addrl)
            }
            Target::Immediate(value) => write!(f, "#<{}>", value),
        }
    }
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instructions::ABCD(src, dst) => write!(f, "ABCD\t {}, {}", src, dst),
            Instructions::ADD(src, dst, size) => write!(f, "ADD{}\t {}, {}", size, src, dst),
            Instructions::ADDA(src, dst, size) => write!(f, "ADDA{}\t {}, {}", size, src, dst),
            Instructions::ADDI(imm, dst, size) => write!(f, "ADDI{}\t #<{}>, {}", size, imm, dst),
            Instructions::ADDQ(imm, dst, size) => write!(f, "ADDQ{}\t #<{}>, {}", size, imm, dst),
            Instructions::ADDX(src, dst, size) => write!(f, "ADDX{}\t {}, {}", size, src, dst),
            Instructions::AND(src, dst, size) => write!(f, "AND{}\t {}, {}", size, src, dst),
            Instructions::ANDI(imm, dst, size) => write!(f, "ANDI{}\t #<{}>, {}", size, imm, dst),
            Instructions::ASL(src, dst, size) => write!(f, "ASL{}\t {}, {}", size, src, dst),
            Instructions::ASR(src, dst, size) => write!(f, "ASR{}\t {}, {}", size, src, dst),
            Instructions::Bcc(cond, label) => write!(f, "Bcc {}\t {}", cond, label),
            Instructions::BCHG(src, dst, size) => write!(f, "BCHG{}\t {}, {}", size, src, dst),
            Instructions::BCLR(src, dst, size) => write!(f, "BCLR{}\t {}, {}", size, src, dst),
            Instructions::BRA(label) => write!(f, "BRA\t {}", label),
            Instructions::BSET(src, dst, size) => write!(f, "BSET{}\t {}, {}", size, src, dst),
            Instructions::BSR(label) => write!(f, "BSR\t  {}", label),
            Instructions::BTST(src, dst, size) => write!(f, "BTST{}\t {}, {}", size, src, dst),
            Instructions::CHK(src, reg, size) => write!(f, "CHK{}\t {}, {}", size, src, dst),
            Instructions::CLR(dst, size) => write!(f, "CLR{}\t {}", size, dst),
            Instructions::CMP(src, dst, size) => write!(f, "CMP{}\t {}, {}", size, src, dst),
            Instructions::CMPA(src, reg, size) => write!(f, "CMPA{}\t {}, {}", size, reg, dst),
            Instructions::CMPI(imm, dst, size) => write!(f, "CMPI{}\t {}, {}", size, imm, dst),
            Instructions::CMPM(src, dst, siz) => write!(f, "CMPM{}\t {}, {}", size, src, dst),
            Instructions::DBcc(cond, reg, label) => {
                write!(f, "DBcc\t {}\t {}\t {}", cond, reg, label)
            }
            Instructions::DIVSW(src, reg, size) => write!(f, "DIVSW{}\t {}, {}", size, src, reg),
            Instructions::DIVUW(src, reg, size) => write!(f, "DIVUW{}\t {}, {}", size, src, reg),
            Instructions::EOR(src, dst, size) => write!(f, "EOR{}\t {}, {}", size, src, dst),
            Instructions::EORtoCCR(imm) => write!(f, "EORtoCCR\t #<{}>, CCR", imm),
            Instructions::EORtoSR(imm) => write!(f, "EORtoCCR\t #<{}>, CCR", imm),
            Instructions::EORI(imm, dst, size) => write!(f, "EORI{}\t #<{}>, {}", size, imm, dst),
            Instructions::EXG(src, dst) => write!(f, "EXG\t {}, {}", src, dst),
            Instructions::EXT(reg, size, size) => write!(f, "EXT\t {}\t {}\t {}", reg, size, size),
            Instructions::ILLEGAL => write!(f, "ILLEGAL"),
            Instructions::JMP(dst) => write!(f, "JMP\t {}", dst),
            Instructions::JSR(dst) => write!(f, "JSR\t {}", dst),
            Instructions::LEA(src, reg) => write!(f, "LEA\t {}, {}", src, reg),
            Instructions::LINK(reg, disp) => write!(f, "LINK\t {}, {}", reg, disp),
            Instructions::LSL(src, dst, size) => write!(f, "LSL{}\t {}, {}", size, src, dst),
            Instructions::LSR(src, dst, size) => write!(f, "LSL{}\t {}, {}", size, src, dst),
            Instructions::MOVE(src, dst, size) => write!(f, "MOVE{}\t {}, {}", size, src, dst),
            Instructions::MOVEA(src, dst, size) => write!(f, "MOVEA{}\t {}, {}", size, src, dst),
            Instructions::MOVEfromCCR(dst) => write!(f, "MOVEfromCCR\t {}", dst),
            Instructions::MOVEtoCCR(src) => write!(f, "MOVEtoCCR\t {}", src),
            Instructions::MOVEfromSR(dst) => write!(f, "MOVEfromSR\t {}", dst),
            Instructions::MOVEtoSR(src) => write!(f, "MOVEtoSR\t {}", src),
            Instructions::MOVEUSP(targ, dir) => write!(f, "MOVEUSP\t {}\t {}", targ, dir),
            Instructions::MOVEC(targ, control, dir) => {
                write!(f, "MOVEC\t {}, {}\t {}", targ, control, dir)
            }
            Instructions::MOVEM(targ, size, label, dir) => {
                write!(f, "MOVEM{}\t {}\t {}\t {}", size, targ, label, dir)
            }
            Instructions::MOVEP(src, dst, label, size, dir) => {
                write!(f, "MOVEP{}\t {}, {}\t {}\t {}", size, src, dst, label, dir)
            }
            Instructions::MOVEQ(imm, reg) => write!(f, "MOVEQ\t #<{}>, {}", imm, reg),
            Instructions::MULSW(src, reg) => write!(f, "MULSW\t {}, {}", src, reg),
            Instructions::MULUW(src, reg) => write!(f, "MULUW\t {}, {}", src, reg),
            Instructions::NBCD(dst) => write!(f, "NBCD\t {}", dst),
            Instructions::NEG(dst, size) => write!(f, "NEG{}\t {}", size, dst),
            Instructions::NEGX(dst, size) => write!(f, "NEGX{}\t {}", size, dst),
            Instructions::NOP => write!(f, "NOP"),
            Instructions::NOT(dst, size) => write!(f, "NOT{}\t {}", size, dst),
            Instructions::OR(src, dst, size) => write!(f, "OR{}\t {}, {}", size, src, dst),
            Instructions::ORI(imm, dst, size) => write!(f, "ORI{}\t #<{}>, {}", size, imm, dst),
            Instructions::ORItoCCR(imm) => write!(f, "ORItoCCR\t #<{}>, CCR", imm),
            Instructions::ORItoSR(imm) => write!(f, "ORItoSR\t #<{}>, CCR", imm),
            Instructions::PEA(targ) => write!(f, "PEA\t {}", targ),
            Instructions::RESET => write!(f, "RESET"),
            Instructions::ROL(src, dst, size) => write!(f, "ROL{}\t {}, {}", size, src, dst),
            Instructions::ROR(src, dst, size) => write!(f, "ROR{}\t {}, {}", size, src, dst),
            Instructions::ROXL(src, dst, size) => write!(f, "ROXL{}\t {}, {}", size, src, dst),
            Instructions::ROXR(src, dst, size) => write!(f, "ROXR{}\t {}, {}", size, src, dst),
            Instructions::RTD(label) => write!(f, "RTD\t {}", label),
            Instructions::RTE => write!(f, "RTE"),
            Instructions::RTR => write!(f, "RTR"),
            Instructions::RTS => write!(f, "RTS"),
            Instructions::SBCD(src, dst) => write!(f, "SBCD\t {}, {}", src, dst),
            Instructions::Scc(cond, dst) => write!(f, "Scc\t {}\t {}", cond, dst),
            Instructions::STOP(imm) => write!(f, "STOP\t #<{}>", imm),
            Instructions::SUB(src, dst, size) => write!(f, "SUB{}\t {}, {}", size, src, dst),
            Instructions::SUBA(src, reg, size) => write!(f, "SUBA{}\t {}, {}", size, src, reg),
            Instructions::SUBI(imm, dst, size) => write!(f, "SUBI{}\t #<{}>, {}", size, imm, dst),
            Instructions::SUBQ(imm, dst, size) => write!(f, "SUBQ{}\t #<{}>, {}", size, imm, dst),
            Instructions::SUBX(src, dst, size) => write!(f, "SUBX{}\t {}, {}", site, src, dst),
            Instructions::SWAP(reg) => write!(f, "SWAP\t {}", reg),
            Instructions::TAS(dst) => write!(f, "TAS\t {}", dst),
            Instructions::TRAP(imm) => write!(f, "TRAP\t #<{}>", imm),
            Instructions::TRAPV => write!(f, "TRAPV"),
            Instructions::TST(dst, size) => write!(f, "TST{}\t {}", size, dst),
            Instructions::UNLK(reg) => write!(f, "UNLNK\t {}", reg),
            Instructions::NotImplemented => write!(f, "Instruction not implemented!"),
        }
    }
}
