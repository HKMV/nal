fn main() {
    build()
}

#[cfg(target_os = "windows")]
fn build(){
    println!("Windows env");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=patches/");
    cargo_patch::patch().expect("Failed while patching");
    return;
}

fn build(){
    println!("Other env");
}