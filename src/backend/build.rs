use chrono::Utc;
use std::process::Command;

fn main() {
    let build_date = Utc::now().to_rfc3339();

    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    let git_dirty = Command::new("git")
        .args(["diff-index", "--quiet", "HEAD", "--"])
        .status()
        .map(|status| if status.success() { "" } else { "-dirty" })
        .unwrap_or("");

    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=GIT_SUFFIX={}", git_dirty);

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");
}
