use crate::internal::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Routine {
    pub name: String,
    pub start: Option<String>,
    pub duration: String,
    pub activities: HashMap<DayOfWeek, Vec<String>>,
}

impl Routine {
    pub fn new(mut data: Routine) -> Self {
        // Normalize the start time
        if let Some(ref mut start) = data.start {
            *start = start.trim().to_lowercase();
        }
        data
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // Duration of the routine in minutes
    pub fn duration(&self) -> Result<f64> {
        let re = Regex::new(r"^(\d+(?:\.\d+)?)\s*(m|h)$")?;
        let caps = re
            .captures(&self.duration)
            .ok_or_else(|| anyhow!("Invalid duration format: {}", self.duration))?;

        let value: f64 = caps[1].parse()?;
        let unit = &caps[2];

        Ok(match unit {
            "h" => value * 60.0,
            "m" => value,
            _ => return Err(anyhow!("Invalid duration unit: {}", unit)),
        })
    }

    pub fn start_minutes(&self) -> Result<Option<u32>> {
        let Some(ref start) = self.start else {
            return Ok(None);
        };

        let start = start.trim().to_lowercase();
        let re = Regex::new(r"^(\d{1,2}):?(\d{2})?\s*(am|pm)?$")?;
        let caps = re.captures(&start).ok_or_else(|| {
            anyhow!(
                "Invalid start time format: {}",
                self.start.as_ref().unwrap()
            )
        })?;

        let mut hour: u32 = caps[1].parse()?;
        let minute: u32 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));

        if let Some(ampm) = caps.get(3) {
            match ampm.as_str() {
                "pm" if hour < 12 => hour += 12,
                "am" if hour == 12 => hour = 0,
                _ => {}
            }
        }

        Ok(Some(hour * 60 + minute))
    }

    pub fn pretty_duration(&self) -> Result<String> {
        let duration = self.duration()?;
        if duration <= 90.0 {
            Ok(format!("{}m", duration as u32))
        } else {
            let hours = duration / 60.0;
            let formatted = if hours.fract() == 0.0 {
                format!("{}h", hours as u32)
            } else {
                format!("{:.1}h", hours)
            };
            Ok(formatted)
        }
    }

    pub fn activities(&self, day: DayOfWeek) -> Vec<String> {
        let mut activities = Vec::new();

        // Add everyday activities
        if let Some(everyday) = self.activities.get(&DayOfWeek::Everyday) {
            activities.extend_from_slice(everyday);
        }

        // Add weekend activities if applicable
        if matches!(day, DayOfWeek::Sat | DayOfWeek::Sun) {
            if let Some(weekends) = self.activities.get(&DayOfWeek::Weekends) {
                activities.extend_from_slice(weekends);
            }
        }

        // Add weekday activities if applicable
        if matches!(
            day,
            DayOfWeek::Mon | DayOfWeek::Tue | DayOfWeek::Wed | DayOfWeek::Thu | DayOfWeek::Fri
        ) {
            if let Some(weekdays) = self.activities.get(&DayOfWeek::Weekdays) {
                activities.extend_from_slice(weekdays);
            }
        }

        // Add specific day activities or default
        if let Some(day_activities) = self.activities.get(&day) {
            activities.extend_from_slice(day_activities);
        } else if let Some(default) = self.activities.get(&DayOfWeek::Default) {
            activities.extend_from_slice(default);
        }

        activities
    }
}
