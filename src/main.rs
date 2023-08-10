use std::{env, fs, u8};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: brainduck <path>");
    }
    
    let filepath = &args[1];
    let contents = fs::read_to_string(filepath).expect("error reading!");
    
    let mut interpreter = Interpreter::new(&contents);
    interpreter.interpret();
}


pub struct Interpreter {
    instructions: Vec<char>,
    pc: usize, // program counter
    memory: Vec<u8>,
    pmem: usize, // memory pointer
}

impl Interpreter {
    const MEM_SIZE: usize = 30000;
    pub fn new(instructions: &String) -> Self {
        Interpreter { instructions: instructions.chars().collect(), pc: 0, memory: vec![0; Interpreter::MEM_SIZE], pmem: 0}
    }

    pub fn interpret(&mut self) {
        let mut st: Vec<usize> = Vec::new();
        loop {
            if self.pc >= self.instructions.len() {
                break;

            }
            let instruction = &self.instructions[self.pc];
            // println!("{}", instruction);
            match instruction {
                '>' => self.pmem = if self.pmem == Interpreter::MEM_SIZE - 1 { usize::MIN } else { self.pmem + 1 },
                '<' => self.pmem = if self.pmem == usize::MIN { Interpreter::MEM_SIZE - 1 } else { self.pmem - 1 },
                '+' => self.memory[self.pmem] = if self.memory[self.pmem] == u8::MAX { u8::MIN } else { self.memory[self.pmem] + 1 },
                '-' => self.memory[self.pmem] = if self.memory[self.pmem] == u8::MIN { u8::MAX } else { self.memory[self.pmem] - 1 },
                '.' => print!("{}", self.memory[self.pmem] as char),
                ',' => {}, // TODO
                '[' => {
                    if self.memory[self.pmem] == 0 {
                        while self.pc < self.instructions.len() && self.instructions[self.pc] != ']' {
                            self.pc = self.pc + 1;
                        }
                    } else {
                        st.push(self.pc);
                    }
                },
                ']' => {
                    if self.memory[self.pmem] != 0 {
                        self.pc = st.pop().unwrap() - 1;
                    } else {
                        st.pop();
                    }
                }
                _ => {},
            }
            self.pc = self.pc + 1;
        }
    }
}
