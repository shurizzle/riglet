use std::{
    env,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    fetch().unwrap();
    make().unwrap();
    generate().unwrap();
}

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn source() -> PathBuf {
    output().join("figlet")
}

fn fetch() -> io::Result<()> {
    let output_base_path = output();
    let clone_dest_dir = "figlet".to_owned();
    let _ = std::fs::remove_dir_all(output_base_path.join(&clone_dest_dir));

    let status = Command::new("git")
        .current_dir(&output_base_path)
        .arg("clone")
        .arg("--depth=1")
        .arg("https://github.com/cmatsuoka/figlet")
        .arg(&clone_dest_dir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
    }
}

fn make() -> io::Result<()> {
    if !Command::new("make")
        .arg("-j")
        .arg(num_cpus::get().to_string())
        .current_dir(&source())
        .status()?
        .success()
    {
        return Err(io::Error::new(io::ErrorKind::Other, "make failed"));
    }

    Ok(())
}

fn generate() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("path.rs");
    let mut test_file = File::create(&destination).unwrap();
    write!(
        test_file,
        "const FIGLET_PATH: &'static str = {:?};",
        source().join("figlet").display()
    )
}
