# Parms

A simple CLI tool for making interacting with SSM parameters suck less. The name is a private joke, not a typo.

## Installation

### MacOS

`$ brew install icevans/parms/parms`

### Linux

Not yet supported.

### Building from source

1. [Install rustup](https://rustup.rs)
2. From the root of this repository, run `cargo build --release`
3. Copy `./release/target/parms` to somewhere in your `PATH` variable

## Usage

```
Usage: parms [OPTIONS] <COMMAND>

Commands:
fetch Fetches the value of selected parameter
edit  Allows to edit the current value of selected parameter
help  Print this message or the help of the given subcommand(s)

Options:
-r, --region <REGION>  Search in this AWS region [default: us-west-2]
-h, --help Print help
-V, --version Print version
```