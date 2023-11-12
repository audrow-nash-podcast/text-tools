# README

This is a set of tools to make podcast publication easier.

To use, make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

## Setup

### Running the tests

Run the tests with `cargo test`.

Note, this repo has snapshots with [`insta`](https://insta.rs/). To install `insta` you can use the following command:

```bash
cargo install cargo-insta
```

From there, you can use

```bash
cargo insta review
```

to checkout the modified snapshots, or

```bash
cargo insta test
```

to run tests and then reviews.