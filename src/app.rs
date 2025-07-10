//! The App struct is facade / convenience object for the common operations
//! that are specific to the guidebook-plan application.
//!
use crate::internal::*;
use std::path::PathBuf;
use std::process::Command;

pub struct App {
    git_dir: Option<PathBuf>,
}

impl App {
    //------------------------------------------------------------------------//
    // Constructor
    //------------------------------------------------------------------------//

    pub fn new() -> Self {
        Self { git_dir: None }
    }

    //------------------------------------------------------------------------//
    // Configuration related
    //------------------------------------------------------------------------//

    pub fn git_dir(&mut self) -> Result<PathBuf> {
        if self.git_dir.is_none() {
            self.git_dir = Some(self.guidebook_root(false)?);
        }
        Ok(self.git_dir.clone().unwrap())
    }

    pub fn guidebook_root(&self, skip_check: bool) -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow!("HOME directory not found"))?;
        let basedir = home.join(".local/share/guidebook");

        if skip_check {
            return Ok(basedir);
        }

        if !basedir.exists() {
            let pretty_basedir = basedir
                .to_string_lossy()
                .replace(&home.to_string_lossy().to_string(), "~");

            cprintln!("#F90", "Guidebook root not found at {}", pretty_basedir);
            cprintln!("#F90", "Perhaps you need to run:");
            cprintln!("#DD8", "guidebook-plan clone");
            std::process::exit(1);
        }

        Ok(basedir)
    }

    pub fn guidebook_root_exists(&self) -> bool {
        let Ok(root) = self.guidebook_root(true) else {
            return false;
        };
        root.exists()
    }

    pub fn guidebook_root_pretty(&self) -> Result<String> {
        let guidebook_root = self.guidebook_root(true)?;
        let home = dirs::home_dir().unwrap();
        let pretty_path = guidebook_root
            .to_string_lossy()
            .replace(&home.to_string_lossy().to_string(), "~");
        Ok(pretty_path)
    }

    pub fn find_data_filename(&self) -> Result<PathBuf> {
        let guidebook_root = self.guidebook_root(false)?;
        Ok(guidebook_root.join("guidebook-plan/plan.yaml"))
    }

    //------------------------------------------------------------------------//
    // File utilities
    //------------------------------------------------------------------------//

    pub fn dir_exists(&self, path: &PathBuf) -> bool {
        path.is_dir()
    }

    //-----------------------------------------------------------------------//
    // git related
    //-----------------------------------------------------------------------//

    pub fn git_status(&mut self) -> Result<String> {
        self.run_git_cmd(&["status", "--porcelain"])
    }

    pub fn run_git_cmd(&mut self, args: &[&str]) -> Result<String> {
        let git_dir = self.git_dir()?;
        let output = Command::new("git")
            .args(args)
            .current_dir(git_dir)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "Git command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
