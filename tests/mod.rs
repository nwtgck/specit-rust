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
