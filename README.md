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

## TODO

マウスだけで書き込みが出ようにしたい

[VS Code 外部.bat ファイルを起動 |](https://geoserver.sakura.ne.jp/blog/%E6%9C%AA%E5%88%86%E9%A1%9E/vs-code-%E5%A4%96%E9%83%A8-bat%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E3%82%92%E8%B5%B7%E5%8B%95/)

## LICENSE

GPL-3.0
