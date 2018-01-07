use instruction::Instruction;

use std::collections::HashMap;

use std::sync::mpsc::{Sender, Receiver};

#[derive(Debug)]
pub struct Duet {
    code_segment: Vec<Instruction>,
    instruction_pointer: i64,
    registers: HashMap<char, i64>,
    frequency: i64,
    program_id: i64,
    tx: Sender<i64>,
    rx: Receiver<i64>,
    send_counter: u32,
    part2_flag: bool
}

impl Duet {
    pub fn new(instructions: Vec<Instruction>, program_id: i64, tx: Sender<i64>, rx: Receiver<i64>, part2_flag: bool) -> Duet {
        let mut registers: HashMap<char, i64> = HashMap::new();
        for instruction in instructions.iter() {
            match instruction {
                &Instruction::NoOp => {
                },
                &Instruction::SndR(register) => {
                    registers.entry(register).or_insert(0);
                }
                &Instruction::SndN(_) => {
                },
                &Instruction::SetR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::SetN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::AddR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::AddN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::MulR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::MulN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::ModR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::ModN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::RcvR(register) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::RcvN(_) => {
                },
                &Instruction::JgzRR(register1, register2) => {
                    registers.entry(register1).or_insert(0);
                    registers.entry(register2).or_insert(0);
                },
                &Instruction::JgzRN(register, _) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::JgzNR(_, register) => {
                    registers.entry(register).or_insert(0);
                },
                &Instruction::JgzNN(_, _) => {
                }
            }
        }

        if part2_flag == true {
            if let Some(value) = registers.get_mut(&'p') {
                *value = program_id;
            }
        }

        Duet {
            code_segment: instructions,
            instruction_pointer: 0,
            registers: registers,
            frequency: 0,
            program_id: program_id,
            tx: tx,
            rx: rx,
            send_counter: 0,
            part2_flag: part2_flag
        }
    }

    pub fn execute(&mut self) {
        let mut jumped: bool = false;
        loop {
            match self.code_segment[self.instruction_pointer as usize] {
                Instruction::NoOp => {
                },
                Instruction::SndR(register) => {
                    if self.part2_flag == false {
                        if let Some(value) = self.registers.get(&register) {
                            self.frequency = *value;
                        }
                    } else {
                        if let Some(value) = self.registers.get(&register) {
                            self.tx.send(*value).unwrap();
                            self.send_counter += 1;
                            println!("[{}] ({}) send = {}", 
                                     self.program_id, 
                                     self.send_counter, 
                                     *value);
                        };
                    }
                },
                Instruction::SndN(number) => {
                    if self.part2_flag == false {
                        self.frequency = number;
                    } else {
                        self.tx.send(number).unwrap();
                        self.send_counter += 1;
                        println!("[{}] ({}) send = {}", 
                                 self.program_id, 
                                 self.send_counter, 
                                 number);
                    }
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
                Instruction::AddR(register1, register2) => {
                    let mut addend = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        addend = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value += addend;
                    }
                },
                Instruction::AddN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value += number;
                    }
                },
                Instruction::MulR(register1, register2) => {
                    let mut factor = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        factor = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value *= factor;
                    }
                },
                Instruction::MulN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value *= number;
                    }
                },
                Instruction::ModR(register1, register2) => {
                    let mut divisor = 0;
                    if let Some(value) = self.registers.get(&register2) {
                        divisor = *value;
                    }
                    if let Some(value) = self.registers.get_mut(&register1) {
                        *value = *value % divisor;
                    }
                },
                Instruction::ModN(register, number) => {
                    if let Some(value) = self.registers.get_mut(&register) {
                        *value = *value % number;
                    }
                },
                Instruction::RcvR(register) => {
                    if self.part2_flag == false {
                        if let Some(value) = self.registers.get(&register) {
                            if *value != 0 {
                                println!("part 1: frequency = {}", 
                                         self.frequency);
                                return;
                            }
                        }
                    } else {
                        if let Some(value) = self.registers.get_mut(&register) {
                            *value = self.rx.recv().unwrap();
                            println!("[{}] ({}) recv = {}", 
                                     self.program_id, 
                                     self.send_counter, 
                                     *value);
                        }
                    }
                },
                Instruction::RcvN(number) => {
                    if self.part2_flag == false {
                        if number != 0 {
                            println!("part 1: frequency = {}",
                                     self.frequency);
                            return;
                        }
                    } else {
                        unimplemented!("RcvN");
                    }
                },
                Instruction::JgzRR(register1, register2) => {
                    let mut x = 0;
                    if let Some(value) = self.registers.get(&register1) {
                        x = *value;
                    }
                    if let Some(offset) = self.registers.get(&register2) {
                        if x > 0 {
                            self.instruction_pointer += *offset;
                            jumped = true;
                        }
                    }
                },
                Instruction::JgzRN(register, number) => {
                    let mut x = 0;
                    if let Some(value) = self.registers.get(&register) {
                        x = *value;
                    }
                    if x > 0 {
                        self.instruction_pointer += number;
                        jumped = true;
                    }
                },
                Instruction::JgzNR(number, register) => {
                    if let Some(offset) = self.registers.get(&register) {
                        if number > 0 {
                            self.instruction_pointer += *offset;
                            jumped = true;
                        }
                    }
                },
                Instruction::JgzNN(number1, number2) => {
                    if number1 > 0 {
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
