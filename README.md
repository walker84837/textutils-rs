# textutils-rs

A command-line utility for manipulating text in various ways, inspired by [Convert Case](https://convertcase.net/) and the GNU Coreutils `wc` utility.

## Table of Contents

- [How to Use](#how-to-use)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## How to Use

### Installation

You can use **textutils-rs** by [downloading the latest release](https://github.com/walker84837/textutils-rs/releases) for your platform or compiling from source.
To compile from source, you need to have Rust and Cargo - its package manager installed on your computer. To install it, you can follow the official [Rust installation guide](https://www.rust-lang.org/tools/install). Once you installed Cargo and Rust, you can do

```shell
cargo install --path .
```

### Usage 

```shell
textutils [OPTION] [TEXT]
```

- `-l, --lowercase`: Convert the input text to lowercase.
- `-u, --uppercase`: Convert the input text to uppercase.
- `-r, --reverse`: Reverse the input text.
- `-w, --wordcount`: Count the number of words in the input text.
- `-c, --charcount`: Count the number of characters in the input text.
- `-h, --help`: Display this help message.

### Examples

To convert text to lowercase:

```shell
./textutils-rs -l "This is some Text"
```

To count words in a file:

```shell
./textutils-rs -w "$(cat file.txt)"
```

Reverse a file's text:

```shell
./textutils-rs -r "$(cat file.txt)"
```

## Contributing

Contributions are welcome. If you'd like to contribute, you can do so by following the following procedure.

1. Fork the repository.
2. Clone the forked repository.
3. Make your changes, commit them with a descriptive message.
4. Push the changes, and open a pull request.

## License

This project is licensed under the GPLv3. For more information, look at the [LICENSE](LICENSE.md) file or [the GNU website](https://www.gnu.org/licenses/gpl-3.0.html).

