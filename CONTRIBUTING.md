# Contributing to rustic-db

Thank you for considering contributing to **rustic-db**! This document outlines some guidelines and information to help you contribute effectively.

## Code of Conduct

Please read and adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Make sure your interactions with the community are respectful and align with our shared values.

## Getting Started
- Please go through the [README.md](README.md)
- Fork the repository and clone your fork to your local machine.
- Create a new branch for your contribution:

  ```bash
  git checkout -b {username}_feature_name
  ```
  > Note: `{username}` here can be your initials as well
  >> eg: `ym_add_hmap` as my initials are `ym`

## Development guidelines

### Code Style
- Follow the Rust Style Guide.
- Use `rustfmt` to format your code:
    ```bash
    cargo fmt
    ```
    > NOTE: A pre-commit hook is already set up to format the code for you.

### Testing
- Please add a unit test for any feature you work on.
- To run tests, you can use the command:
    ```bash
    cargo test
    ```

### Documentation
- Document your code using Rust doc comments.
- Keep the README up-to-date with relevant information.
- (WIP) Add documentation to the `rustic-db-docs`.

### Commit messages
- Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification for your commit messages.

### Pull requests
- Open a pull request against the `main` branch (or any branch that your feature depends on).
- Ensure your pull request includes relevant tests.
- Reference the issue your pull request is addressing, if applicable.

### Issues
- Use the [GitHub Issues](https://github.com/yatharthmathur/rustic-db/issues) section to report bugs, request features, or ask questions.

### License
- By contributing to **rustic-db**, you agree that your contributions will be licensed under the [MIT License](https://opensource.org/license/mit/).

Thank you for contributing to **rustic-db**! 🚀