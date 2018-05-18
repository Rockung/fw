## Unit Testing

### tests

functions that verify that the non-test code is functioning in the expected manner

### the body of test function

* perform some setup
* run the test code
* assert whether the results are what we expect

### test attribute

* #[cfg(test)]: modules
* #[test]: functions

### assertion macros

* assert!(expression): panics if evaluating to false
* assert_eq!(left, right): equality 
* assert_ne!(left, right): inequality

### testing panics

* #[should_panic(expected = ...)]

### ignoring tests

* #[ignore]: exclude the tests
* cargo test -- --ignored: run the ignored tests
