mod common;

#[test]
fn unit_test_add() {
    common::setup();
    assert_eq!(integration_testing::add(3, 2), 5);
}
