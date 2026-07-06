// Copyright © 2026, Julian Scheffers
// SPDX-License-Identifier: MIT

fn main() {
    let mut cc = cc::Build::new();
    cc.include("flanterm/src");
    if std::env::var("CARGO_FEATURE_ALLOC").is_ok() {
        cc.define("FLANTERM_FB_DISABLE_BUMP_ALLOC", "1");
    }
    cc.file("flanterm/src/flanterm.c");
    cc.file("flanterm/src/flanterm_backends/fb.c");
    cc.compile("flanterm-cc");

    println!("cargo::rustc-link-arg=flanterm-cc")
}
