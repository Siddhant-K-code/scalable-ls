# scalable-ls

> Rust implementation of [this blog or discussion](http://be-n.com/spw/you-can-list-a-million-files-in-a-directory-but-not-with-ls.html)

## Introduction

`scalable-ls` is a Rust-based command-line tool designed to list the contents of directories containing an extremely large number of files. Traditional tools like `ls` can struggle or fail in these environments, but `scalable-ls` is optimized to handle such scenarios efficiently.

## Installation

Ensure you have Rust and Cargo installed on your system. Clone this repository and navigate to the project directory. Build the project using Cargo:

```sh
cargo build --release
```

## Usage

### Create empty files

To create a directory and populate it with empty files under `demo-files` directory, use the following command:

```sh
./create_files.sh
```

To list the files in a directory, use the following command:

```sh
cargo run -- [OPTIONS] <PATH>
```

Options:

- `-b, --buf_size <BUF_SIZE>`: Set the buffer size for reading directory entries (default is 5MB).

Example:

```sh
cargo run -- ./demo_files/
```

```sh
cargo run -- --buf-size 1000000 ./demo_files/
```

## Features

- Handles directories with millions of files.
- Efficient directory listing without hanging or crashing.
- Customizable buffer size for performance tuning.

## Example

For 20000 files, both seem to be working. But, it could fail for larger number of files & also output is inconsistent in native `ls` in case of large numbers of files:

![image](https://github.com/Siddhant-K-code/scalable-ls/assets/55068936/4fbcade5-f4df-481f-be04-be541003422f)
