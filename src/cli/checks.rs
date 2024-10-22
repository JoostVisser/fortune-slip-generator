use std::path::Path;

use anyhow::{anyhow, bail, Result};
use indoc::printdoc;
use log::debug;
use owo_colors::{OwoColorize, Stream};
#[cfg(unix)]
use rust_fontconfig::{FcFontCache, FcPattern};
use which::which;

use crate::{cli::windows, fortune::fortune_data::FortuneData};
#[cfg(windows)]
use font_loader::system_fonts;

const REQUIRED_FONTS: [&str; 3] = ["Dosis", "Hina Mincho", "Kaushan Script"];

/// Checks for the prerequisites to run the program.
///
/// Exits the program if not all prerequisites are met.
pub fn check_prerequisites(config_path: &Path) -> Result<()> {
    println!("Prerequisites:");

    let inkscape_check = check_if_inkscape_is_installed();
    let fonts_check = check_if_fonts_are_installed();
    let fortune_settings_check = check_if_fortune_settings_are_valid(config_path);

    print_prerequisite("1. Inkscape is installed", &inkscape_check);
    print_prerequisite("2. Fonts are installed", &fonts_check);
    print_prerequisite("3. Fortune settings are valid", &fortune_settings_check);

    println!();

    if inkscape_check.is_err() || fonts_check.is_err() || fortune_settings_check.is_err() {
        printdoc! {"
            For installation details, check: {url}
            (Own risk: you can skip the checks with the `--skip-checks` flag.)

            Exiting program, as not all prerequisites are met.
        ", url = "https://github.com/JoostVisser/fortune-slip-generator/blob/main/README.md"}

        windows::press_a_key_to_continue_windows_only();

        bail!("Not all prerequisites are met.");
    }

    Ok(())
}

fn print_prerequisite(prerequisite: &str, is_installed: &Result<()>) {
    print!("{}: ", prerequisite);

    match is_installed {
        Ok(_) => println!(
            "{}",
            "OK".if_supports_color(Stream::Stdout, |text| text.green()),
        ),
        Err(msg) => println!(
            "{} - {}",
            "Error".if_supports_color(Stream::Stdout, |text| text.red()),
            msg.if_supports_color(Stream::Stdout, |text| text.red()),
        ),
    }
}

pub fn check_if_inkscape_is_installed() -> Result<()> {
    which("inkscape")
        .map(|_| ())
        .map_err(|_| anyhow!("Inkscape is not installed or hasn't been added to PATH."))
}

#[cfg(unix)]
pub fn check_if_fonts_are_installed() -> Result<()> {
    let fonts_cache = FcFontCache::build();

    debug!("Font cache: {:?}", fonts_cache.list());

    for required_font in REQUIRED_FONTS {
        let font_name_query = &FcPattern {
            name: Some(String::from(required_font)),
            ..Default::default()
        };

        let family_name_query = &FcPattern {
            family: Some(String::from(required_font)),
            ..Default::default()
        };

        if fonts_cache.query(family_name_query).is_none()
            && fonts_cache.query(font_name_query).is_none()
        {
            bail!("Font '{}' is not installed.", required_font);
        }
    }

    Ok(())
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

pub fn check_if_fortune_settings_are_valid(config_path: &Path) -> Result<()> {
    FortuneData::open(config_path).map(|_| ())
}
