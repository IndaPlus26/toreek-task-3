# toreek-task-3
A feature complete chess library 

## Language installation instructions
Follow the official install instruction:
https://doc.rust-lang.org/book/ch01-01-installation.html

## Run instructions

How to compile the library:
```bash
cargo build --release
```

## How to test

Run the following command to run all available tests, including a console implementation of the chess engine. 
```bash
cargo test -- --nocapture --test-threads=1
``` 

## Known bugs & missing features
- Function `get_possible_moves()` crashes when given a non-chess position string (e.g. "B9", "ABC"). This is intended
- Castling and en passant remain unimplemented

## Docs
To access documentation, open file at doc/toreek-task-3/index.html

