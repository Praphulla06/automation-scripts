use std::process::Command;

fn main() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "chrome",
            "https://gmail.com",
            "https://csit21.cf",
            "https://chatgpt.com",
            "https://classroom.google.com"
        ])
        .spawn()
        .expect("Failed to open chrome");
}