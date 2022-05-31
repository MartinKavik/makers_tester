# makers_tester

## Test with installed `cargo-make`:

```
makers tester 5
```

where `5` is the duration of sleeping in seconds to simulate graceful shutdown or zombie processes.

## Test with cloned `cargo-make`:

```
cargo run --manifest-path "../cargo-make/Cargo.toml" --bin "makers" -- tester 5
```
