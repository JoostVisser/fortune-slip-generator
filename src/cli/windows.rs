use std::process::Command;

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
