# MySQL Environment Configurator CLI

This __CLI__ tool is designed to simplify the process of configuring environment variables for __MySQL__ on __Windows__. It allows users to easily set up the `MYSQLCLIENT_LIB_DIR` and `MYSQLCLIENT_VERSION` environment variables, as well as update the system `PATH` with the appropriate MySQL binary directory.

## Purpose

The main purpose of this CLI is to automate the configuration of MySQL paths in the environment variables, which is especially useful for developers who frequently switch between different versions of MySQL on their Windows machines. By using this tool, you can quickly configure your environment to point to the correct MySQL directories without manually editing the system settings.

## Requirements

- **Operating System**: This CLI is only compatible with Windows. If you try to run it on a different operating system, it will exit with an error message.
- **Rust**: You need to have Rust installed on your machine to compile and run this CLI.

## Installation

To use this CLI, clone the repository and build it using Cargo:

```bash
git clone https://github.com/germanfica/mysql-env-cli-rust.git
cd mysql-env-cli-rust
cargo build --release
```
