use adder;

mod common;

#[test]
fn it_adds_two_integration_test() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

#[test]
fn yet_another_it_adds_two_integration_test() {
    assert_eq!(4, adder::add_two(2));
}
