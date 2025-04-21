use crate::vm::input_buffering;

use std::convert::TryFrom;
use std::fs::File;
use std::env::{self, args};
use std::io::{self, stdout, Write, Read};
use input_buffering::{check_key, restore_input_buffering, setup};

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum Registers {
    R_R0 = 0,
    R_R1,
    R_R2,
    R_R3,
    R_R4,
    R_R5,
    R_R6,
    R_R7,
    R_PC,
    R_COND,
    R_COUNT = 10,
}

const MR_KBSR: u16 = 0xFE00; // Keyboard Status Register
const MR_KBDR: u16 = 0xFE02; // Keyboard Data Register

const MEMORY_SIZE: usize=1<<16; 


//R_COND condition flags
#[derive(Debug, Clone, Copy)]
enum R_COND {
    FL_POS = 1 << 0,
    FL_ZRO = 1 << 1,
    FL_NEG = 1 << 2,
}
//16 bits each
#[repr(u16)]
enum Opcodes {
    OP_BR = 0, /* branch */
    OP_ADD,    /* add  */
    OP_LD,     /* load */
    OP_ST,     /* store */
    OP_JSR,    /* jump register */
    OP_AND,    /* bitwise and */
    OP_LDR,    /* load register */
    OP_STR,    /* store register */
    OP_RTI,    /* unused */
    OP_NOT,    /* bitwise not */
    OP_LDI,    /* load indirect */
    OP_STI,    /* store indirect */
    OP_JMP,    /* jump */
    OP_RES,    /* reserved (unused) */
    OP_LEA,    /* load effective address */
    OP_TRAP,   /* execute trap */
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]

enum OP_TRAP {
    TRAP_GETC = 0x20,  /* get character from keyboard, not echoed onto the terminal */
    TRAP_OUT = 0x21,   /* output a character */
    TRAP_PUTS = 0x22,  /* output a word string */
    TRAP_IN = 0x23,    /* get character from keyboard, echoed onto the terminal */
    TRAP_PUTSP = 0x24, /* output a byte string */
    TRAP_HALT = 0x25,  /* halt the program */
}
impl TryFrom<u16> for Opcodes {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcodes::OP_BR),
            1 => Ok(Opcodes::OP_ADD),
            2 => Ok(Opcodes::OP_LD),
            3 => Ok(Opcodes::OP_ST),
            4 => Ok(Opcodes::OP_JSR),
            5 => Ok(Opcodes::OP_AND),
            6 => Ok(Opcodes::OP_NOT),
            7 => Ok(Opcodes::OP_LDR),
            8 => Ok(Opcodes::OP_STR),
            9 => Ok(Opcodes::OP_RTI),
            10 => Ok(Opcodes::OP_LDI),
            11 => Ok(Opcodes::OP_STI),
            12 => Ok(Opcodes::OP_JMP),
            13 => Ok(Opcodes::OP_RES),
            14 => Ok(Opcodes::OP_LEA),
            15 => Ok(Opcodes::OP_TRAP),
            _ => Err(()), // Return an error for invalid opcodes
        }
    }
}
impl OP_TRAP {
    pub fn from_u16(val: u16) -> Option<Self> {
        match val {
            0x20 => Some(OP_TRAP::TRAP_GETC),
            0x21 => Some(OP_TRAP::TRAP_OUT),
            0x22 => Some(OP_TRAP::TRAP_PUTS),
            0x23 => Some(OP_TRAP::TRAP_IN),
            0x24 => Some(OP_TRAP::TRAP_PUTSP),
            0x25 => Some(OP_TRAP::TRAP_HALT),
            _ => None,
        }
    }
}


pub fn sign_extend( value: u16, bit_count: u16) -> u16 {
        if (value >> (bit_count - 1)) & 1 == 1 {
            value | (0xFFFF << bit_count)
        } else {
            value
        }
    }
#[derive(Debug, Clone)]
pub struct VM {
    pub memory: [u16; 1 << 16],
    pub registers_storage: [u16; Registers::R_COUNT as usize],
}


