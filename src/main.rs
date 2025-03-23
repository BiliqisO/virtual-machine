use std::io::Read;
use std::{io, u16, vec};
use std::env::args;
use std::convert::TryFrom;
fn main() {
    println!("Hello, world!");
}
//16 bits each 
#[repr(u16)]  
enum Registers {
    R_R0 = 0,
    R_R1,
    R_R2,
    R_R3,
    R_R4,
    R_R5,
    R_R6,
    R_R7,
    R_PC(u16),
    R_COND(R_COND),
    R_COUNT(u16)
}
const MR_KBSR: u16 = 0xFE00; // Keyboard Status Register
const MR_KBDR: u16 = 0xFE02; // Keyboard Data Register
//R_COND condition flags
enum R_COND {
    FL_POS = 1 << 0,
    FL_ZRO = 1 << 1,
    FL_NEG = 1 << 2,
    
}
//16 bits each
#[repr(u16)]  
enum Opcodes{
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
    OP_TRAP    /* execute trap */
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
struct VM{
    memory: Vec<u16>,
    registers_storage:Vec<u16>

}

impl VM {
    fn new()->Self{
        Self{
            memory:vec![u16::MAX],
            registers_storage:vec![Registers::R_COUNT as u16]

        }
    }
    fn mem_write(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }

    fn memory_read(&mut self, address: u16) -> u16 {
        if address == MR_KBSR {
            if self.check_key() {
                self.memory[MR_KBSR as usize] = 1 << 15;
                self.memory[MR_KBDR as usize] = self.get_char();
            } else {
                self.memory[MR_KBSR as usize] = 0;
            }
        }
        self.memory[address as usize]
    }
    fn check_key(&self) -> bool {
        // Simulate a key press check (would normally check OS I/O buffers)
        // For simplicity, assume there's always a key available
        true
    }

    fn get_char(&self) -> u16 {
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        buffer[0] as u16
    }

    fn setup(self){
        let vm = VM::new(); //initialize the VM
        //assigning the fl_zro to register_storage at index r_cond  
        self.registers_storage[Registers::R_COND as usize] = R_COND::FL_ZRO as u16;
        // let mem_read = self.memory_read(Registers::R_PC as u16);
        let pc_start = Registers::R_PC(0x3000);
       
        let args: Vec<String> = args().collect();
        let running =1;
        while (running > 0){
           self.registers_storage[Registers::R_PC as usize]+=1; 
            let instr =self.memory_read(self.registers_storage[Registers::R_PC as usize]);

            let opcode = instr >> 12;
            match Opcodes::try_from(opcode) {
                Ok(Opcodes::OP_ADD) => {
                    self.add(instr);    

                            }
                Ok(Opcodes::OP_AND) => {
                            }
                Ok(Opcodes::OP_NOT) => {
                            }
                Ok(Opcodes::OP_BR) => {
                            }
                Ok(Opcodes::OP_LD) => todo!(),
                Ok(Opcodes::OP_ST) => todo!(),
                Ok(Opcodes::OP_JSR) => todo!(),
                Ok(Opcodes::OP_LDR) => todo!(),
                Ok(Opcodes::OP_STR) => todo!(),
                Ok(Opcodes::OP_RTI) => todo!(),
                Ok(Opcodes::OP_LDI) => todo!(),
                Ok(Opcodes::OP_STI) => todo!(),
                Ok(Opcodes::OP_JMP) => todo!(),
                Ok(Opcodes::OP_RES) => todo!(),
                Ok(Opcodes::OP_LEA) => todo!(),
                Ok(Opcodes::OP_TRAP) => todo!(),
                Err(_) => {
                    println!("Invalid opcode");
                    break;
                }
                            }

                            if args.len()<2{
                                println!("Usage: ./lc3 [image-file1] ...");
                                println!("Exiting...");
                                break;
                            }
                            for i in 1..args.len(){
                                if !load_image(args[i].as_str()){
                                    println!("Failed to load image: {}", args[i]);
                                    break;
                                }
                            }
            
        }
    }
    
    fn update_flags(r: u16) -> Registers{
       let mut  condition_flag = Registers::R_COND(R_COND::FL_ZRO);
       if r  == 0 {
        condition_flag = Registers::R_COND(R_COND::FL_ZRO)
       }else if r >> 15 ==1 {
        condition_flag = Registers::R_COND(R_COND::FL_NEG)
    
       }else {
        condition_flag =  Registers::R_COND(R_COND::FL_POS)
       }
       condition_flag

  }
    fn add(mut self, instruction:u16){
        //destination register (DR)
        let r0 = (instruction >> 9 ) & 0x7;
        //first source register (SR1)
        let  r1 = (instruction >> 6) &  0x7;
        if (instruction >> 5) & 0x1 == 1{
            let mut imm5:i16 = (instruction & 0x1F).try_into().unwrap();// extract 5-bit immediate value
            if imm5 & 0x10  == 1{
                imm5 |= 0xFFE0;

            }
            self.registers_storage[r0 as usize] = self.registers_storage[r1 as usize] + imm5 as u16;
        }else {
            let r2 = instruction & 0x7;
            self.registers_storage[r0 as usize] = self.registers_storage[r1 as usize] + self.registers_storage[r2 as usize];
        }
        VM::update_flags(self.registers_storage[r0 as usize]);  
    }
}