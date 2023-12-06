# Brainsuck

> Brainfuck with a second cursor.

[![Gfunction Status](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Frtaylor034%2Fproject-tags%2Fmain%2Ftags%2Fstatus%2Fviable.json)](https://github.com/rtaylor034/project-tags)
[![Gfunction Stance](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Frtaylor034%2Fproject-tags%2Fmain%2Ftags%2Fstance%2Fheld.json)](https://github.com/rtaylor034/project-tags)

## Description

Brainsuck is a programming language that is defined by [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) with 3 additional instructions regarding a second 'stored cursor'.

The source code contained in this repository is a simple Brainsuck interpreter. \
*(This interpreter also interprets an additional [debug instruction](https://github.com/rtaylor034/brainsuck#debug-instruction))*

# Command Usage

```brainsuck <source file>```

#### `<source file>`

Path to file that contains Brainsuck code to interpret.

# Documentation

**This documentation only covers the features that Brainsuck adds to [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck).**

*A great place to learn about Brainfuck is an [online interpreter](https://minond.xyz/brainfuck).*

## The Stored Cursor

Brainsuck introduces the `SC` (stored cursor), which is a stored location in the memory array.

The `SC` starts at index 0 in the memory array, and can only be controlled by via the `:` and `|` instructions.

### Instructions:

| Instruction | Description |
| ----------- | ----------- |
| `:` | Jumps the `SC` to the cursor. |
| `;` | Jumps the cursor to the `SC`. |
| `\|` | Swaps the cursor and the `SC`. |

## Debug Instruction

This particular interpreter also interprets the `?` debug instruction.

| Instruction | Description |
| ----------- | ----------- |
| `?` | Prints the current state of memory to standard error; does not affect the state of the program. |
