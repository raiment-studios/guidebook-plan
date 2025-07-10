use crate::internal::*;

use anyhow::Result;

pub fn command_show() -> Result<()> {
    let plan = load_plan()?;
    let mut app = App::new();

    let day_of_week = plan.current_day_of_week();
    let current_mins = plan.current_time_mins();

    let pretty_filename = app.guidebook_root_pretty()?;
    let git_status = app.git_status()?;

    println!();
    cprintln!(
        "#FC1",
        "{} {}",
        plan.current_day_of_week_pretty(),
        plan.current_time_pretty()
    );
    cprintln!("#531", "filename:   [{}](#531)", pretty_filename);

    if !git_status.is_empty() {
        cprintln!("#F90", "git status: [modified](#F90)");
    } else {
        cprintln!("#531", "git status: up to date");
    }

    cprintln!("#555", "{}", "-".repeat(70));

    let current_index = plan.current_activity_index()?;

    if plan.daily_routine.is_empty() {
        return Ok(());
    }

    let first_start = plan.daily_routine[0].start_minutes()?.unwrap_or(0);
    let mut cursor = TimeCursor::new(first_start);

    for (index, routine) in plan.daily_routine.iter().enumerate() {
        let dist = (current_index - index as i32).abs();

        if let Some(activity_start) = routine.start_minutes()? {
            cursor.set(activity_start);
        }

        let activities = routine.activities(day_of_week.clone());

        let (c0, c1, c2, c3) = if dist == 0 {
            ("#555", "#738", "#55C", "#8DF")
        } else {
            ("#444", "#555", "#666", "#69B")
        };

        if dist == 0 {
            cprintln!("#FC1", "{}", "-".repeat(70));
        }

        let duration_text = if dist == 0 {
            let remaining = cursor.minutes_remaining(current_mins, routine.duration()? as u32);
            format!(" total, {}m left", remaining)
        } else {
            String::new()
        };

        cprintln!(
            c0,
            "[▪]({}) [{}]({}) [{}]({}) ([{}{}]({}))",
            c1,
            cursor.pretty(),
            c2,
            routine.name(),
            c3,
            routine.pretty_duration()?,
            duration_text,
            c2
        );

        for activity in activities {
            let color = if dist == 0 { "#788" } else { "#444" };
            cprintln!(color, "        {}", activity);
        }

        if dist == 0 {
            cprintln!("#FC1", "{}", "-".repeat(70));
        }

        cursor.add(routine.duration()? as u32);
    }

    Ok(())
}

fn load_plan() -> Result<Plan> {
    let app = App::new();
    let filename = app.find_data_filename()?;
    let text = std::fs::read_to_string(filename)?;
    let data: Plan = serde_yaml::from_str(&text)?;
    Ok(Plan::new(data))
}
