pub mod vm;


fn main(){
    // Initialize the VM
    let mut vm = vm::run::VM::new();

    // // Load a program into memory
    // let program: [u16; 5] = [
    //     0x1234, // Example instruction
    //     0x5678, // Example instruction
    //     0x9ABC, // Example instruction
    //     0xDEF0, // Example instruction
    //     0x0000, // End of program
    // ];
    // vm.load_program(&program);

    // Run the program
    vm.run();
}