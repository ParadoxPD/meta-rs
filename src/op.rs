use strum::FromRepr;

#[repr(u8)]
#[derive(Debug, FromRepr, Copy, Clone, PartialEq, PartialOrd)]
pub enum Op {
    // operations without arguments
    Pop = 0x01,
    Add,
    Inc,
    Dec,
    Sub,
    Mul,
    Div,
    Mod,
    Print,
    Halt,
    Dup,
    Dup2,
    Swap,
    Clear,
    Over,

    // operations with arguments
    Push,
    Je,
    Jn,
    Jg,
    Jl,
    Jge,
    Jle,
    Jmp,
    Jz,
    Jnz,
}
