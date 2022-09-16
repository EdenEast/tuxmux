default_bin := "tm"

run *args:
  cargo run --bin tm -- {{args}}

build:
  cargo build --bin tm

check:
  cargo check --bin tm

test:
  cargo test --bin tm

install:
  cargo install --path ./crates/tmgr

man:
  pandoc --standalone --to man doc/tm.md -o doc/tm.1

release:
  #/usr/bin/env bash
  cargo build --release --bin tm
  mkdir -p release/{completions,man}
  cp -f ./{LICENSE,readme.md} ./release
  cp -f ./target/release/tm ./release
  OUT_DIR=release/completions cargo run --release --bin tmgr-cmpl
  pandoc --standalone --to man doc/tm.md -o release/man/tm.1
