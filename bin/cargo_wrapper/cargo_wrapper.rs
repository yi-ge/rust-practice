use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let output = Command::new("cargo")
        .args(&args[1..])
        .output()
        .expect("Failed to execute Cargo");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("ok");
}
