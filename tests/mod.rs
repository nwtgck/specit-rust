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

#[it("should work with tokio::test")]
#[tokio::test]
async fn t() {
    let f = async { 10 };
    assert_eq!(f.await, 10);
}
