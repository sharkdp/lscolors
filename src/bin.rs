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
        .unwrap_or_default();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut buf = vec![];
    while let Some(size) = stdin.lock().read_until(b'\n', &mut buf).ok() {
        if size == 0 {
            break;
        }

        let path_str = String::from_utf8_lossy(&buf[..(buf.len() - 1)]);
        let path = Path::new(path_str.as_ref());
        let style = ls_colors.get_style_for(path);

        if let Some(style) = style {
            writeln!(stdout, "{}", style.to_ansi_style().paint(path_str))?;
        } else {
            writeln!(stdout, "{}", path_str)?;
        }

        buf.clear();
    }

    Ok(())
}

fn main() {
    run().ok();
}
