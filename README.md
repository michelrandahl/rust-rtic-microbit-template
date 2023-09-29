# Cargo template for RTIC on microbit-v2
Simple minimal template for setting up a Rust microbit-v2 project.
Contains `flake.nix` for nix development environment and `.neoconf.json` for neovim support.

## Get started
- Run `cargo generate --git https://github.com/michelrandahl/rust-rtic-microbit-template.git --name my-project`
- Run `nix develop` to create a minimal development environment with necessary packages. (You probably have to create a commit first to stop nix from whining about dirty directory)
- remember to change the name of the project inside Cargo.toml
- Physically connect microbit-v2 and run `make embed` (to support dbg debugging) or `make probe-run` for more effecient logging (and no debugging).

There is a lot of make commands available, go have a look in the Makefile.

## Logging
Supports two different types of logging that are enabled with feature flags.
- defmt, https://docs.rs/defmt/latest/defmt/
- rtt, https://docs.rs/rtt-target/0.4.0/rtt_target/

The reason for supporting two different logging implementations, is that defmt must be flashed/deployed using `probe-run` which does not support debugging with dbg. However, the logging performed by defmt might be more effecient so perhaps also more useful when testing things that require a little more performance.

## Debugging
To be able to debug with gdb, you will have to run `make embed`. This will use `cargo embed` to build and flash the program and initialize a gdb server on the microbit.

## Adjustments
- Change the 'rust-channel-version' in `./rust-toolchain.toml`. Default is latest, eg. `nightly`.
- Adjust neovim lsp settings for the project in `.neoconf.json` (requires `folke/neoconf.nvim` plugin)
