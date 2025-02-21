use std::process::Command;
use std::path::Path;

fn main() {
    let sdrad_cdir = Path::new("./secure-rewind-and-discard/src");
    
    Command::new("sh")
            .arg("-c")
            .arg("make")
            .current_dir(&sdrad_dir)
            .status().unwrap();

    let original_ld_library_path = env::var("LD_LIBRARY_PATH").unwrap_or_default();
    println!(r"cargo:rustc-link-search={}", &sdrad_dir.display());
    println!(r"cargo:rustc-env=LD_LIBRARY_PATH={}:{}", &sdrad_dir.display(), &original_ld_library_path);
}
