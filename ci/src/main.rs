use xshell::{cmd, Shell};

fn main() {
    let sh = Shell::new().expect("Couldn't obtain Shell instance.");

    cmd!(sh, "cargo fmt --all -- --check")
        .run()
        .expect("Format-check failed; run 'cargo fmt --all'.");

    cmd!(sh, "cargo check --all")
        .run()
        .expect("Compile-check failed; fix compile errors above.");

    cmd!(sh, "cargo test")
        .run()
        .expect("Tests failed; fix failing tests above.");
}
