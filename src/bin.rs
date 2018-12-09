use std::alloc::System;
use std::env;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use lscolors::LsColors;

#[global_allocator]
static A: System = System;

fn run() -> io::Result<()> {
    let ls_colors_env = env::var("LS_COLORS");
    let ls_colors = ls_colors_env
        .as_ref()
        .map(|s| LsColors::from_string(s))
        .unwrap_or(LsColors::default());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let path = Path::new(&line);
        let style = ls_colors.get_style_for(path);

        if let Some(style) = style {
            println!("{}", style.to_ansi_style().paint(path.to_string_lossy()));
        } else {
            println!("{}", path.to_string_lossy());
        }
    }
}
