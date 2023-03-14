# Laze Compiler / Laze コンパイラ

## 実行用のコマンド / Command to execute

```
cargo build --release
target/release/laze <ファイルパス/FILEPATH> --compile --parser=<パーサーファイルパス/PARSER FILE PATH>
```

例:

```
cargo build --release
target/release/laze ./laze_tests/if_statement/if_else.laze --compile --parser=./parser_files/ja.peg
```
