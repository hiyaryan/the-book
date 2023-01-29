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
