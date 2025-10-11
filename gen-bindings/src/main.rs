fn main() {
    #[cfg(target_os = "windows")]
    windows();
}

#[cfg(target_os = "windows")]
fn windows() {
    let output_file = format!("..\\bindings\\src\\bindings.rs");
    let args: [&str; _] = [
        "--in",
        "default",
        ".windows/pluginauthenticator.winmd",
        "--sys",
        "--sys-fn-ptrs",
        "--filter",
        "Windows.PluggablePasskeys",
        "--out",
        &output_file,
    ];

    windows_bindgen::bindgen(args).unwrap();
}