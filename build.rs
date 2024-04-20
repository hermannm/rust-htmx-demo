use std::{env, process::Command};

/**
 * - Generates CSS with Tailwind based on class names in our Rust files to static/styles.css
 * - Compiles our TypeScript files from src/scripts to static/script.js
 */
fn main() -> Result<(), String> {
    println!("cargo::rerun-if-changed=src/app/");
    println!("cargo::rerun-if-changed=src/scripts/");

    let dir: String = env::var("CARGO_MANIFEST_DIR").map_err(|err| err.to_string())?;
    let tailwind_input = format!("{dir}/tailwind.css");
    let tailwind_output = format!("{dir}/static/styles.css");

    let tailwind_err = |err| format!("Failed to run tailwindcss: {err}");
    let typescript_err = |err| format!("Failed to run typescript: {err}");

    // Spawns Tailwind and TypeScript before we wait on them, so they can run concurrently
    let tailwind_process = Command::new("npx")
        .args([
            "--yes",
            "tailwindcss",
            "-i",
            &tailwind_input,
            "-o",
            &tailwind_output,
        ])
        .spawn()
        .map_err(tailwind_err)?;
    let typescript_process = Command::new("npx")
        .arg("tsc")
        .spawn()
        .map_err(typescript_err)?;

    let tailwind_output = tailwind_process.wait_with_output().map_err(tailwind_err)?;
    if !tailwind_output.status.success() {
        let error = String::from_utf8_lossy(&tailwind_output.stderr);
        println!("cargo::warning=Tailwind: {error}");
        return Err("Tailwind CSS generation failed".to_string());
    }

    let typescript_output = typescript_process
        .wait_with_output()
        .map_err(typescript_err)?;
    if !typescript_output.status.success() {
        let error = String::from_utf8_lossy(&tailwind_output.stderr);
        println!("cargo::warning=Typescript: {error}");
        return Err("TypeScript compilation failed".to_string());
    }

    Ok(())
}