impl VM {
    pub fn new() -> Self {
        Self {
            memory: [0;MEMORY_SIZE],
         registers_storage: [0; Registers::R_COUNT as usize],
        }
    }
    fn load_arguments(&mut self){
    let args: Vec<String> = env::args().collect();

    // Check if at least one image file is passed
    if args.len() < 2 {
        eprintln!("Usage: lc3 [image-file1] ...");
        std::process::exit(2);
    }


    for filename in &args[1..] {
        match File::open(filename) {
            Ok(file) => {
                if let Err(e) = self.read_image_file(file) {
                    eprintln!("Failed to load image: {} ({})", filename, e);
                    std::process::exit(1);
                }
            }
            Err(_) => {
                eprintln!("Could not open file: {}", filename);
                std::process::exit(1);
            }
        }
    }

}

  

    pub fn run(&mut self) {
        setup();
            let mut vm = VM::new();

        self.load_arguments();
      

        self.registers_storage[Registers::R_COND as usize] = R_COND::FL_ZRO as u16;//
        let pc_start: u16 = 0x3000;
        self.registers_storage[Registers::R_PC as usize] = pc_start;//.


        let mut running = true;
        while running {
            let instr = self.memory_read(self.registers_storage[Registers::R_PC as usize]);
            self.registers_storage[Registers::R_PC as usize] += 1;

            let opcode = instr >> 12;
            match Opcodes::try_from(opcode) {
                Ok(Opcodes::OP_ADD) => {
                    self.add(instr);
                    break;
                }
                Ok(Opcodes::OP_AND) => {
                    self.and(instr);
                    break;
                }
                Ok(Opcodes::OP_NOT) => {
                    self.not(instr);
                    break;
                }
                Ok(Opcodes::OP_BR) => {
                    self.branch(instr);
                    break;
                }
                Ok(Opcodes::OP_LD) => {self.load(instr);
                break;}

                Ok(Opcodes::OP_ST) => {self.store(instr);
                break;},
                Ok(Opcodes::OP_JSR) => {
                    self.jump_register(instr);
                    break;
                }
                Ok(Opcodes::OP_LDR) =>{ self.load_register(instr);
                break;},
                Ok(Opcodes::OP_STR) => {self.store_register(instr);
                break;},
                Ok(Opcodes::OP_RTI) => {
                    break;
                }
                Ok(Opcodes::OP_LDI) => {
                    self.ldi(instr);
                    break;
                }
                Ok(Opcodes::OP_STI) => {self.store_indirect(instr);
                break;},
                Ok(Opcodes::OP_JMP) => {
                    self.jump(instr);
                    break;
                }
                Ok(Opcodes::OP_RES) => {
                    break;
                }
                Ok(Opcodes::OP_LEA) =>{ self.lea(instr);
                break;},
                Ok(Opcodes::OP_TRAP) => {
                    self.registers_storage[Registers::R_R7 as usize] =
                        self.registers_storage[Registers::R_PC as usize];
                    let instr = instr & 0xFF;
                    if let Some(trap_vector) = OP_TRAP::from_u16(instr) {
                        match trap_vector {
                            OP_TRAP::TRAP_GETC => {
                                self.trap_getc(None);
                            }
                            OP_TRAP::TRAP_OUT => {
                              self.trap_out();
                            }
                            OP_TRAP::TRAP_PUTS => {
                                self.trap_puts();
                            }
                            OP_TRAP::TRAP_IN => {
                                self.trap_in();
                            }
                            OP_TRAP::TRAP_PUTSP => {
                                self.trap_putsp();
                             
                            }
                            OP_TRAP::TRAP_HALT => {
                                    println!("Halting the program...");
                                    io::stdout().flush().unwrap();
                                    running = false;
                                }
                            
                        }
                    } else {
                        println!("Invalid TRAP vector: {}", instr);
                    }
                    break;
                }
                Err(_) => {
                      eprintln!("Invalid opcode encountered: {:#X}", opcode);
                    break;
                }
   
            }
                   
  
     
          
        }
        restore_input_buffering();
    
    }
        pub fn swap16(x: u16) -> u16 {
        x << 8 | x >> 8
    }

