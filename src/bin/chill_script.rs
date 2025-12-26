use std::process::Command;

fn main() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "firefox",
            "https://youtube.com",
            "https://reddit.com",
            "https://instagram.com",
        ])
        .spawn()
        .expect("Failed to open firefox");
}