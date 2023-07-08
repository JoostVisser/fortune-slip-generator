use std::process::Command;

pub fn press_a_key_to_continue_windows_only() {
    if cfg!(target_os = "windows") {
        println!();
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    }
}
