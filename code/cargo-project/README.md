## What's cargo again ?

Cargo is Rust’s build system and package manager.

## Creating a project with Cargo

```
$ cargo new hello_cargo
$ cd hello_cargo

```

The first command creates a new directory and project called hello_cargo. We’ve named our project hello_cargo, and Cargo creates its files in a directory of the same name.

Go into the hello_cargo directory and list the files. You’ll see that Cargo has generated two files and one directory for us: a Cargo.toml file and a src directory with a main.rs file inside.

It has also initialized a new Git repository along with a .gitignore file.

## Building and Running a Cargo Project

From your hello_cargo directory, build your project by entering the following command:

```
$ cargo build

```

This creates an executable file inside **target/debug**

Using **cargo run** is more convenient than having to remember to run **cargo build** and then use the whole path to the binary, so most developers use cargo run
