use std::process::Command;

fn main() {
    let n = 5;
    let mut child = Command::new("sleep").arg(n.to_string()).spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("After {} secs, reached the end of main", n);
}
