use ch_11_03_test_org::add_two;

mod common;

#[test]
fn add_two_test() {
    common::setup();
    assert_eq!(add_two(2), 4);
}
