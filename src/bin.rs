use std::env;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use lscolors::{LsColors, Style};

fn print_path(handle: &mut dyn Write, ls_colors: &LsColors, path: &str) -> io::Result<()> {
    for (component, style) in ls_colors.style_for_path_components(Path::new(path)) {
        let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
        write!(handle, "{}", ansi_style.paint(component.to_string_lossy()))?;
    }
    writeln!(handle)?;

    Ok(())
}

fn run() -> io::Result<()> {
    let ls_colors = LsColors::from_env().unwrap_or_default();

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut args = env::args();

    if args.len() >= 2 {
        // Skip program name
        args.next();

        for arg in args {
            print_path(&mut stdout, &ls_colors, &arg)?;
        }
    } else {
        let stdin = io::stdin();
        let mut buf = vec![];
        while let Some(size) = stdin.lock().read_until(b'\n', &mut buf).ok() {
            if size == 0 {
                break;
            }

            let path_str = String::from_utf8_lossy(&buf[..(buf.len() - 1)]);

            print_path(&mut stdout, &ls_colors, path_str.as_ref())?;

            buf.clear();
        }
    }

    Ok(())
}

fn main() {
    run().ok();
}
