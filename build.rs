fn main() {
    println!("cargo:rerun-if-changed=src/app/");

    let dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = format!("{dir}/tailwind.css");
    let output = format!("{dir}/static/styles.css");

    let result = std::process::Command::new("npx")
        .args(["--yes", "tailwindcss", "-i", &input, "-o", &output])
        .output()
        .expect("Unable to generate CSS");

    if !result.status.success() {
        let error = String::from_utf8_lossy(&result.stderr);
        println!("cargo:warning=Unable to build CSS!");
        println!("cargo:warning=Output: {error}");
    }
}
