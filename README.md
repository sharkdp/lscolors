# lscolors

<img src="https://i.imgur.com/RE4Ont5.png" align="right">

[![Build Status](https://travis-ci.org/sharkdp/lscolors.svg?branch=master)](https://travis-ci.org/sharkdp/lscolors)
[![Crates.io](https://img.shields.io/crates/v/lscolors.svg)](https://crates.io/crates/lscolors)
[![Documentation](https://docs.rs/lscolors/badge.svg)](https://docs.rs/lscolors)

A cross-platform library for colorizing paths according to the `LS_COLORS` environment variable (like `ls`).

## Usage

```rust
use lscolors::{LsColors, Style};

let lscolors = LsColors::from_env().unwrap_or_default();

let path = "some/folder/test.tar.gz";
let style = lscolors.style_for_path(path);

// If you want to use `ansi_term`:
let ansi_style = style.map(Style::to_ansi_term_style)
                      .unwrap_or_default();
println!("{}", ansi_style.paint(path));
```

## Command-line application

This crate also comes with a small command-line program `lscolors` that
can be used to colorize the output of other commands:
```bash
> find . -maxdepth 2 | lscolors

> rg foo -l | lscolors
```

You can install it by running `cargo install lscolors` or by downloading one
of the prebuilt binaries from the [release page](https://github.com/sharkdp/lscolors/releases).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## References

Information about the `LS_COLORS` environment variable is sparse. Here is a short list of useful references:

* [`LS_COLORS` implementation in the GNU coreutils version of `ls`](https://github.com/coreutils/coreutils/blob/17983b2cb3bccbb4fa69691178caddd99269bda9/src/ls.c#L2507-L2647) (the reference implementation)
* [`LS_COLORS` implementation in `bfs`](https://github.com/tavianator/bfs/blob/2d3b03183c9f1cdb685977f349bf4bbc74a2038d/color.c#L308) by [**@tavianator**](https://github.com/tavianator)
* [The `DIR_COLORS(5)` man page](https://linux.die.net/man/5/dir_colors)
