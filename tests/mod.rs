use specit::{describe, it};

// #[it("should be correct")]
// fn t() {
//     assert_eq!(2 + 2, 4);
// }
//
// #[it("should be wrong")]
// #[should_panic]
// fn t() {
//     assert_eq!(1 + 1, 3);
// }

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

#[cfg(feature = "tokio")]
mod pure_it_with_tokio {
    use specit::it;

    #[it("should work with tokio::test")]
    #[tokio::test]
    async fn t() {
        let f = async { 10 };
        assert_eq!(f.await, 10);
    }
}

#[cfg(feature = "tokio")]
mod tokio {
    use specit::tokio_it as it;

    #[it("should work with non-async function")]
    fn t() {
        assert_eq!(2 + 2, 4);
    }

    #[it("should work with tokio")]
    async fn t() {
        let f = async { 10 };
        assert_eq!(f.await, 10);
    }

    #[it("should panic with tokio")]
    #[should_panic]
    async fn t() {
        let f = async { 10 };
        assert_eq!(f.await, 12);
    }
}

#[cfg(feature = "async-std")]
mod async_std {
    use specit::async_std_it as it;

    #[it("should work with non-async function")]
    fn t() {
        assert_eq!(2 + 2, 4);
    }

    #[it("should be correct")]
    async fn t() {
        let f = async { 10 };
        assert_eq!(f.await, 10);
    }

    #[it("should panic with async_std")]
    #[should_panic]
    async fn t() {
        let f = async { 10 };
        assert_eq!(f.await, 12);
    }
}

#[cfg(feature = "lib-wasm-bindgen")]
mod wasm_bindgen_test {
    use specit::wasm_bindgen_test_it as it;
    use wasm_bindgen::prelude::JsValue;
    use wasm_bindgen_futures::JsFuture;

    #[it("should work with non-async function")]
    fn t() {
        assert_eq!(2 + 2, 4);
    }

    #[it("should be correct")]
    async fn t() {
        let promise = js_sys::Promise::resolve(&JsValue::from(42));
        let x = JsFuture::from(promise).await.unwrap();
        assert_eq!(x, 42);
    }

    // TODO: "wasm_bindgen_test" is not supported #[should_panic] (ref: https://github.com/rustwasm/wasm-bindgen/issues/2286)
    // #[it("should be wrong")]
    // #[should_panic]
    // async fn t() {
    //     let promise = js_sys::Promise::resolve(&JsValue::from(42));
    //     let x = JsFuture::from(promise).await.unwrap();
    //     assert_eq!(x, 43);
    // }
}
