use std::process::Command;

#[cfg(target_os = "windows")]
use anyhow::{bail, Result};
#[cfg(target_os = "windows")]
use font_loader::Font;

pub fn press_a_key_to_continue_windows_only() {
    if cfg!(windows) {
        println!();
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    }
}

pub fn enable_ansi_support() {
    let ansi_support_result = enable_ansi_support::enable_ansi_support();
    if ansi_support_result.is_err() {
        owo_colors::set_override(false);
    }
}


#[cfg(windows)]
pub fn check_if_fonts_are_installed() -> Result<()> {
    let sysfonts = system_fonts::query_all();

    for required_font in &REQUIRED_FONTS {
        if !sysfonts
            .iter()
            .any(|sysfont| sysfont.contains(required_font))
        {
            bail!("Font '{}' is not installed.", required_font);
        }
    }

    Ok(())
}
