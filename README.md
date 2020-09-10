# specit
![CI](https://github.com/nwtgck/specit-rust/workflows/CI/badge.svg)

Spec "it" for Rust testing

## Install

```toml
# Cargo.toml

[dev-dependencies]
specit = "0.3.0"
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

The test output is like the following.

```
running 2 tests
test should_be_correct ... ok
test should_be_wrong ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### describe

```rust
use specit::describe;

#[describe("arithmetic operations")]
mod m {
    use specit::it;

    #[it("should add two numbers")]
    pub fn t() {
        assert_eq!(2 + 2, 4);
    }

    #[it("should multiple two numbers")]
    pub fn t() {
        assert_eq!(3 * 3, 9);
    }
}
```

The test output with `describe` is like the following.

```
running 2 tests
test arithmetic_operations::should_add_two_numbers ... ok
test arithmetic_operations::should_multiple_two_numbers ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## #[tokio::test] support

You can test with `#[tokio::test]` for asynchronous functions.
```rust
use specit::it;

#[it("should work with tokio::test")]
#[tokio::test]
async fn t() {
    let f = async { 10 };
    assert_eq!(f.await, 10);
}
```

You can get short your code using the following features in each asynchronous runtime.

### `features = ["tokio"]`

You can use `use specit::tokio_it as it` for testing asynchronous functions without `#[tokio::test]` like the following.

```rust
use specit::tokio_it as it;

#[it("should work with tokio")]
async fn t() {
    let f = async { 10 };
    assert_eq!(f.await, 10);
}
```

### `features = ["async-std"]`

Use `#[it(...)]` instead of `#[async_std::test]` as follows.

```rust
use specit::async_std_it as it;

#[it("should be correct")]
async fn t() {
    let f = async { 10 };
    assert_eq!(f.await, 10);
}
```

### `features = ["lib-wasm-bindgen"]`

Use `#[it(...)]` instead of `#[wasm_bindgen_test::wasm_bindgen_test]` as follows.

```rust
use specit::wasm_bindgen_test_it as it;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen_futures::JsFuture;

#[it("should be correct")]
async fn t() {
    let promise = js_sys::Promise::resolve(&JsValue::from(42));
    let x = JsFuture::from(promise).await.unwrap();
    assert_eq!(x, 42);
}
```

## Internal

Internally, the functions above are `should_be_correct()` and `should_be_wrong()`. You can use any string. Non-alphanum characters are encoded into `'_'`.
