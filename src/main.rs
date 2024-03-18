use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("ls")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let output = Command::new("grep")
        .arg("hello")
        .arg("file.txt")
        .output()
        .expect("failed to execute process");

    println!("grep: {}", String::from_utf8_lossy(&output.stdout));

    let grep_cat = Command::new("grep")
        .arg("hello")
        .arg("file.txt")
        // .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn grep");

    let cat_sort = Command::new("sort")
        .stdin(grep_cat.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn sort");

    let output = cat_sort.wait_with_output().expect("failed to wait on sort");

    println!("sorted: {}", String::from_utf8_lossy(&output.stdout));
}
