# specit
![CI](https://github.com/nwtgck/specit-rust/workflows/CI/badge.svg)

it-should style for Rust

## Install

```toml
# Cargo.toml

[dev-dependencies]
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

## `#[tokio::test]` support


You can use with `#[tokio::test]` for asynchronous functions.
```rust
use specit::it;

#[it("should work with tokio::test")]
#[tokio::test]
async fn t() {
    let f = async { 10 };
    assert_eq!(f.await, 10);
}
```

## Internal

Internally, the functions above are `should_be_correct()` and `should_be_wrong()`. You can use any string. Non-alphanum characters are encoded into `'_'`.
