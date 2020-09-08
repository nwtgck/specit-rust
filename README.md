# specit
it-should style for Rust

## Install

```toml
# Cargo.toml

[dependencies]
specit = { git = "https://github.com/nwtgck/specit-rust.git" }
```

## Usage

```rust
use specit::it;

#[it("should be correct")]
fn t() {
    assert_eq!(2 + 2, 4);
}

#[it("should be wrong")]
#[should_panic]
fn t() {
    assert_eq!(1 + 1, 3);
}
```

## Internal

Internally, the functions above are `should_be_correct()` and `should_be_wrong()`. You can use any string. Non-alphanum characters are encoded into `'_'`.

