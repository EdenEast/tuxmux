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
  cp ./target/release/tm release
  OUT_DIR=release/completions cargo run --release --bin tmgr-completions
  pandoc --standalone --to man doc/tm.md -o release/man/tm.1
