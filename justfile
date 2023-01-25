default_bin := "tm"

man:
  pandoc --standalone --to man doc/tm.md -o doc/tm.1

release:
  #/usr/bin/env bash
  cargo build --release --bin tm
  mkdir -p release/{completions,man}
  cp -f ./{LICENSE,readme.md} ./release
  cp -f ./target/release/tm ./release
  ./release/tm completions bash > ./release/completions/tm.bash
  ./release/tm completions zsh > ./release/completions/_tm
  ./release/tm completions fish > ./release/completions/tm.fish
  ./release/tm completions powershell > ./release/completions/_tm.ps1
  ./release/tm completions elvish > ./release/completions/tm.elv
  pandoc --standalone --to man doc/tm.md -o release/man/tm.1
