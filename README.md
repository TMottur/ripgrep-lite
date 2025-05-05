ripgrep-lite
------------

A minimalist command-line search tool written in Rust.
It mimics basic grep functionality while supporting regex matching,
case-insensitive search, and colorized output of matched text.

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
