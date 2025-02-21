use std::collections::VecDeque;

use crate::instr::Instr;
use crate::op;

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Integer(i64),
}

pub struct VM {
    stack: VecDeque<Value>,
}

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Invalid number of bytes : {0} (must be divisible by 8)")]
    InvalidNumberOfBytes(usize),

    #[error("Stack Empty")]
    StackEmpty,

    #[error("Invalid Type : Expected {0:?}, got {1:?}")]
    InvalidType(Value, Value),

    #[error("Division By Zero")]
    DivByZero,

    #[error("Division (modulo) by zero")]
    ModByZero,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, bytecode: Vec<Instr>) -> anyhow::Result<(), InterpreterError> {
        let mut pc = 0;
        let prog_length = bytecode.len();
        loop {
            let instr = &bytecode[pc];
            let mut nextpc = pc + 1;
            println!(">> Stack: (bottom) {:?} (top)", self.stack);
            match instr.op {
                op::Op::Pop => {
                    _ = self.stack.pop_back();
                    println!("Pop");
                }
                op::Op::Add => {
                    println!("> add ");
                    self.apply_transform_to_two_integers(|a, b| Ok(a + b))?;
                }
                op::Op::Inc => {
                    println!("> inc ");
                    self.apply_transform_to_top_integer(|a| *a += 1)?;
                }
                op::Op::Dec => {
                    println!("> dec ");
                    self.apply_transform_to_top_integer(|a| *a -= 1)?;
                }
                op::Op::Sub => {
                    println!("> sub ");
                    self.apply_transform_to_two_integers(|a, b| Ok(a - b))?;
                }
                op::Op::Mul => {
                    println!("> mul ");
                    self.apply_transform_to_two_integers(|a, b| Ok(a * b))?;
                }
                op::Op::Div => {
                    println!("> siv ");
                    self.apply_transform_to_two_integers(|a, b| {
                        if b == 0 {
                            Err(InterpreterError::DivByZero)
                        } else {
                            Ok(a / b)
                        }
                    })?;
                }
                op::Op::Mod => {
                    println!("> mod ");
                    self.apply_transform_to_two_integers(|a, b| {
                        if b == 0 {
                            Err(InterpreterError::ModByZero)
                        } else {
                            Ok(a % b)
                        }
                    })?;
                }
                op::Op::Print => {
                    println!("> print");
                    if let Some(value) = self.stack.pop_back() {
                        match value {
                            Value::Integer(int) => println!("{}", int),
                        }
                    }
                }
                op::Op::Halt => {
                    println!("> halt");
                    return Ok(());
                }
                op::Op::Dup => {
                    println!("> dup");
                    if let Some(top) = self.stack.back() {
                        self.stack.push_back(*top);
                    } else {
                        return Err(InterpreterError::StackEmpty);
                    }
                }
                op::Op::Dup2 => todo!(),
                op::Op::Swap => todo!(),
                op::Op::Clear => todo!(),
                op::Op::Over => {
                    println!("> over");
                    let len = self.stack.len();
                    if len < 2 {
                        return Err(InterpreterError::StackEmpty);
                    }
                    self.stack.push_back(self.stack[len - 2]);
                }
                op::Op::Push => {
                    self.stack.push_back(Value::Integer(instr.value));
                    println!("Push {}", instr.value);
                }
                op::Op::Je => todo!(),
                op::Op::Jn => todo!(),
                op::Op::Jg => todo!(),
                op::Op::Jl => todo!(),
                op::Op::Jge => todo!(),
                op::Op::Jle => todo!(),
                op::Op::Jmp => {
                    nextpc = instr.value as usize;
                }
                op::Op::Jz => todo!(),
                op::Op::Jnz => todo!(),
                // _ => todo!("Op {:?}", instr.op),
            }
            if nextpc > 0 && nextpc < prog_length {
                pc = nextpc;
            } else {
                panic!("Jumping out of bounds")
            }
        }
        // Ok(())
    }

    fn pop_stack_top(&mut self) -> Result<Value, InterpreterError> {
        self.stack.pop_back().ok_or(InterpreterError::StackEmpty)
    }

    fn apply_transform_to_two_integers(
        &mut self,
        func: impl Fn(i64, i64) -> Result<i64, InterpreterError>,
    ) -> Result<(), InterpreterError> {
        let b = self.pop_stack_top()?;
        let a = self.pop_stack_top()?;
        if let Value::Integer(b) = b {
            if let Value::Integer(a) = a {
                self.stack.push_back(Value::Integer(func(a, b)?));
            } else {
                return Err(InterpreterError::InvalidType(Value::Integer(0), a));
            }
        } else {
            return Err(InterpreterError::InvalidType(Value::Integer(0), b));
        }
        Ok(())
    }

    fn apply_transform_to_top_integer(
        &mut self,
        func: impl Fn(&mut i64) -> (),
    ) -> Result<(), InterpreterError> {
        if let Some(mut back_value) = self.stack.back_mut() {
            if let Value::Integer(back) = &mut back_value {
                Ok(func(back))
            } else {
                Err(InterpreterError::InvalidType(
                    Value::Integer(0),
                    back_value.clone(),
                ))
            }
        } else {
            return Err(InterpreterError::StackEmpty);
        }
    }
}

const INSTR_SIZE: usize = 8;
pub fn decode_instructions(bytes: Vec<u8>) -> anyhow::Result<Vec<Instr>> {
    if bytes.len() % 8 != 0 {
        return Err(InterpreterError::InvalidNumberOfBytes(bytes.len()).into());
    }
    let mut result: Vec<Instr> = Vec::with_capacity(bytes.len() / 8);

    for n in 0..(bytes.len() / INSTR_SIZE) {
        let mut bytes8 = [0; 8];
        bytes8[0..].copy_from_slice(&bytes[n * INSTR_SIZE..(n + 1) * INSTR_SIZE]);
        result.push(Instr::from_u64(u64::from_le_bytes(bytes8)));
    }

    Ok(result)
}

pub fn encode_instructions(bytecode: &[Instr]) -> anyhow::Result<Vec<u8>> {
    let mut result: Vec<u8> = Vec::with_capacity(bytecode.len() * 8);

    for instr in bytecode {
        let encoded = &instr.to_u64();
        for byte in encoded.to_be_bytes() {
            result.push(byte);
        }
    }
    Ok(result)
}
