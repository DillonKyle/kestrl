# Kestrl - Programming Language

So far this is just working through Rober Nystrom's "Crafting Interpreters" book
and using rust to build out the Interpreter.

The scripts will use a .kst file extension (short for Kestrl), but not even that
is set in stone.

Ive got a scanner and parser that are working so far and the interpreter is very basic, but this is still early stages.

The scope of this project is currently "Am I capable of making a basic language".
As I go along learning more about the language, I will better define what niche this language would fit in.

## Using Kestrl

Currently, only basic math functions are used to show that the scanner, parser, and interpreter are functioning.

### Run the working script

```
// after cloning the repo
cargo run -- working.kst
```

### Start up the REPL

```
cargo run
```

Inside the REPL:

```
>>> 8\*5;
Result: Number(40.0)
>>>exit
```

## Test Scripts

### haiku.kst

This is a simple script that will serve as completion of the first milestone,
once the interpreter can utilize it properly

### working.kst

This is a test of some working capabilities of the language as I build them out.
