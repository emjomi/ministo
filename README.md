# Ministo

## Building

Before you begin, make sure you have installed the necessary [build dependencies](https://github.com/tari-project/randomx-rs?tab=readme-ov-file#build-dependencies) for RandomX.

Clone the repository and navigate to the project directory:

```bash 
git clone https://github.com/emjomi/ministo.git
cd ./ministo
```

Then build Ministo using Cargo:

```bash 
cargo build --release
```

The compiled binary will be available in the `./target/release/` directory.

## Usage

To start mining, execute the following command:

```bash
./ministo -o <POOL> -u <WALLET>
```

Replace `<POOL>` with your desired mining pool address and `<WALLET>` with your wallet address. 

Ministo will utilize default settings for any omitted parameters. To explore available command line options, use:

```bash
./ministo --help
```
