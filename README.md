# 自作キーボード用のファームウェア

## usage

### 環境準備

```power shell
rustup target install thumbv6m-none-eabi
cargo install flip-link probe-run
cargo install elf2uf2-rs
cargo install --git https://github.com/rspeir/elf2uf2-rs --branch branch2
```

### ビルド

```power shell
cargo build
```

### 書き込み

```power shell
cargo run
```
