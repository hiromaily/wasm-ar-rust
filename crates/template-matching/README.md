# template-matching

This is based on [template-matching](https://crates.io/crates/template-matching), GPU-accelerated template matching library for Rust.

I updated dependencies and fixed some problems like untreated errors and asynchronous processing.

## Changed log

`TemplateMatcher::new` has been changed to the asynchronous function async. `pollster::block_on` has been removed from this function, and asynchronous processing has been applied using `await`.

Similarly, `wait_for_result` has been made into an asynchronous function and `pollster::block_on` has been replaced with `await`.

`match_template` itself has been changed to an asynchronous function, and the caller has been changed accordingly.
