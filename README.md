## Table of Contents

- Introduction
- Features
- Installation
- Usage
- How the Compiler Works
- Makefile Automation
- Testing
- Example Runs

---

## Introduction

The **Adder Compiler** is a simple compiler written in Rust that compiles a subset of S-Expressions into x86-64 assembly. It supports basic arithmetic operations such as:

```
- Add1 (increment)
- Sub1 (decrement)
- Negate (negation)
```

This project focuses on understanding compiler design by covering:

- Lexing and parsing S-Expressions
- Abstract Syntax Tree (AST) representation
- Code generation to x86-64 assembly
- Automated testing and execution

---

## Features

- Parses S-Expression input and converts it to an AST (Abstract Syntax Tree)
- Generates x86-64 assembly code for arithmetic operations
- Compiles and links assembly code into an executable binary
- Includes a Makefile for automated build and testing
- Provides a testing script that logs results to an output file

---

## Installation

### Required Dependencies

Ensure you have:

- **Rust** 🦀 ([Install Rust](https://rustup.rs/))
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- NASM (Assembler for x86-64)

  ```sh
  # Ubuntu/Debian
  sudo apt install nasm

  # macOS
  brew install nasm
  ```

- GCC or Clang (For linking the final executable)

  ```sh
  # Ubuntu/Debian
  sudo apt install gcc

  # macOS (Clang is part of LLVM)
  brew install llvm
  ```

### Steps to Install

1. Install NASM for assembling machine code.
2. Install GCC or Clang to link the compiled code.
3. Clone the repository from the version control system.
   ```sh
   git@github.com:samiransarii/assembly-adder-interpreter-and-compiler-rust.git
   ```
4. Navigate to the project directory.
5. Build the compiler using the provided build system.
   ```sh
   cargo build
   ```

---

## Usage

### Compiling a Source File

To compile a program, run the build system with the appropriate target. This process includes:

1. Converting the source file into an intermediate representation.
2. Assembling the intermediate code into an object file.
3. Linking the object file to generate an executable.

To compile a program, use:

```sh
    make test/add.run  # Example: Compiling test/add.snek
```

### Running the Compiled Program

Once compiled, execute the generated binary to see the output of the computation.

```sh
    ./test/add.run
```

---

## How the Compiler Works

### Step 1: Parsing S-Expressions

The compiler reads an input file containing S-Expressions and converts it into an Abstract Syntax Tree (AST). The AST is a hierarchical representation of the program.

#### Example S-Expression

```sh
(add1 (sub1 5))
```

#### Corresponding AST Representation

```sh
Add1(Sub1(Num(5)))
```

---

### Step 2: Generating Assembly Code

The AST is traversed, and corresponding x86-64 assembly instructions are generated.

#### Conceptual Transformation

```sh
Add1(Sub1(Num(5)))
```

Becomes:

```sh
- Load 5 into a register
- Subtract 1
- Add 1
- Return the result
```

---

### Step 3: Compiling and Executing

1. The generated assembly is converted into machine code.
2. The machine code is linked with the necessary runtime components.
3. The final executable is produced and can be run on the system.

---

## Makefile Automation

The Makefile simplifies the build process with predefined commands.

### Available Commands

```sh
make clean       # Remove all generated files (assembly, executables, object files)
make all         # Clean, compile, and build all test cases
make test-all    # Compile all .snek test files into executable binaries
```

---

## Testing

### Running Automated Tests

The test script compiles and executes test cases, comparing actual output with expected results. The results are logged for review.

### Viewing Test Results

The results of all tests can be reviewed in the generated transcript file.

---

## Example Runs

### Example 1: Arithmetic Computation

#### Input

```
(add1 (sub1 73))
```

#### Internal Representation

Abstract Syntax Tree:

```
- Add1
- Sub1
  - Num(73)
```

#### Execution Process

1. Load 73 into memory.
2. Perform subtraction by reducing it by 1.
3. Perform addition by increasing it by 1.
4. Return the final result.

#### Output

```
73
```

---
