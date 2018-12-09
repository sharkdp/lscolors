# lscolors

[![Build Status](https://travis-ci.org/sharkdp/lscolors.svg?branch=master)](https://travis-ci.org/sharkdp/lscolors)

A library for colorizing paths according to the `LS_COLORS` environment variable.

## Usage

```rust
use lscolors::{LsColors, Style};

let lscolors = LsColors::from_env().unwrap_or_default();

let path = "some/folder/archive.zip";
let style = lscolors.style_for_path(path);

let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
println!("{}", ansi_style.paint(path));
```

## CLI example

This crate also comes with a small command-line program `lscolors` that
can be used to colorize the output of other commands:
```bash
> find . -maxdepth 2 | lscolors

> rg foo -l | lscolors
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
