# Crates.io

<https://crates.io/crates/cow-parser >

# Cow Parser

A parser for the esoteric Cow programming language, built in Rust using the `pest` library.

## Technical Description

This parser processes text-based `.cow` files. It identifies and extracts the 12 valid Cow commands (`MoO`, `moO`, etc.) while ignoring all other text, which is treated as comments. The result of a successful parse is an Abstract Syntax Tree (AST), represented by a sequence of `pest` pairs, where each pair corresponds to a recognized command. This AST can then be used as input for an interpreter or a compiler.

### Grammar that contains **14 grammar rules** 

The parsing logic is defined by the following rules:

```rust
increment = { "MoO" }
decrement = { "MOo" }
move_left = { "mOo" }
move_right = { "moO" }

increment_block = { increment+ }
decrement_block = { decrement+ }
cancelling_moves = { (move_left ~ move_right) | (move_right ~ move_left) }

other_command = {
    "moo" | "mOO" | "Moo" | "MOO" | "OOO" | "MMM" | "OOM" | "oom"
}

command = {
    increment | decrement | move_left | move_right | other_command
}

program_token = _{ cancelling_moves | increment_block | decrement_block | command | ANY }

program = _{ SOI ~ program_token* ~ EOI }