---
applyTo: 'backend/**'
---

# Rust Development Practices

- Run `cargo clippy` after making changes to Rust code to ensure code quality.
  It is fine to wait to do this until the task is complete, so you don't have to run it after intermediate changes.
- When available, use the `cargo clippy --fix` command to automatically fix linting issues instead of manually fixing them.
- Unless you need to run the backend, you can use `cargo check` to quickly check for errors without building the entire project.
- When adding a new Rust dependency, use `cargo search` to find the latest version before adding it to your `Cargo.toml` file.
- When adding a new Rust dependency, make sure to run `cargo update` to update the lock file.
- Prefer to derive the `Error` trait using `thiserror` for custom error types. This provides a consistent way to handle errors across the codebase.
