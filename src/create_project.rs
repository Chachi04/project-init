use exitfailure::ExitFailure;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

pub fn directory(path: &PathBuf) -> Result<(), ExitFailure> {
    if std::path::Path::new(path).is_dir() {
        panic!("{} already exists", path.display());
    }
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub fn laravel() {
    // TODO
    println!("Creating Laravel project");
}

pub fn python(path: &PathBuf) -> Result<(), ExitFailure> {
    let mut create_python_venv = Command::new("python");
    create_python_venv
        .arg("-m")
        .arg("venv")
        .arg("venv")
        .current_dir(path)
        .status()
        .expect("Process failed to execute");
    let mut create_python = Command::new("touch");
    create_python
        .arg("main.py")
        .current_dir(path)
        .status()
        .expect("Process failed to execute");
    Ok(())
}

pub fn rust(path: &PathBuf) {
    let mut create_rust_app = Command::new("cargo");
    create_rust_app
        .arg("init")
        .arg("--vcs")
        .arg("none")
        .current_dir(path)
        .status()
        .expect("Process failed to execute");
}

pub fn cpp(path: &PathBuf) -> Result<(), ExitFailure> {
    let mut create_cpp_app = Command::new("touch");
    create_cpp_app
        .arg("app.cpp")
        .current_dir(path)
        .status()
        .expect("Process failed to execute");
    let mut file = File::create("main.py")?;
    file.write_all(b"HelloWorld")?;
    Ok(())
}

pub fn react(path: &PathBuf) {
    let mut create_react_app = Command::new("npx");
    create_react_app
        .arg("create-react-app")
        .arg(".")
        .current_dir(path)
        .status()
        .expect("Process failed to execute");
}
