# JDKMan

[简体中文](README_zh-CN.md)

JDKMan is a cross-platform JDK version manager written in Rust. It allows you to easily manage multiple JDK installations on your system.

## Features

- Add, remove, list, and activate different JDK versions
- Cross-platform support (Windows, macOS, Linux)
- Internationalization support (currently English and Simplified Chinese)

## Installation

### Pre-built Binaries

You can download pre-built binaries for Windows, macOS, and Linux from the [Releases](https://github.com/bli22ard/jdkman/releases) page.

1. Download the appropriate binary for your operating system.
2. Extract the archive (zip for Windows, tar.gz for macOS and Linux).
3. Add the extracted directory to your system's PATH.

### Building from Source

To build JDKMan from source, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from [https://rustup.rs/](https://rustup.rs/).

Once Rust is installed, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/bli22ard/jdkman.git
   cd jdkman
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The compiled binary will be available in `target/release/jdkman` (or `target/release/jdkman.exe` on Windows).

4. (Optional) Add the binary to your system's PATH for easy access.

## Usage

JDKMan provides the following commands:

- `jdkman add <name> <path>`: Add a new JDK
- `jdkman remove <name>`: Remove an added JDK
- `jdkman list`: List all added JDKs
- `jdkman activate <name>`: Activate a specified JDK

For more information on each command, use the `--help` option:
