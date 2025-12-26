use std::process::Command;

fn main() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "firefox",
            "https://youtube.com",
            "https://github.com",
            "https://chatgpt.com",
            "https://doc.rust-lang.org/book/title-page.html",
        ])
        .spawn()
        .expect("Failed to open firefox");

    Command::new("cmd")
        .args([
            "/C",
            "start",
            "code",
        ])
        .spawn()
        .expect("Failed to open VsCode");
}