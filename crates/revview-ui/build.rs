// Runs the Slint compiler to generate Rust code from your .slint files.

fn main() {
    slint_build::compile("dashboard.slint").unwrap();
}
