# The Grep utility written in Rust

### How to use
The app take 2 arguments: query search, file pattern.
Also, it has a couple of options set up from environment variables.
See the examples below:
```bash
cargo run -- "fn main" "src/*.rs"
IGNORE_CASE=1 cargo run -- "result" "src/main.rs"
IS_REGEXP=1 cargo run -- "fn.*Result" "src/main*"
IS_REGEXP=1 IGNORE_CASE=1 cargo run -- "fn.*result" "src/main*"
```

Or if you want to run the binary only:
```bash
cargo build
IS_REGEXP=1 IGNORE_CASE=1 ./target/debug/mygrep "fn.*result" "src/main*"
```
