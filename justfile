default_bin := "tm"

run *args:
  cargo run --bin tm -- {{args}}

build:
  cargo build --bin tm

check:
  cargo check --bin tm

test:
  cargo test --bin tm

release:
  #/usr/bin/env bash
  cargo build --release --bin tm
  mkdir -p release/{completions,man}
  cp {LICENSE,README.md} release/
  OUT_DIR=release/completions cargo run --release --bin tmgr-completions
  OUT_DIR=release/man cargo run --release --bin tmgr-mangen
  cp ./target/release/tm release
