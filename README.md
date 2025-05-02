# o3e-rs &mdash; a Tomasulo's algorithm simulator 

<br>

[<img alt="github" src="https://img.shields.io/badge/MELLIVORANDY%2Fo3e--rs-%23f5cc5b?style=for-the-badge&logo=GITHUB&label=GITHUB" height="20">](https://github.com/mellivorandy/o3e-rs)
[<img alt="build" src="https://github.com/mellivorandy/o3e-rs/actions/workflows/rust.yml/badge.svg" height="20">](https://github.com/mellivorandy/o3e-rs/actions)
[<img alt="license" src="https://img.shields.io/github/license/mellivorandy/o3e-rs?style=for-the-badge&logo=GITHUB&color=light%20green" height="20">](https://github.com/mellivorandy/o3e-rs?tab=MIT-1-ov-file)

<br>

A cycle-accurate implementation of Tomasulo’s algorithm. This simulator models out-of-order execution (OoOE), register renaming, and reservation station behavior based on the classic single Common Data Bus (CDB) architecture, originally introduced in 1967 by IBM computer architect Robert Tomasulo.

<br>

## Features

- **In-order issue**, **out-of-order execution**, **out-of-order completion**

- Implements Reservation Stations, Load Buffers, Store Buffers

- Tracks Register Result Status (Qi) for register renaming and data dependency resolution

- Precise per-cycle simulation of execution timing

- Outputs complete cycle-by-cycle state trace to a `.txt` file

- Modular and well-structured implementation

- Supports a MIPS-like floating-point instruction set: `L.D`, `S.D`, `ADD.D`, `SUB.D`, `MUL.D`, `DIV.D`

<br>

## Overview

This project is a cycle-accurate simulator of Tomasulo's algorithm, written in Rust. It models the behavior of a MIPS-like floating-point pipeline that leverages dynamic scheduling and register renaming to resolve data hazards and exploit instruction-level parallelism (ILP).

The simulator focuses on clarity and precision, providing detailed snapshots of internal states at every cycle. It is well-suited for learners, researchers, and developers who wish to understand or experiment with out-of-order execution mechanisms used in modern CPUs.

<br>

## Getting Started <br><br>

### Prerequisites

- [Rust](https://www.rust-lang.org/) (recommended 1.84.1 or higher)
- A terminal or command prompt to run `cargo`
<br><br>
---

### Building and Running

Clone the repository

```bash
git clone https://github.com/mellivorandy/o3e-rs.git
```

```bash
cd o3e-rs
```

<br>

From the project root, run:

```Rust
cargo build --release
```

<br>

To execute the program, use the following command:

```Rust
cargo run -- <txt_file_path>
```

- <txt_file_path>: The path of the txt file to be simulated.

<br>

In this project structure, the txt file for simulation is located in o3e-rs/data, change the path if the txt file is moved or new files are added.

<br>

Note: If the file path remains unchanged, use the path as given. Simply copy and paste the following command into your terminal:

#### Example command

```Rust
cargo run --release -- data/sample/sample_0.txt
```

<br>

---

### Test

A `[cfg(test)]` unit test module is provided under `utils/parser.rs` to verify the functionality of the instruction parser. The test reads a sample `.txt` input file and prints all parsed instructions. To run the tests and see the parser output:

```Rust
cargo test -- --nocapture
```

This will load and parse data/sample/sample_0.txt, printing a formatted list of parsed instructions to the console and saving the same output to output.txt at the same time.

#### Example Output

```txt
000:  LD     rd: F6     rs: -      rt: -      offset: 34     base: R2
001:  LD     rd: F2     rs: -      rt: -      offset: 45     base: R3
002:  MULTD  rd: F0     rs: F2     rt: F4     offset: -      base: -
003:  SUBD   rd: F8     rs: F6     rt: F2     offset: -      base: -
004:  DIVD   rd: F10    rs: F0     rt: F6     offset: -      base: -
005:  ADDD   rd: F6     rs: F8     rt: F2     offset: -      base: -
```

<br>

---

### Project Structure

```bash
src/
├── simulator/   # Core logic and internal data structures
├── utils/       # Instruction Parser
└── main.rs      # Binary entry point
```

---

### Use Cases & Applications

This Tomasulo algorithm simulator can serve as:

- Educational Tool: A clear, cycle-accurate simulator for understanding dynamic scheduling, register renaming, and data hazard resolution. Ideal for computer architecture courses.

- Instruction Pipeline Analysis: Simulates in-order issue, out-of-order execution, and out-of-order completion, allowing step-by-step examination of how instructions flow through functional units and buffers.

- Debugging & Trace Generation: Each cycle produces a complete snapshot of internal states—including reservation stations, and register status, enabling easy trace-based debugging or educational demonstrations.

- MIPS-like Architecture Exploration: Includes support for a simplified floating-point instruction set (L.D, S.D, ADD.D, SUB.D, MUL.D, DIV.D), useful for simulating real-world pipelines on RISC architectures.

---

### Contributing

Contributions are welcome!  

1. Fork the repo and create a branch.  
2. Make changes and ensure everything works.  
3. Follow the coding style.  
4. Open a pull request with details.  

For major changes, please open an issue first.

<br>

---

### License

This project is licensed under <a href="LICENSE">MIT license</a>.
