pub fn main() {
    println!("cargo:rerun-if-changed=src/arch/aarch64/layout.ld");
    println!("cargo:rerun-if-changed=src/arch/aarch64/crt0.S");
}
