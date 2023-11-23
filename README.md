# README

This is a set of tools to make podcast publication easier.

* [`description_maker`](./description_maker/): Create text used for promoting the podcast on several platforms

* [`transcript_marker`](./transcript_marker/): Interleaves the outline with a transcript and adds a table of contents.

## Setup

To use, make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

You can install either of these executables with `cargo install --path`.


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