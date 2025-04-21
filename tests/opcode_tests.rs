

#[cfg(test)]


mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Write, BufWriter};
    use virtual_vm::vm::run::{VM, Registers};
    use virtual_vm::vm::input_buffering::{setup, disable_input_buffering, restore_input_buffering};

    #[test]
    fn test_vm_run_fn() {
        //this test is progressive
        let mut vm = VM::new();
        vm.registers_storage[1] = 1;
        vm.registers_storage[2] = 5;
        // vm.run(0x1042);
        println!(" ft {:?}", vm.registers_storage[Registers::R_R0 as usize]);
    }
    #[test]
    fn test_add_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[0] = 4;
        vm.registers_storage[1] = 1;
        vm.registers_storage[2] = 5;

        vm.add(0x1042);
        assert_eq!(vm.registers_storage[Registers::R_R0 as usize], 6);
    }
    #[test]
    fn test_ldi_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_PC as usize] = 0x3000;
        vm.mem_write(0x3005, 0x2ffd);
        vm.mem_write(0x2ffd, 42);
        vm.ldi(0xA405);
        assert_eq!(vm.registers_storage[Registers::R_R2 as usize], 42);
    }
    #[test]
    fn test_and_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[1] = 4;
        vm.and(0x5464);
        assert_eq!(vm.registers_storage[Registers::R_R2 as usize], 4);
    }
    #[test]
    fn test_not_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[2] = 4;
        vm.not(0x96BF);
        assert_eq!(vm.registers_storage[Registers::R_R3 as usize], !4);
    }
    #[test]
    fn test_br_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_PC as usize] = 0x3000;
        vm.branch(0x1404);
        assert_eq!(vm.registers_storage[Registers::R_PC as usize], 0x3004);
    }
    #[test]
    fn test_jump_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_PC as usize] = 0x3000;
        vm.registers_storage[Registers::R_R6 as usize] = 0x3001;
        vm.jump(0xC380);
        assert_eq!(vm.registers_storage[Registers::R_PC as usize], 0x3001);
    }
    #[test]
    fn test_jump_register_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_PC as usize] = 0x3000;
        vm.registers_storage[Registers::R_R3 as usize] = 0x2305;
        vm.jump_register(0x04C0);
        assert_eq!(vm.registers_storage[Registers::R_R7 as usize], 0x3000);
        assert_eq!(vm.registers_storage[Registers::R_PC as usize], 0x2305);
    }
    #[test]
    fn test_load_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_PC as usize] = 0x3000;
        vm.mem_write(0x3005, 42);
        vm.load(0x2205);
        assert_eq!(vm.registers_storage[Registers::R_R1 as usize], 42);
    }
    #[test]
    fn test_load_register_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_R3 as usize] = 0x3000;
        vm.mem_write(0x3004, 42);
        vm.load_register(0x64C4);
        assert_eq!(vm.registers_storage[Registers::R_R2 as usize], 42);
    }
    #[test]
    fn test_lea_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_PC as usize] = 0x3000;
        vm.lea(0xE203);
        assert_eq!(vm.registers_storage[Registers::R_R1 as usize], 0x3003);
    }
    #[test]
    fn test_store_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_PC as usize] = 0x3000;
        vm.registers_storage[Registers::R_R2 as usize] = 42;
        vm.store(0x3406);
        assert_eq!(vm.memory[0x3006 as usize], 42);
    }
    #[test]
    fn test_store_indirect_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_PC as usize] = 0x3000;
        vm.registers_storage[Registers::R_R2 as usize] = 42;
        vm.mem_write(0x3006, 0x3007);
        vm.store_indirect(0x3406);
        assert_eq!(vm.memory[0x3007 as usize], 42);
    }
    #[test]
    fn test_store_register_opcode() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_R2 as usize] = 42;
        vm.registers_storage[Registers::R_R3 as usize] = 0x3000;
        vm.store_register(0x64C4);
        assert_eq!(vm.memory[0x3004 as usize], 42);
    }
    #[test]
    fn test_trap_puts() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_R0 as usize] = 0x3000;
        vm.mem_write(0x3000, 0x0048);
        vm.mem_write(0x3001, 0x0069);
        vm.mem_write(0x3002, 0x0021);
        vm.mem_write(0x3003, 0x0021);
        vm.mem_write(0x3004, 0x0021);
        vm.mem_write(0x3005, 0x0000); // Null terminator
        vm.trap_puts();

    }
    #[test]
    fn test_trap_putsp() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_R0 as usize] = 0x3000;
        vm.mem_write(0x3000, 0x6548);
        vm.mem_write(0x3001, 0x6C6C);
        vm.mem_write(0x3002, 0x006F);
        vm.mem_write(0x3003, 0x0021);// Null terminator
        vm.trap_putsp();

    }
    #[test]
    fn test_trap_getc() {
        let mut vm = VM::new();    
        vm.trap_getc(Some('A' as u16));
        assert_eq!(vm.registers_storage[Registers::R_R0 as usize], 'A' as u16);
    }
    #[test]
    fn test_trap_out() {
        let mut vm = VM::new();
        vm.registers_storage[Registers::R_R0 as usize] = 'A' as u16;
        vm.trap_out();
        // Check the output manually
    }   
    // #[test]
    // fn test_trap_in() {
    //     let mut vm = VM::new();
    //     vm.trap_in();
    //     // Check the output manually
    // }  
    #[test]
    fn test_read_image(){
        let mut vm = VM::new();
        let filename = "tests/test_image.bin";
        vm.read_image(filename).unwrap();

        // Check if the memory is loaded correctly
        assert_eq!(vm.memory[0x3000], 0x1234); 
    } 
   #[test]
   fn test_read_image_file() {
        let mut vm = VM::new();
        let filename = "tests/test_image.bin";

    // Create and write to the file
    {
        let mut file = File::create(filename).unwrap();

        // Write the starting address (0x3000) as two bytes
        file.write_all(&0x3000u16.to_be_bytes()).unwrap();

        // Write the memory content (0x1234) as two bytes
        file.write_all(&0x1234u16.to_be_bytes()).unwrap();
    } // File is closed here

    // Reopen the file for reading
    let file = File::open(filename).unwrap();

    // Pass the file to the VM
    vm.read_image_file(file).unwrap();

    // Check if the memory is loaded correctly
    assert_eq!(vm.memory[0x3000], 0x1234); 
}

}