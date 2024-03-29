# Recipes for avoiding bounds checks in Rust

This repository showcases various approaches to avoiding bounds checks in Rust code, without `unsafe` code.

Each code snippet includes a command to view its assembly. Install `cargo-show-asm` to run them, and `hyperfine` to benchmark the snippets:

```
cargo install cargo-show-asm hyperfine
```

An article with all the details:

[**How to avoid bounds checks in Rust (without unsafe!)**](https://shnatsel.medium.com/how-to-avoid-bounds-checks-in-rust-without-unsafe-f65e618b4c1e)
