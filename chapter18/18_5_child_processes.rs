use std::process::Command;

fn main() {
    let output = Command::new("rustcc")
                 .arg("--version")
                 .output()
                 .unwrap_or_else(|e| {
                    panic!("Failed to execute process: {}", e);
                 });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("restc failed and stderr was:\n{}", s);
    }
}
