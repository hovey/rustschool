// use adder;
use adder-chapter-11;
// Move to just plain "adder" instead of "adder-chapter-11",
// since the dash ("-")" is not allowed in the library name.

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
