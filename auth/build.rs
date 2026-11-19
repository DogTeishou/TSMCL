use std::path::PathBuf;

fn main() {
    let env_path = PathBuf::from(std::env::current_dir().unwrap().parent().unwrap())
        .join(".env");
    println!("cargo:rerun-if-changed={}", env_path.display());
    dotenvy::from_path(&env_path).ok();
    if let Ok(client_id) = std::env::var("TSMCL_CLIENT_ID") {
        println!("cargo:rustc-env=TSMCL_CLIENT_ID={}", client_id);
    }
}