use exitfailure::ExitFailure;
use std::path::PathBuf;

pub fn directory(path: &PathBuf) -> Result<(), ExitFailure> {
    if std::path::Path::new(path).is_dir() {
        panic!("{} already exists", path.display());
    }
    std::fs::create_dir_all(path)?;
    Ok(())
}
pub fn default() {
    println!("Creating default project");
}
pub fn laravel() {
    println!("Creating Laravel project");
}
pub fn python() {
    println!("Creating Python project");
}
pub fn rust() {
    println!("Creating Rust project");
}
pub fn cpp() {
    println!("Creating Cpp project");
}
pub fn csharp() {
    println!("Creating Csharp project")
}
pub fn react() {
    println!("Creating React project")
}
