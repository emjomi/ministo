# Ministo

Ministo is a RandomX CPU miner written in rust.

## Instalation

Ensure that you have [Rust](https://www.rust-lang.org/tools/install) and [CMake](https://cmake.org/download/) installed
on your system.

Install Ministo using Cargo:

```bash 
cargo install ministo
```

## Usage

To start mining, execute the following command:

```bash
ministo -o <POOL> -u <WALLET>
```

Here are the arguments you need to provide:

* `<POOL>`: The address (`<URL>:<PORT>`) of mining pool.
* `<WALLET>`: Your wallet address.

Explore other command-line options and their default values using:

```bash
ministo --help
```
