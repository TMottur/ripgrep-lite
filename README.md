ripgrep-lite
------------
A minimalist command-line search tool written in Rust.
It mimics basic grep functionality while supporting regex matching,
case-insensitive search, and colorized output of matched text.
### Background

This project was originally based on the [Rust Book's](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) "minigrep" tutorial project. I extended it by:

- Replacing basic string matching with full regex support
- Adding ANSI-colored highlighting for matched text
- Improving error handling and robustness
- Writing and expanding unit tests
- Packaging it as a portfolio-ready CLI tool

The goal was to take a guided learning exercise and evolve it into a standalone, functional tool that reflects real-world Rust development practices.

-------------------
Project Features
-------------------
- Regex-based line search (powered by the `regex` crate)
- Case-insensitive mode via the IGNORE_CASE environment variable
- Highlighted matches using the `colored` crate
- Graceful error handling (invalid regex, missing file, etc.)
- Unit tests included

-------------
How to Use
-------------
1. Build the project:
   cargo build --release

2. Run the tool:
   cargo run -- <pattern> <file_path>

   Example:
   cargo run -- "Rust.*productive" ./sample.txt

3. Case-insensitive search:
   IGNORE_CASE=1 cargo run -- "rust" ./sample.txt

-------------------
Running the Tests
-------------------
To run all included unit tests:

   cargo test

----------------
Dependencies
----------------
- regex
- colored

-----------------------
Future Improvements
-----------------------
- Support for CLI flags like --ignore-case
- Optional literal match mode
- Multiple match highlights per line

---------
License
---------
MIT License

(c) 2025 TMottur
