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
3. Copy `./target/release/parms` to somewhere in your `PATH` variable

## Usage

`parms` respects the usual AWS configuration methods. First, it will look in
AWS environment variables, then in your ~/.aws/config file. If you connect via
SSO, be sure that you have logged in via SSO, and that you have selected a profile
by setting the `AWS_PROFILE` environment variable.

A note on region:

1. If you provide the `--region` argument, this takes precedence
2. If you omit this, `parms` will check for a `AWS_REGION` or `AWS_DEFAULT_REGION` environment variable
3. If this is missing, `parms` will check for a region on your selected profile
4. If this too is missing, you will get an error

```
Usage: parms [OPTIONS] <COMMAND>

Commands:
  create  Creates a new parameter
  fetch   Fetches the value of selected parameter
  edit    Allows to edit the current value of selected parameter
  delete  Delete a parameter
  help    Print this message or the help of the given subcommand(s)

Options:
  -r, --region <REGION>  Search in this AWS region
  -h, --help             Print help
  -V, --version          Print version
```

