use std::process::Command;

pub fn press_a_key_to_continue_windows_only() {
    if cfg!(target_os = "windows") {
        println!();
        println!("Press any key to continue...");
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    }
}
