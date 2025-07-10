use crate::internal::*;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

/// Clones an existing remote repository into the local guidebook root directory.
///
pub fn command_clone() -> Result<()> {
    let app = App::new();
    let guidebook_root = app.guidebook_root(true)?;
    let guidebook_root_pretty = app.guidebook_root_pretty()?;
    let git_dir = guidebook_root.join(".git");

    cprintln!("h1", "Preparing to clone guidebook repository...");

    if app.dir_exists(&git_dir) {
        cprintln!(
            "warn",
            "git repository already found at [{}](filename)",
            guidebook_root_pretty,
        );
    } else {
        do_clone(&app)?;
    }

    cprintln!("", "");
    cprintln!("", "Contents of [{}](filename):", guidebook_root_pretty);
    print_dir(&guidebook_root, "    ")?;
    cprintln!("success", "\nYou're ready go!");

    Ok(())
}

fn do_clone(app: &App) -> Result<()> {
    let guidebook_root = app.guidebook_root(true)?;

    cprint!("", "Enter your GitHub username: ");

    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let username = input.trim();
    if username.is_empty() {
        cprintln!(
            "warn",
            "GitHub username is required to initialize the repository."
        );
        std::process::exit(1);
    }

    let repo_name = format!("{}/guidebook-local.git", username);
    let parent_dir = guidebook_root.parent().unwrap();
    std::fs::create_dir_all(parent_dir)?;

    let root_pretty = app.guidebook_root_pretty()?;
    let args = [
        "clone",
        &format!("git@github.com:{}", repo_name),
        &guidebook_root.to_string_lossy(),
    ];
    let mut pretty_args = args.clone();
    pretty_args[2] = root_pretty.as_str();
    cprintln!("", "Running [git {}](command)", pretty_args.join(" "));

    let output = Command::new("git").args(&args).output()?;
    if !output.status.success() {
        cprintln!(
            "error",
            "Failed to clone repository. Please check the above error and try again."
        );
        std::process::exit(1);
    }
    Ok(())
}

fn print_dir(dir: &PathBuf, prefix: &str) -> Result<()> {
    fn print_dir_recursive(dir: &PathBuf, prefix: &str) -> Result<()> {
        let entries = std::fs::read_dir(dir)?;

        for entry in entries {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.starts_with(".git") {
                continue;
            }

            let is_dir = entry.file_type()?.is_dir();
            let suffix = if is_dir { "/" } else { "" };

            cprintln!("filename", "{}{}{}", prefix, name_str, suffix);
            if is_dir {
                let subdir = entry.path();
                print_dir_recursive(&subdir, &format!("{}    ", prefix))?;
            }
        }
        Ok(())
    }
    print_dir_recursive(dir, prefix)
}
