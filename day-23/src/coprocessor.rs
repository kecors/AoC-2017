use instruction::Instruction;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Coprocessor {
    code_segment: Vec<Instruction>,
    instruction_pointer: i64,
    registers: HashMap<char, i64>,
    mul_count: u32
}

impl Coprocessor {
    pub fn new(instructions: Vec<Instruction>) -> Coprocessor {
        let mut registers: HashMap<char, i64> = HashMap::new();
        for letter in "abcdefgh".chars() {
            registers.insert(letter, 0);
        }

        Coprocessor {
            code_segment: instructions,
            instruction_pointer: 0,
            registers: registers,
            mul_count: 0
        }
    }

    pub fn execute(&mut self) {
        let mut jumped: bool = false;
        loop {
            if self.instruction_pointer as usize >= self.code_segment.len() {
                println!("part 1: mul count = {}", self.mul_count);
                return;
            }
            match self.code_segment[self.instruction_pointer as usize] {
                Instruction::NoOp => {
                },
                Instruction::SetR(register1, register2) => {
                    let mut new_value = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        new_value = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value = new_value;
                    }
                },
                Instruction::SetN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value = number;
                    };
                },
                Instruction::SubR(register1, register2) => {
                    let mut minuend = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        minuend = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value -= minuend;
                    }
                },
                Instruction::SubN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value -= number;
                    }
                },
                Instruction::MulR(register1, register2) => {
                    let mut factor = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        factor = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value *= factor;
                        self.mul_count += 1;
                    }
                },
                Instruction::MulN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value *= number;
                        self.mul_count += 1;
                    }
                },
                Instruction::JnzRR(register1, register2) => {
                    let mut x = 0;
                    if let Some(value) = self.registers.get(&register1) {
                        x = *value;
                    }
                    if let Some(offset) = self.registers.get(&register2) {
                        if x != 0 {
                            self.instruction_pointer += *offset;
                            jumped = true;
                        }
                    }
                },
                Instruction::JnzRN(register, number) => {
                    let mut x = 0;
                    if let Some(value) = self.registers.get(&register) {
                        x = *value;
                    }
                    if x != 0 {
                        self.instruction_pointer += number;
                        jumped = true;
                    }
                },
                Instruction::JnzNR(number, register) => {
                    if let Some(offset) = self.registers.get(&register) {
                        if number != 0 {
                            self.instruction_pointer += *offset;
                            jumped = true;
                        }
                    }
                },
                Instruction::JnzNN(number1, number2) => {
                    if number1 != 0 {
                        self.instruction_pointer += number2;
                        jumped = true;
                    }
                }
            }
            if jumped == false {
                self.instruction_pointer += 1;
            }
            jumped = false;
            // Since the puzzle presumes that rcv will be executed, ignore
            // the possibility of continuing or jumping off either end
            // of the progam.
        }
    }
}
