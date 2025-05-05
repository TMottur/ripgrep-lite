ripgrep-lite
------------

This tool was built as part of my early Rust learning journey. It demonstrates how to build a CLI application using idiomatic Rust, including:

- Regex-based pattern matching (with regex)
- ANSI-colored output (via colored)
- Environment variable support
- Error handling and unit testing

While minimalist by design, this project showcases real-world Rust fundamentals and serves as a base for more advanced CLI work.

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
