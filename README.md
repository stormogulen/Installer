# Transactional Task Execution Program [![Rust](https://github.com/stormogulen/Installer/actions/workflows/rust.yml/badge.svg)](https://github.com/stormogulen/Installer/actions/workflows/rust.yml)

This Rust program illustrates how to implement transactional task execution and rollbacks. It uses a `Transaction` struct to manage tasks and their corresponding rollback functions.

### Program Description

This program showcases the implementation of transactional task execution with rollback functionality. It performs the following actions:

- Defines a Transaction struct to hold task execution and rollback closures.

- Implements functions for different tasks (e.g., copying files, creating directories).

- Executes tasks sequentially and handles rollbacks if needed.

- Prints informative messages during task execution and rollback.

### Features

- Transactional Task Execution: The program groups tasks within transactions. If any task within a transaction fails, the program performs rollbacks for the tasks that have already executed.

- Rollback Logic: Each task is associated with a corresponding rollback function. If a task fails, its rollback function is called to revert any changes made by the task.

### Running Tests

This program includes unit tests that verify the functionality of transactional task execution and rollbacks. To run the tests, use the following command:
```cargo test```
