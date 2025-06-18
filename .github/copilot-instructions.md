# Development Environment

- This environment uses the fish shell. Make sure to use fish syntax when writing shell commands.
- This environment uses `nix` for package management. The project uses `direnv` to automatically enter a development shell.
- This environment consists of a Rust backend and a Vue 3/TypeScript frontend.

# Task Development Guidelines

Sometimes I will provide you with a task file that describes a specific task to complete.
You should work independently on these tasks, following the instructions provided in the task file.
Report back to me when you have completed the task, or if you encounter any issues that require my attention.
Here are the guidelines for working on these tasks:

- The task file contains the task description, acceptance criteria, and technical requirements.
- These files are located in the `tasks` directory. Only work on the tasks that I assigned to you in the chat.
- When working on a task, ensure that you only complete the steps listed in the task file.
  Do not add additional features or make changes outside the scope of the task.
- If you encounter any issues or have questions about the task, you can ask for clarification.
- Once you have completed the task, make sure to test your changes thoroughly to ensure they meet the acceptance criteria.
- Once the task is complete, run any unit tests, linters, code formatters relevant to the code you are working on.
  Correct any issues that arise from these tools.
- After testing, you must submit/report your changes to me for review.
- When I accept your changes, mark the task as complete. This is done by
  adding a checkmark to the correct bullet point in the Implementation-Plan.md file.
  I must give explict approval before you can mark the task as complete.

# Rust Development Practices

## After finishing coding a task

- Run `cargo fmt --all` to format the code according to the project's style guidelines.
- Run `cargo clippy --examples --all-targets --all-features --fix --allow-dirty` after making changes to Rust code to ensure code quality.
- Run `cargo clippy --examples --all-targets --all-features` to check for any remaining warnings or suggestions and correct them manually.
- Run `cargo test --all-targets --all-features` to ensure all tests pass.

## General Rust Development Guidelines

- Unless you need to run the backend, you can use `cargo check` to quickly check for errors without building the entire project.
- When adding a new Rust dependency, use `cargo search` to find the latest version before adding it to your `Cargo.toml` file.
- When adding a new Rust dependency, make sure to run `cargo update` to update the lock file.
- Prefer to derive the `Error` trait using `thiserror` for custom error types. This provides a consistent way to handle errors across the codebase.
