use std::process::Command;
fn main() {
    let output = Command::new("cmd")
        .args(["/C", "yarns -v"])
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;
    println!("{}", &output.status.success());
    println!("{}", String::from_utf8_lossy(&hello))
}
