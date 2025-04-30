fn main() {
    // run make minimal.wit
    let mut cmd = std::process::Command::new("make");
    cmd.arg("build");
    let output = cmd.output().unwrap();
    if !output.status.success() {
        panic!("make failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    println!("cargo:rerun-if-changed=minimal.wat");
    println!("cargo:rerun-if-changed=minimal.wit");
}
