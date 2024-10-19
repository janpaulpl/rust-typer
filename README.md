# What is this?

Remember the classic [Hacker Typer website](https://hackertyper.net) we all used to visit back in the day? Well this is the same, but with the Rust repo, and it's written in Rust!

## Features

- Fetch and display random `.rs` files from the official Rust GitHub repository.
- Option to display files from any local directory.
- Gradual text reveal (5 characters per key press) for an interactive experience.
- Ability to exit the game by pressing `ESC`.

## Why You Should Learn Rust

Rust has gained significant popularity due to its strong focus on performance, memory safety, and concurrency. It is an excellent language for both system-level programming and web development, and it ensures that developers write clean and bug-free code with its strict compiler checks.

If you're new to Rust, the language can be a bit challenging at first, but it offers substantial long-term rewards such as:
- **Memory Safety**: Rust helps you avoid common bugs like null pointer dereferencing and buffer overflows.
- **Concurrency**: Rust's ownership system ensures safe concurrency without needing garbage collection.
- **Growing Ecosystem**: With its increasing community support, packages and frameworks in Rust are growing rapidly, making it easier to build reliable and fast applications.

Want to learn Rust? Check out this amazing resource: [Rust Learn Book](https://rust-book.cs.brown.edu). It's a comprehensive guide that will help you get started with Rust!

## Installation

### From Cargo

To install the game, you can use `cargo` (make sure you have the Rust toolchain installed):

cargo install rust-typer

## How to Play

### Fetch from GitHub (Default Mode)

By default, Rust Typer fetches and displays random Rust files from the official Rust GitHub repository. To start playing:

cargo run

### Use a Local Directory

You can also specify a local directory if you want to play the game with your own files:

cargo run -- --local /path/to/local/directory

### Interactive Gameplay

- Press any key to reveal 5 characters at a time.
- Press `ESC` to exit the game.

## Requirements

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))

## Resources

- [Rust Official Website](https://www.rust-lang.org)
- [Rust Learn Book](https://rust-book.cs.brown.edu)
