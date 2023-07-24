use lscolors::LsColors;
use lscolors::Style;

use std::process::Command;

fn get_ls_style(ls_colors_env: Option<&str>, path: &str) -> Option<Style> {
    let output_gnu_ls = {
        let mut cmd = Command::new("ls");

        cmd.arg("--color=always").arg("--directory").arg(path);

        if let Some(ls_colors_env) = ls_colors_env {
            cmd.env("LS_COLORS", ls_colors_env);
        } else {
            cmd.env_remove("LS_COLORS");
        }

        cmd.output().unwrap().stdout
    };

    let output_gnu_ls = String::from_utf8(output_gnu_ls).expect("valid UTF-8 output from ls");

    eprint!("[GNU ls output] = {}", &output_gnu_ls);

    let style_str = output_gnu_ls.trim().trim_start_matches("\x1b[0m\x1b[");

    let end_of_ansi_code = style_str.find(&format!("m{path}", path = path))?;
    let style_str = &style_str[0..end_of_ansi_code];

    // For a proper integration test, we would ideally compare the output on an ANSI-escape-sequence level.
    // Unfortunately, those are not unambiguous. Both \x1b[1m as well as \x1b[01m render something in bold.
    // So instead, we trust our escape sequence parser (and its tests) and compare the output on a `Style`
    // level.
    Style::from_ansi_sequence(&style_str)
}

fn assert_style_matches_ls(lscolors: &LsColors, ls_colors_env: Option<&str>, path: &str) {
    let lscolors_style = lscolors.style_for_path(path);
    let ls_style = get_ls_style(ls_colors_env, path);

    assert_eq!(lscolors_style, ls_style.as_ref());
}

#[cfg(unix)]
#[test]
fn gnu_ls_compatibility_no_ls_colors() {
    let lscolors = LsColors::default();

    assert_style_matches_ls(&lscolors, None, "tests/");
    assert_style_matches_ls(&lscolors, None, "Cargo.toml");
}

#[cfg(unix)]
#[test]
fn gnu_ls_compatibility_custom_ls_colors() {
    {
        let ls_colors_env = "*.toml=01;31";
        let lscolors = LsColors::from_string(ls_colors_env);

        assert_style_matches_ls(&lscolors, Some(ls_colors_env), "tests/");
        assert_style_matches_ls(&lscolors, Some(ls_colors_env), "Cargo.toml");
    }
    {
        let ls_colors_env = "*.toml=01;31:di=32";
        let lscolors = LsColors::from_string(ls_colors_env);

        assert_style_matches_ls(&lscolors, Some(ls_colors_env), "tests/");
        assert_style_matches_ls(&lscolors, Some(ls_colors_env), "Cargo.toml");
    }
}

#[cfg(unix)]
#[test]
fn gnu_ls_compatibility_resetting_styles() {
    {
        let ls_colors_env = "di=31:di=0";
        let lscolors = LsColors::from_string(ls_colors_env);

        assert_style_matches_ls(&lscolors, Some(ls_colors_env), "tests/");
    }
    {
        let ls_colors_env = "*.toml=31:*.toml=32";
        let lscolors = LsColors::from_string(ls_colors_env);

        assert_style_matches_ls(&lscolors, Some(ls_colors_env), "Cargo.toml");
    }
}
