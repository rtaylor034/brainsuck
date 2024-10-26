# Brainsuck

> Brainfuck with a second cursor.

# Overview

Brainsuck is [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) with 3 additional instructions regarding a 'stored cursor'.

The source code contained in this repository is a simple Brainsuck interpreter written in Rust. \
This particular interpreter also interprets an additional debug instruction that does not affect program function.

# Command Usage

```brainsuck <source file>```

`<source file>`: \
Path to file that contains Brainsuck code to interpret.

# Documentation

For Brainfuck instructions, see it's [wiki](https://en.wikipedia.org/wiki/Brainfuck) or an [online interpreter](https://minond.xyz/brainfuck).

Brainsuck programs have the extension `.bs`. \
Example programs are provided in the [examples](examples/) directory.

## The Stored Cursor

Brainsuck introduces the `SC`, which is a second cursor location that starts at index 0, and can be interacted with via the following instructions.

| Instruction | Description |
| ----------- | ----------- |
| `:` | Moves the `SC` to the cursor. |
| `;` | Moves the cursor to the `SC`. |
| `\|` | Swaps the cursor and the `SC`. |

### Debug Instruction

This particular interpreter also interprets the `?` debug instruction, which prints the current state of memory to standard error.
