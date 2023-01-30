How to Write Tests

How to verify that non-test code is functioning in the expected manner.
1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.


The Anatonmy of a Test Function
A test is a function that annotated with the `test` attribute.

Attributes are metadata about the pieces of Rust code e.g. `derive` attribute used with structs.

Add #[test] on the line before `fn` to change a function into a test function.

`cargo test` command, tells Rust to create a test runner binary that runs the annotated functions and reports on whether each test function passes or fails.

Library projects created using Cargo automatically generates a test module with a test function.
The module gives a template for writing tests where additional test functions and test modules may be added.

Overview of Test Options
`[#ignored]`: ignores a particular test
filtering: regex runs tests matching a pattern passed as an argument to cargo test "<>"
measured: benchmark tests
Doc-tests: documentation tests that keeps docs and code in sync.


Example Test Output
$cargo test

```
# Results of individual tests
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

# Detailed reason for each test failure
failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:17:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

# A list of the names of the failing tests
failures:
    tests::another

# Test summary
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```


Checking Results with the `assert!` Macro
`assert!` macro is provided by the standard library and it used to ensure that a condition in a test evaluates to true.
	- If the value is true: nothing happens, test passes.
	- If the value is false: assert! macro calls panic! causing the test to fail


Testing Equality with the assert_eq! and assert_ne! Macros
Instead of using assert! to test equality with `==`, assert_eq! and assert_ne! macros from the stadard library can test it more conveniently.

`assert_eq!` tests for equality
`assert_ne!` tests for inequality

If the assertion fails two values are printed making it easier to see why the test failed.
`assert!` only indicates that it got a false value for the `==` expression without printing the value that led to the false value.

`assert_ne!` macro is useful for cases when it is not certain what the value should be but it is certain what is should not be.

These macros print there arguments using the debug formatting and must implement the `PartialEq` and `Debug` traits. For structs and enums, `PartialEq` trait must be included to assert equality. Since these traits are derivable, they can be added to the top of each code snippt simply with `#[derive(PartialEq, Debug)]`.


Adding Custom Failure Messages
Custom messages can be added as an optional argument to any of the `assert` macros. Any arguments specified after the required arguments are passed along to the `format!` macro. So a format string in `{}` can be passed to the macro.


Checking for Panics with `should_panic`
`should_panic` helps ensure that code handles error conditions as expected.


