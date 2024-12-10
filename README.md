# tui-rain-cli

`tui-rain-cli` is a CLI extension of the library crate `tui-rain`. This tool allows you to create and visualize rain-like effects in your terminal using the `tui-rain` library.

## Features

- **Rain Simulation**: Generate and display rain effects directly in your terminal.
- **Customizable**: Adjust the intensity, speed, and appearance of the rain.
- **Lightweight**: Minimal dependencies and easy to install.

## Installation

To install `tui-rain-cli`, you need to have Rust and Cargo installed on your system. You can install the CLI tool using Cargo:

```sh
cargo install tui-rain-cli
```

## Usage

Once installed, you can run `tui-rain-cli` from your terminal:

```sh
tui-rain-cli
```

### Options

```bash
CLI wrapper around tui-rs to create terminal rain effects

Usage: tui-rain-cli [OPTIONS]

Options:
  -t, --rain-type <RAIN_TYPE>
          Type of rain effect [rain|matrix|snow|data|emoji] [default: rain] [possible values: rain, matrix, snow, data, emoji]
  -d, --density <DENSITY>
          Rain density computes the number of drops based on the frame size. Lower value is denser
  -s, --speed <SPEED>
          Rain speed in pixels / second
  -v, --variance-speed <VARIANCE_SPEED>
          Rain speed variance
  -l, --lifespan-tail <LIFESPAN_TAIL>
          Tail lifespan in milliseconds
  -c, --color <COLOR>
          Color of the rain [black|red|green|yellow|blue|magenta|cyan|gray|darkgray|lightred|lightgreen|lightyellow|lightblue|lightmagenta|lightcyan|white]
  -k, --head-color <HEAD_COLOR>
          Color of the rain [black|red|green|yellow|blue|magenta|cyan|gray|darkgray|lightred|lightgreen|lightyellow|lightblue|lightmagenta|lightcyan|white]
  -e, --effect-dim <EFFECT_DIM>
          Dim effect [possible values: true, false]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

Example:

```sh
tui-rain-cli --intensity 10 --speed 2 --color green
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes. Ensure that your code adheres to the existing style and includes tests for any new functionality.

## License

`tui-rain-cli` is licensed under the MIT License.

## Acknowledgements

Special thanks to the contributors of the `tui-rain` library for making this CLI extension possible.
