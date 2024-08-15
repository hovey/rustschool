// use adder;

// mod common;

/// This module contains integration tests for the `add_two` functions.
mod integration_tests {
    use adder::add_two;

    // mod super::common::setup;

    /// This test checks if `add_two` returns the expected result
    /// when given a specific input.
    ///
    /// # Example
    ///
    /// ```
    /// let result = add_two(2);
    /// assert_eq!(result, 4);
    /// ```
    #[test]
    pub fn it_adds_two_integration_test() {
        // common::setup();
        // assert_eq!(4, adder::add_two(2));
        assert_eq!(4, add_two(2));
    }

    /// This is another test with `add_two`.
    #[test]
    fn yet_another_it_adds_two_integration_test() {
        // assert_eq!(4, adder::add_two(2));
        assert_eq!(4, add_two(2));
    }
}