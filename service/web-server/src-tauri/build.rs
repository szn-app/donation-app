fn main() {
    // fixes rendering issue on linux NVIDIA GPU
    #[cfg(target_os = "linux")] // Apply only for Linux
    println!("cargo:rustc-env=WEBKIT_DISABLE_COMPOSITING_MODE=1");

    tauri_build::build()
}
