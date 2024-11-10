<h3 align="center">Rust Gzip Compression Tool</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/bangadam/rust-compression.svg)](https://github.com/bangadam/rust-compression/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/bangadam/rust-compression.svg)](https://github.com/bangadam/rust-compression/pulls)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg)](/LICENSE)

</div>

---

<p align="center"> A simple command-line tool for compressing files using Gzip compression.
    <br>
</p>

## Features

- Compresses files using Gzip compression.
- Provides command-line interface for specifying source and target files.

## Requirements

- Rust (latest stable version)
- Cargo (Rust package manager)

## Installation

1. Clone the repository:

```sh
git clone https://github.com/bangadam/rust-compression.git
```

2. Build the project:

```sh
cargo build --release
```

3. Run the executable:

```sh
./target/release/rust-compression <source_file>  <target_file>
```

## Usage

The tool compresses the source file and writes the compressed data to the target file. The source file is not modified.

```sh
rust-compression <source_file> <target_file>
```

## Author

- LinkedIn - [Muhammad Meganata Adam](https://www.linkedin.com/in/bangadam/)
