# Contributing

Contributions are welcome. This document outlines how to contribute to the project.

## Development Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd slack-sdk
   ```

2. Install Rust (1.75 or later):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run tests:
   ```bash
   cargo test --lib
   ```

## Running Integration Tests

Integration tests require Slack credentials:

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Add your credentials to `.env`:
   ```bash
   SLACK_XOXC_TOKEN=xoxc-...
   SLACK_XOXD_COOKIE=xoxd-...
   # or
   SLACK_BOT_TOKEN=xoxb-...
   ```

3. Run integration tests:
   ```bash
   cargo test --tests
   ```

## Code Style

- Follow standard Rust conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Write documentation for public APIs
- Prefer explicit error handling over panics

## Adding a New API Module

1. Create a new file in `src/api/`:
   ```rust
   //! Module description

   use crate::client::SlackClient;
   use crate::error::Result;
   use serde::{Deserialize, Serialize};

   pub struct NewApi {
       client: SlackClient,
   }

   impl NewApi {
       pub(crate) fn new(client: SlackClient) -> Self {
           Self { client }
       }

       pub async fn method(&self) -> Result<Response> {
           // Implementation
       }
   }
   ```

2. Add the module to `src/api/mod.rs`:
   ```rust
   pub mod new_api;
   ```

3. Add accessor method to `SlackClient` in `src/client.rs`:
   ```rust
   pub fn new_api(&self) -> NewApi {
       NewApi::new(self.clone())
   }
   ```

4. Create integration tests in `tests/integration_new_api.rs`

## Submitting Changes

1. Fork the repository
2. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature
   ```

3. Make your changes with clear commit messages

4. Ensure tests pass:
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy
   ```

5. Submit a pull request with:
   - Clear description of changes
   - Reference to any related issues
   - Test coverage for new functionality

## Reporting Issues

When reporting issues, include:

- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Relevant error messages or logs

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