    pub fn read_image(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        self.read_image_file(file)
    }
    pub fn read_image_file(&mut self, mut file: File) -> io::Result<()> {
        // Read origin (2 bytes)
        let mut origin_buf = [0u8; 2];
        file.read_exact(&mut origin_buf)?;
       let origin = u16::from_be_bytes(origin_buf) as usize;
        println!("origin {:}", origin);

        // Read the rest of the image into memory starting at `origin`
        let max_read = MEMORY_SIZE - origin;
        let mut buffer = vec![0u8; max_read * 2]; // Each word is 2 bytes
        let read_bytes = file.read(&mut buffer)?;

        // Convert bytes to u16 and store in memory
        for i in 0..(read_bytes / 2) {
            let byte_pair = [buffer[i * 2], buffer[i * 2 + 1]];
            self.memory[origin + i] = u16::from_be_bytes(byte_pair);
        }

        Ok(())
    }
    pub fn trap_puts(&mut self) {
        let mut address = self.registers_storage[Registers::R_R0 as usize];
        loop {
            let c = self.memory_read(address);
            if c == 0 {
                break;
            }
            print!("{}", c as u8 as char);

            address += 1;
        }
        stdout().flush().unwrap();
    }
    pub fn trap_putsp(&mut self){
           let mut address = self.registers_storage[Registers::R_R0 as usize];

                                loop {
                                    let word = self.memory_read(address);
                                    if word == 0 {
                                        break;
                                    }

                                    // Extract lower 8 bits (char1) and upper 8 bits (char2)
                                    let char1 = (word & 0xFF) as u8 as char;
                                    print!("{}", char1);

                                    let char2 = (word >> 8) as u8;
                                    if char2 != 0 {
                                        print!("{}", char2 as char);
                                    }

                                    address += 1;
                                }

                                io::stdout().flush().unwrap();
    }
    // if else block and input param is so I can test the funnction independently, 
    pub fn trap_getc(&mut self, input: Option<u16>) {
        if let Some(value) = input {
            self.registers_storage[Registers::R_R0 as usize] = value;
        } else {
            self.registers_storage[Registers::R_R0 as usize] = self.get_char();
        }
        self.update_flags(Registers::R_R0 as u16);
    }
    pub fn trap_out(&mut self) {
        let c = self.registers_storage[Registers::R_R0 as usize] as u8 as char;
        print!("{}", c);
        io::stdout().flush().unwrap();
    }
    pub fn trap_in(&mut self) {
        print!("Enter a character: ");
        io::stdout().flush().unwrap(); // Flush to display the prompt

        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();

        let c = buffer[0] as u16;

        // Echo the character back to the console
        print!("{}", c as u8 as char);
        io::stdout().flush().unwrap();

        // Store in R0
        self.registers_storage[Registers::R_R0 as usize] = c;

        // Update condition flags based on the character value
        self.update_flags(Registers::R_R0 as u16);
    }
    pub fn update_flags(&mut self, r: u16) -> u16 {
        println!("r  {:?}", r);
        let mut condition_flag = self.registers_storage[Registers::R_COND as usize];

        let content_at_r = self.registers_storage[r as usize];

        println!("content at r {:}", content_at_r);
        if content_at_r == 0 {
            condition_flag = R_COND::FL_ZRO as u16
        } else if content_at_r >> 15 == 1 {
            condition_flag = R_COND::FL_NEG as u16
        } else {
            condition_flag = R_COND::FL_POS as u16
        }
        println!("condition flag {:}", condition_flag);
        condition_flag
    }

  

    pub fn not(&mut self, instruction: u16) {
        //destination register (DR)
        let dr = (instruction >> 9) & 0x7;
        //first source register (SR1)
        let r1 = (instruction >> 6) & 0x7;

        self.registers_storage[dr as usize] = !self.registers_storage[r1 as usize];
        self.update_flags(dr);
    }

    pub fn jump(&mut self, instruction: u16) {
        let r1 = (instruction >> 6) & 0x7;

        self.registers_storage[Registers::R_PC as usize] = self.registers_storage[r1 as usize];
    }
    pub fn jump_register(&mut self, instruction: u16) {
        let long_flag = (instruction >> 11) & 0x1;
        self.registers_storage[Registers::R_R7 as usize] =
            self.registers_storage[Registers::R_PC as usize];
        if long_flag == 1 {
            let pc_offset = instruction & 0x7FF;
            let pc_offset = sign_extend(pc_offset, 11);
            self.registers_storage[Registers::R_PC as usize] += pc_offset;
        } else {
            let r1 = (instruction >> 6) & 0x7;
            self.registers_storage[Registers::R_PC as usize] = self.registers_storage[r1 as usize];
        }
    }

