# Brainsuck

> Brainfuck with a second cursor.

[![Gfunction Status](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Frtaylor034%2Fproject-tags%2Fmain%2Ftags%2Fstatus%2Fviable.json)](https://github.com/rtaylor034/project-tags)
[![Gfunction Stance](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Frtaylor034%2Fproject-tags%2Fmain%2Ftags%2Fstance%2Fheld.json)](https://github.com/rtaylor034/project-tags)

## Description

Brainsuck is the esotaric programming language [Brainfuck] with the added feature of a stored cursor location, along with 3 additional instructions regarding it.

The source code contained in this repository is a simple Brainsuck compiler. \
*(With the additional recognition of a [debug instruction])*

# Command Usage

```brainsuck <source file>```

#### `<source file>`

Path to file that contains Brainsuck code to interpret.

# Documentation

**This documentation assumes prior knowledge of [Brainfuck].**

*A great place to learn is an [online interpreter].*

## Additional Instructions

Brainsuck introduces the `SC` (stored cursor), which is a stored location in the memory array.

| Instruction | Description |
| ----------- | ----------- |
| `:` | Moves the `SC` to the cursor. |
| `;` | Moves the cursor to the `SC`. |
| `\|` | Swaps the cursor and the `SC`. |

### Debug Instruction:

This particular interpreter also includes recognition of a 'debug' instruction. \
*This is NOT part of the Brainsuck language itself.*

| Instruction | Description |
| ----------- | ----------- |
| `?` | Prints the current state of memory to standard error. |

### Dev Comments:

> `src/lib.rs` is written for easy addition and/or modification of interpreted instructions.
