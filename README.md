
Dev build:

```
RUSTFLAGS="$RUSTFLAGS -A dead_code -A unused-mut -A unused-imports -A unused-variables" cargo run
```

Optimized build:

```
RUSTFLAGS="$RUSTFLAGS -A dead_code -A unused-mut -A unused-imports -A unused-variables" cargo build --release && ./target/release/rts2
```