    pub fn branch(&mut self, instruction: u16) {
        let pc_offset = instruction & 0x1FF;
        let pc_offset = sign_extend(pc_offset, 9);
        self.registers_storage[Registers::R_PC as usize] += pc_offset;
    }

    pub fn ldi(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0x7;

        let pc_offset = instruction & 0x1FF;
        let pc_offset = sign_extend(pc_offset, 9);

        let address =
            self.memory_read(self.registers_storage[Registers::R_PC as usize] + pc_offset);

        self.registers_storage[dr as usize] = self.memory_read(address);

        self.update_flags(dr);
    }

    pub fn add(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r1 = (instruction >> 6) & 0x7;

        if (instruction >> 5) & 0x1 == 1 {
            let imm5 = instruction & 0x1F;
            let imm5 = sign_extend(imm5, 5);
            self.registers_storage[r0 as usize] = self.registers_storage[r1 as usize] + imm5;
        } else {
            let r2 = instruction & 0x7;
            self.registers_storage[r0 as usize] =
                self.registers_storage[r1 as usize] + self.registers_storage[r2 as usize];
        }
        self.update_flags(r0);
    }

    pub fn and(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0x7;
        let r1 = (instruction >> 6) & 0x7;

        if (instruction >> 5) & 0x1 == 1 {
            let imm5 = instruction & 0x1F;
            let imm5 = sign_extend(imm5, 5);
            self.registers_storage[dr as usize] = self.registers_storage[r1 as usize] & imm5;
        } else {
            let r2 = instruction & 0x7;
            self.registers_storage[dr as usize] =
                self.registers_storage[r1 as usize] & self.registers_storage[r2 as usize];
        }
        self.update_flags(dr);
    }
    pub fn load(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0x7;

        let pc_offset = instruction & 0x1FF;
        let pc_offset = sign_extend(pc_offset, 9);
        self.registers_storage[dr as usize] =
            self.memory_read(self.registers_storage[Registers::R_PC as usize] + pc_offset);
        self.update_flags(dr);
    }
    pub fn load_register(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0x7;
        let r1 = (instruction >> 6) & 0x7;

        let offset = instruction & 0x3F;
        let offset = sign_extend(offset, 6);
        self.registers_storage[dr as usize] =
            self.memory_read(self.registers_storage[r1 as usize] + offset);
        self.update_flags(dr);
    }
    pub fn lea(&mut self, instruction: u16) {
        let dr = (instruction >> 9) & 0x7;
        let pc_offset = instruction & 0x1FF;
        let pc_offset = sign_extend(pc_offset, 9);
        self.registers_storage[dr as usize] =
            self.registers_storage[Registers::R_PC as usize] + pc_offset;
        self.update_flags(dr);
    }
    pub fn store(&mut self, instruction: u16) {
        let sr = (instruction >> 9) & 0x7;

        let pc_offset = instruction & 0x1FF;
        let pc_offset = sign_extend(pc_offset, 9);
        self.mem_write(
            self.registers_storage[Registers::R_PC as usize] + pc_offset,
            self.registers_storage[sr as usize],
        );
    }
    pub fn store_indirect(&mut self, instruction: u16) {
        let sr = (instruction >> 9) & 0x7;

        let pc_offset = instruction & 0x1FF;
        let pc_offset = sign_extend(pc_offset, 9);
        let address =
            self.memory_read(self.registers_storage[Registers::R_PC as usize] + pc_offset);
        self.mem_write(address, self.registers_storage[sr as usize]);
    }
    pub fn store_register(&mut self, instruction: u16) {
        let sr = (instruction >> 9) & 0x7;
        let r1 = (instruction >> 6) & 0x7;
        println!("sr {:}", sr);
        println!("r1 {:}", r1);

        let offset = instruction & 0x3F;
        let offset = sign_extend(offset, 6);
        self.mem_write(
            self.registers_storage[r1 as usize] + offset,
            self.registers_storage[sr as usize],
        );
    }

    pub fn mem_write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }

    pub fn memory_read(&mut self, address: u16) -> u16 {
        if address == MR_KBSR {
            if check_key() {
                self.memory[MR_KBSR as usize] = 1 << 15;
                self.memory[MR_KBDR as usize] = self.get_char();
            } else {
                self.memory[MR_KBSR as usize] = 0;
            }
        }
        self.memory[address as usize]
    }

    fn get_char(&self) -> u16 {
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        buffer[0] as u16
    }
}

