fn main() {
    let arch = match std::env::consts::ARCH {
        "x86" | "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        _ => "unknown",
    };

    let os = match std::env::consts::OS {
        "linux" => "linux",
        "macos" => "darwin",
        "windows" => "windows",
        _ => "unknown",
    };

    println!("{arch}-{os}")
}
