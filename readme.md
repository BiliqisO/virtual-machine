# Virtual Machine Emulator

This project is a **Virtual Machine Emulator** written in Rust. It emulates a simple virtual machine capable of executing instructions, handling memory, and performing basic I/O operations. The emulator supports a variety of opcodes and traps, making it a versatile tool for learning and experimentation.

## Features

- **Instruction Set**: Implements a range of opcodes, including arithmetic, logical, memory, and control flow instructions.
- **Memory Management**: Supports a 16-bit addressable memory space.
- **Registers**: Includes 10 general-purpose registers and condition flags.
- **Trap Routines**: Provides built-in support for input/output operations like `GETC`, `OUT`, `PUTS`, `PUTSP`, and `HALT`.
- **Binary Image Loading**: Reads binary image files into memory for execution.
- **Input Buffering**: Handles input buffering for smooth I/O operations.

## Project Structure

```

├── src
│ ├── vm
│ │ ├── run.rs # Core implementation of the virtual machine
│ │ ├── input_buffering.rs # Input buffering utilities
│ └── main.rs # Entry point for the emulator
├── tests
│ ├── opcode_tests.rs # Unit tests for opcodes and VM functionality
│ ├── test_image.bin # Binary image file for testing
├── Cargo.toml # Rust project configuration
└── README.md # Project documentation

```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/virtual-vm.git
   cd virtual-vm
   ```



2. Build the project:

   ```bash
   cargo build
   ```

3. Run the emulator:
   ```bash
   cargo run -- <binary-image-file>
   ```

### Running Tests

To run the unit tests, use the following command:

```bash
cargo test
```

### Example Usage

1. Create a binary image file (e.g., `test_image.bin`) with the following content:

   - Starting address: `0x3000`
   - Memory content: `0x1234`

2. Run the emulator with the binary image:

   ```bash
   cargo run -- tests/test_image.bin
   ```

3. Observe the output and behavior of the virtual machine.

## Instruction Set

The virtual machine supports the following opcodes:

| Opcode    | Description            |
| --------- | ---------------------- |
| `OP_ADD`  | Add                    |
| `OP_AND`  | Bitwise AND            |
| `OP_NOT`  | Bitwise NOT            |
| `OP_BR`   | Branch                 |
| `OP_LD`   | Load                   |
| `OP_ST`   | Store                  |
| `OP_JSR`  | Jump Register          |
| `OP_LDR`  | Load Register          |
| `OP_STR`  | Store Register         |
| `OP_LDI`  | Load Indirect          |
| `OP_STI`  | Store Indirect         |
| `OP_JMP`  | Jump                   |
| `OP_LEA`  | Load Effective Address |
| `OP_TRAP` | Execute Trap Routine   |

## Trap Routines

| Trap Code    | Description                       |
| ------------ | --------------------------------- |
| `TRAP_GETC`  | Get a character from the keyboard |
| `TRAP_OUT`   | Output a character                |
| `TRAP_PUTS`  | Output a string                   |
| `TRAP_PUTSP` | Output a packed string            |
| `TRAP_HALT`  | Halt the program                  |

## Memory Layout

- **Memory Size**: 16-bit addressable space (`0x0000` to `0xFFFF`)
- **Special Registers**:
  - `MR_KBSR`: Keyboard Status Register (`0xFE00`)
  - `MR_KBDR`: Keyboard Data Register (`0xFE02`)

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Commit your changes and push them to your fork.
4. Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the LC-3 Virtual Machine.
- Special thanks to the Rust community for their excellent documentation and support.


````
