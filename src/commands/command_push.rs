use crate::internal::*;
use chrono::Local;

/// Pushes the local changes to the plan to the remote repository.
///
pub fn command_push() -> Result<()> {
    let mut app = App::new();
    let git_status = app.run_git_cmd(&["status", "--porcelain"])?;

    if !git_status.is_empty() {
        let filename = app.find_data_filename()?;
        let home = dirs::home_dir().unwrap();
        let pretty_filename = filename
            .to_string_lossy()
            .replace(&home.to_string_lossy().to_string(), "~");

        let now = Local::now();
        let formatted_date = now.format("%Y-%m-%d %H:%M").to_string();

        cprintln!("#39F", "Pushing changes to git: {}", pretty_filename);

        app.run_git_cmd(&["add", &filename.to_string_lossy()])?;
        app.run_git_cmd(&[
            "commit",
            "-m",
            &format!("Update guidebook plan ({})", formatted_date),
        ])?;
        app.run_git_cmd(&["push"])?;
    } else {
        cprintln!("#3F9", "No changes to push.");
    }

    Ok(())
}
