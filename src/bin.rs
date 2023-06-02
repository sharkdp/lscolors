use std::env;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use lscolors::{LsColors, Style};

#[cfg(all(
    not(feature = "nu-ansi-term"),
    not(feature = "gnu_legacy"),
    not(feature = "ansi_term"),
    not(feature = "crossterm")
))]
compile_error!("one feature must be enabled: ansi_term, nu-ansi-term, crossterm, gnu_legacy");

fn print_path(handle: &mut dyn Write, ls_colors: &LsColors, path: &str) -> io::Result<()> {
    for (component, style) in ls_colors.style_for_path_components(Path::new(path)) {
        #[cfg(any(feature = "nu-ansi-term", feature = "gnu_legacy"))]
        {
            let ansi_style = style.map(Style::to_nu_ansi_term_style).unwrap_or_default();
            write!(handle, "{}", ansi_style.paint(component.to_string_lossy()))?;
        }

        #[cfg(feature = "ansi_term")]
        {
            let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
            write!(handle, "{}", ansi_style.paint(component.to_string_lossy()))?;
        }

        #[cfg(feature = "crossterm")]
        {
            let ansi_style = style.map(Style::to_crossterm_style).unwrap_or_default();
            write!(handle, "{}", ansi_style.apply(component.to_string_lossy()))?;
        }
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

        while let Ok(size) = stdin.lock().read_until(b'\n', &mut buf) {
            if size == 0 {
                break;
            }

            let path_str = String::from_utf8_lossy(&buf[..(buf.len() - 1)]);
            #[cfg(windows)]
            let path_str = path_str.trim_end_matches('\r');
            print_path(&mut stdout, &ls_colors, path_str.as_ref())?;

            buf.clear();
        }
    }

    Ok(())
}

fn main() {
    run().ok();
}
