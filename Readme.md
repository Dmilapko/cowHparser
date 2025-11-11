# Crates.io

<https://crates.io/crates/cow-parser >

# Cow Parser

A parser for the esoteric Cow programming language, built in Rust using the `pest` library.

## Technical Description

This parser processes text-based `.cow` files. It identifies and extracts the 12 valid Cow commands (`MoO`, `moO`, etc.) while ignoring all other text, which is treated as comments. The result of a successful parse is an Abstract Syntax Tree (AST), represented by a sequence of `pest` pairs, where each pair corresponds to a recognized command. This AST can then be used as input for an interpreter or a compiler.

### Grammar that contains **14 grammar rules** 

The parsing logic is defined by the following rules:

```rust
command = {
    moo | mOo | moO | mOO | Moo | MOo | MoO | MOO | OOO | MMM | OOM | oom
}

moo = { "moo" }
mOo = { "mOo" }
moO = { "moO" }
mOO = { "mOO" }
Moo = { "Moo" }
MOo = { "MOo" }
MoO = { "MoO" }
MOO = { "MOO" }
OOO = { "OOO" }
MMM = { "MMM" }
OOM = { "OOM" }
oom = { "oom" }

program = _{ SOI ~ (command | ANY)* ~ EOI }
