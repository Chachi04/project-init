use exitfailure::ExitFailure;
use std::path::PathBuf;
use std::process::Command;

pub fn git_init(path: &PathBuf) -> Result<(), ExitFailure> {
    if cfg!(target_os = "windows") {
        panic!("Windows has no yet been implemented")
    }
    let mut git_init = Command::new("git");
    git_init
        .arg("init")
        .current_dir(path)
        .status()
        .expect("Process failed to execute");
    Ok(())
}

pub fn git_remote(path: &PathBuf) -> Result<(), ExitFailure> {
    let mut create_remote = Command::new("gh");
    let project_name = "Test_Project";
    create_remote
        .current_dir(path)
        .arg("repo")
        .arg("create")
        .arg(project_name)
        .status()
        .expect("Process failed to execute");
    Ok(())
}
