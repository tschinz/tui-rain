##################################################
# Variables
#
rust_env := "rustup show"
rust_edition := "2021"
open := if os() == "linux" {
  "xdg-open"
} else if os() == "macos" {
  "open"
} else {
  "start \"\" /max"
}
app_name := "tui-rain-cli"
args := ""
project_directory := justfile_directory()
release := `git describe --tags --always`
url := "https://github.com/tschinz/tui-rain-cli"
##################################################
# COMMANDS
#

# List all commands
@default:
  just --list

# Information about the environment
@info:
  echo "Environment Informations\n------------------------\n"
  echo "OS   : {{os()}}({{arch()}})"
  echo "Open : {{open}}"
  echo "Rust :"
  echo "`{{rust_env}}`"

# Install dependencies
install:
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  cargo install --locked trunk

# install the release version (default is the latest)
install-release release=release:
  cargo install --git {{url}} --tag {{release}}

# install the nightly release
install-nightly:
  cargo install --git {{url}}

# Run the program in debug mode
run args=args:
  cargo run -- {{args}}

# Run the program with default snow
snow:
  cargo run -- -t snow

# Run the program with default matrix
matrix:
    cargo run -- -t matrix

# Run the program with default rain
rain:
  cargo run -- -t rain

# Run the program with default emoji
emoji:
  cargo run -- -t emoji

# Run the program with default data
data:
  cargo run -- -t data

# Build and copy the release version of the program
build:
  cargo build --release
  mkdir -p bin && cp target/release/{{app_name}} bin/

# Run rustfmt with custom configuration
rustfmt:
  find {{invocation_directory()}} -name \*.rs -exec rustfmt --config tab_spaces=2 --edition {{rust_edition}} {} \;
