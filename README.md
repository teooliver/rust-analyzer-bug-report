### Type Inference Bug report

rust-analyzer version: 02904e99a 2022-02-14 stable
rustc version: rustc 1.58.1 (db9d1b20b 2022-01-20)

Rust-analyzier marks `data` variable as `&{unknown}` type, but it actually kind of knows what type it really is as showns in the images.
