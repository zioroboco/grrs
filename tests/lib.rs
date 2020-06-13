#[test]
fn check_matching() {
    assert!(grrs::matches("lorem ipsum", "lorem"));
}

#[test]
fn check_not_matching() {
    assert!(!grrs::matches("lorem ipsum", "dolor"));
}
