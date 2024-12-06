# tui-rain-cli

`tui-rain` is a CLI extension of the library crate `tui-rain`. This tool allows you to create and visualize rain-like effects in your terminal using the `tui-rain` library.

## Features

- **Rain Simulation**: Generate and display rain effects directly in your terminal.
- **Customizable**: Adjust the intensity, speed, and appearance of the rain.
- **Lightweight**: Minimal dependencies and easy to install.

## Installation

To install `tui-rain`, you need to have Rust and Cargo installed on your system. You can install the CLI tool using Cargo:

```sh
cargo install tui-rain
```

## Usage

Once installed, you can run `tui-rain` from your terminal:

```sh
tui-rain
```

### Options

- `-t, --type <TYPE>`: Set the type of animation: rain, matrix, snow, emoji (default: rain).
- `-i, --intensity <INTENSITY>`: Set the intensity of the rain (default: 5).
- `-s, --speed <SPEED>`: Set the speed of the rain (default: 1).
- `-c, --color <COLOR>`: Set the color of the rain (default: depending on animation).

Example:

```sh
tui-rain --intensity 10 --speed 2 --color green
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes. Ensure that your code adheres to the existing style and includes tests for any new functionality.

## License

`tui-rain` is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

Special thanks to the contributors of the `tui-rain` library for making this CLI extension possible.
