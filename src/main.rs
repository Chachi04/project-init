use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use exitfailure::ExitFailure;
use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;

mod create_project;
mod git;

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
enum ProjectType {
    Laravel,
    Python,
    Rust,
    Cpp,
    React,
}

impl std::str::FromStr for ProjectType {
    type Err = serde_json::error::Error;
    fn from_str(s: &str) -> Result<ProjectType, serde_json::error::Error> {
        Ok(serde_json::from_str(&format!("\"{}\"", s.to_lowercase()))?)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Project Init",
    // author = env!("CARGO_PKG_AUTHORS"),
    about = "Easy initialization of projects."
)]
struct Cli {
    /// [Laravel/Python/Rust/Cpp/React]
    #[structopt(short = "l", long = "lang")]
    project_type: Option<ProjectType>,
    /// Initialize a git repository in the project directory
    #[structopt(short = "g", long = "git")]
    git: bool,
    /// Initialize a remote git repository in github
    #[structopt(short = "r", long = "remote")]
    remote: bool,
    /// Name of the project [default=project-directory]
    #[structopt(short = "n")]
    name: Option<String>,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    create_project::directory(&args.path)?;

    match args.project_type {
        Some(ProjectType::Laravel) => create_project::laravel(),
        Some(ProjectType::Python) => create_project::python(&args.path)?,
        Some(ProjectType::Rust) => create_project::rust(&args.path),
        Some(ProjectType::Cpp) => create_project::cpp(&args.path)?,
        Some(ProjectType::React) => create_project::react(&args.path),
        _ => {
            match Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select project language")
                .items(&vec!["Laravel", "Python", "Rust", "Cpp", "React"])
                .default(0)
                .interact()?
            {
                0 => create_project::laravel(),
                1 => create_project::python(&args.path)?,
                2 => create_project::rust(&args.path),
                3 => create_project::cpp(&args.path)?,
                4 => create_project::react(&args.path),
                _ => println!("Some Error"),
            }
        }
    }

    if args.git {
        git::git_init(&args.path)?;
        if args.remote {
            git::git_remote(&args.path)?;
        } else {
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to create a remote repo in github?")
                .interact()?
            {
                git::git_remote(&args.path)?;
            }
        }
    } else {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to initialize a git repository?")
            .interact()?
        {
            git::git_init(&args.path)?;
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to create a remote repo in github?")
                .interact()?
            {
                git::git_remote(&args.path)?;
            }
        }
    }
    Ok(())
}
