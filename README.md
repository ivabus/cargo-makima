# cargo-makima

Makima's here to abuse you when running cargo~

Fork of cargo-mommy

# Installation

Install cargo-makima like you would any other cargo extension~

```text
> cargo install cargo-makima
```

# Usage

Run whatever cargo command you would normally but add makima after cargo~

```text
> cargo makima test

    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src\main.rs (target\debug\deps\cargo_makima-3804b5c850d46137.exe)

running 1 test
test test ... FAILED

failures:

---- test stdout ----
thread 'test' panicked at 'oops!!', src\main.rs:26:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin cargo-makima`

What could I do to absolutely deny you of a normal life?️

>_
```
