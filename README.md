![Project workflow](https://github.com/yatharthmathur/rustic-db/actions/workflows/build-and-test.yml/badge.svg)

# RusticDB
A fast and lightweight KeyValue store written in Rust.

## Project Setup
- Install Rust (https://www.rust-lang.org/tools/install)
    - I am using (rustc 1.75.0)
- Install pre-commit (https://pre-commit.com/)
    - Then run `pre-commit install` in the project directory
- Development setup:
    - Recommended Editor: VSCode
    - VSCode Extensions:
        - Plugin bundle: https://github.com/1YiB/vsc-bundle
        - rust-analyzer
        - crates

## How to run
```RUST_LOG=info cargo run```

## How to run test cases
```cargo test```

# Contributing to rustic-db
Please checkout our contribution doc [here](CONTRIBUTING.md).
