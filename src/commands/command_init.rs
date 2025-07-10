pub use crate::internal::*;
use std::fs;
use std::io::{self, Write};

/// General initialization command that attempts to make it easy for the
/// user to clone an existing remote repo or create a remote repo all
/// from the command-line.
///
pub async fn command_init() -> Result<()> {
    let mut app = App::new();
    // Step 1: Check if guidebook root directory exists with .git subfolder
    let home = dirs::home_dir().ok_or_else(|| anyhow!("HOME directory not found"))?;
    let guidebook_root = home.join(".local/share/guidebook");
    let git_dir = guidebook_root.join(".git");

    //
    // Step 1: Check if we're already all set...
    //
    if guidebook_root.exists() && git_dir.exists() {
        let root = guidebook_root.display();
        let (git_status, git_status_color) = match app.git_status() {
            Ok(status) if status.is_empty() => ("working tree clean", "success_dim"),
            Ok(_) => ("modified", "warn"),
            Err(e) => {
                return Err(e.into());
            }
        };
        cprintln!(
            "txt",
            r#"
guidebook root directory already exists: [{root}](filename)
git status: [{git_status}]({git_status_color})

[✓ You're ready to go!](success)
            "#,
        );

        return Ok(());
    }

    //
    // Step 2: Explain the user what we're going to do and give them
    // some options.
    //
    cprintln!(
        "txt",
        "{}",
        r#"
[guidebook-plan could not find an existing data directory.](warn)

guidebook-plan stores its data in a remote GitHub repository called [guidebook-local](filename)
If you already have a remote [guidebook-local](filename) repository on GitHub, you can clone it
locally to this machine.  If you do not have a remote repository, guidebook-plan can
create one for you.

Would you like to:

1. [Create](option) a new guidebook-local repository on GitHub
2. [Clone](option) an existing guidebook-local repository from GitHub
3. [Exit](option) without making changes
"#
    );

    cprintln!("text", "");
    cprint!("text", "Enter your choice (1-3): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();

    cprintln!("", "");

    //
    // Step 3: Handle user choice...
    //
    match choice {
        "2" => {
            return crate::command_clone();
        }
        "1" => {
            // Step 3: Create new repository using GitHub OAuth device flow
            cprintln!(
                "",
                "\nCreating a new [guidebook-local](filename) repository..."
            );

            match create_github_repo(&guidebook_root, "guidebook-local").await {
                Ok(_) => {
                    cprintln!(
                        "success",
                        "✓ Successfully created and cloned the repository!"
                    );
                }
                Err(e) => {
                    cprintln!("error", "Failed to create repository: {}", e);
                    cprintln!(
                        "warn",
                        r#"
guidebook-plan failed to create a new 'guidebook-local' repository remotely. You can still 
manually create a 'guidebook-local' repository on GitHub and then use the clone it locally
with the `guidebook-plan clone` command.
"#
                    );
                    return Ok(());
                }
            }
        }
        "3" => {
            return Ok(());
        }
        _ => {
            cprintln!("error", "Invalid choice. Exiting.");
            return Ok(());
        }
    }

    //
    // Step 4: At this point, the guidebook root exists and is a git repository
    //
    let plan_dir = guidebook_root.join("guidebook-plan");
    let plan_file = plan_dir.join("plan.yaml");

    if plan_file.exists() {
        cprintln!(
            "success",
            "✓ Guidebook plan is already initialized and ready to go!"
        );
        return Ok(());
    }

    // Create the default plan.yaml file
    cprintln!("", "Creating default plan.yaml file...");
    if !plan_dir.exists() {
        fs::create_dir_all(&plan_dir)?;
    }
    let default_plan = create_default_plan();
    let yaml_content = serde_yaml::to_string(&default_plan)?;
    fs::write(&plan_file, yaml_content)?;

    cprintln!("success_dim", "✓ Created default plan");
    cprintln!("success_dim", "✓ guidebook-plan is now ready to go!");

    Ok(())
}

fn create_default_plan() -> Plan {
    use std::collections::HashMap;

    let mut activities_map = HashMap::new();
    activities_map.insert(DayOfWeek::Default, vec!["Default activity".to_string()]);

    let routines = vec![
        Routine {
            name: "Wake".to_string(),
            start: Some("7:00 am".to_string()),
            duration: "30m".to_string(),
            activities: {
                let mut map = HashMap::new();
                map.insert(DayOfWeek::Default, vec!["Get up and get ready".to_string()]);
                map
            },
        },
        Routine {
            name: "Work".to_string(),
            start: Some("9:00 am".to_string()),
            duration: "3h".to_string(),
            activities: {
                let mut map = HashMap::new();
                map.insert(DayOfWeek::Default, vec!["Morning work session".to_string()]);
                map
            },
        },
        Routine {
            name: "Lunch".to_string(),
            start: Some("12:00 pm".to_string()),
            duration: "1h".to_string(),
            activities: {
                let mut map = HashMap::new();
                map.insert(DayOfWeek::Default, vec!["Lunch break and rest".to_string()]);
                map
            },
        },
        Routine {
            name: "Work".to_string(),
            start: Some("1:00 pm".to_string()),
            duration: "4h".to_string(),
            activities: {
                let mut map = HashMap::new();
                map.insert(
                    DayOfWeek::Default,
                    vec!["Afternoon work session".to_string()],
                );
                map
            },
        },
        Routine {
            name: "Evening".to_string(),
            start: Some("6:00 pm".to_string()),
            duration: "3h".to_string(),
            activities: {
                let mut map = HashMap::new();
                map.insert(
                    DayOfWeek::Default,
                    vec!["Evening activities and relaxation".to_string()],
                );
                map
            },
        },
        Routine {
            name: "Sleep".to_string(),
            start: Some("10:00 pm".to_string()),
            duration: "9h".to_string(),
            activities: {
                let mut map = HashMap::new();
                map.insert(DayOfWeek::Default, vec!["Rest and sleep".to_string()]);
                map
            },
        },
    ];

    Plan {
        daily_routine: routines,
    }
}

async fn create_github_repo(guidebook_root: &std::path::Path, repo_name: &str) -> Result<()> {
    use reqwest::Client;
    use serde_json::{json, Value};
    use std::process::Command;
    use std::time::Duration;

    // Note: the client_id is safe to share publicly as it is not a secret.
    let client_id = "Ov23li9pg6Ls56QNMGxt";
    let client = Client::new();

    // Step 1: Request device and user codes
    let device_response = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id),
            ("scope", "repo"), // Need repo scope to create private repositories
        ])
        .send()
        .await?;

    if !device_response.status().is_success() {
        let status = device_response.status();
        let error_text = device_response.text().await?;
        return Err(anyhow!(
            "Failed to initiate device flow: {}. Error: {}",
            status,
            error_text
        ));
    }

    let device_data: Value = device_response.json().await?;
    let device_code = device_data["device_code"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing device_code in response"))?;
    let user_code = device_data["user_code"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing user_code in response"))?;
    let verification_uri = device_data["verification_uri"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing verification_uri in response"))?;
    let interval = device_data["interval"].as_u64().unwrap_or(5);

    // Step 2: Display instructions to user
    cprintln!("", "");
    cprintln!(
        "",
        "To authorize this application, visit: [{}]({})",
        verification_uri,
        verification_uri
    );
    cprintln!("", "");
    cprintln!("", "And enter the code: [{}](#4CF)", user_code);
    cprintln!("", "");
    cprintln!("", "Waiting for authorization...");

    // Step 3: Poll for access token
    let mut access_token: Option<String> = None;
    let max_attempts = 60; // 5 minutes max

    for _ in 0..max_attempts {
        tokio::time::sleep(Duration::from_secs(interval)).await;

        let token_response = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&[
                ("client_id", client_id),
                ("device_code", device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .await?;

        let token_data: Value = token_response.json().await?;

        if let Some(token) = token_data["access_token"].as_str() {
            access_token = Some(token.to_string());
            break;
        }

        if let Some(error) = token_data["error"].as_str() {
            match error {
                "authorization_pending" => continue, // Keep polling
                "slow_down" => {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
                "expired_token" => return Err(anyhow!("Device code expired. Please try again.")),
                "access_denied" => return Err(anyhow!("User denied authorization.")),
                _ => return Err(anyhow!("OAuth error: {}", error)),
            }
        }
    }

    let access_token = access_token.ok_or_else(|| anyhow!("Failed to get access token"))?;
    cprintln!("success", "✓ Successfully authorized!");

    // Step 4: Get user information to construct repository URL
    let user_response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "guidebook-plan")
        .send()
        .await?;

    if !user_response.status().is_success() {
        return Err(anyhow!(
            "Failed to get user info: {}",
            user_response.status()
        ));
    }

    let user_data: Value = user_response.json().await?;
    let username = user_data["login"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing username in user response"))?;

    cprintln!("", "Creating repository for user: [{}](#4CF)", username);

    // Step 5: Create the repository
    let repo_response = client
        .post("https://api.github.com/user/repos")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "guidebook-plan")
        .json(&json!({
            "name": repo_name,
            "description": "Personal guidebook planning data",
            "private": true,
        }))
        .send()
        .await?;

    if !repo_response.status().is_success() {
        let error_text = repo_response.text().await?;
        return Err(anyhow!("Failed to create repository: {}", error_text));
    }
    cprintln!("success", "✓ Created private repository '{}'", repo_name);

    // Step 6: Clone the repository locally
    let repo_url = format!(
        "https://{}@github.com/{}/{}.git",
        access_token, username, repo_name
    );
    cprintln!("success", "Cloning repository locally...");

    // Ensure parent directory exists
    if let Some(parent) = guidebook_root.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let output = Command::new("git")
        .args(&["clone", &repo_url, &guidebook_root.to_string_lossy()])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to clone repository: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    cprintln!("success", "✓ Repository cloned successfully");

    Ok(())
}
