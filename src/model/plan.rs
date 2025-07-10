use crate::internal::*;
use chrono::{Datelike, Local, Timelike, Weekday};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub daily_routine: Vec<Routine>,
}

impl Plan {
    pub fn new(mut data: Plan) -> Self {
        // Normalize routines
        for routine in &mut data.daily_routine {
            *routine = Routine::new(routine.clone());
        }
        data
    }

    pub fn routines(&self) -> &[Routine] {
        &self.daily_routine
    }

    pub fn current_day_of_week(&self) -> DayOfWeek {
        let today = Local::now().weekday();
        match today {
            Weekday::Mon => DayOfWeek::Mon,
            Weekday::Tue => DayOfWeek::Tue,
            Weekday::Wed => DayOfWeek::Wed,
            Weekday::Thu => DayOfWeek::Thu,
            Weekday::Fri => DayOfWeek::Fri,
            Weekday::Sat => DayOfWeek::Sat,
            Weekday::Sun => DayOfWeek::Sun,
        }
    }

    pub fn current_day_of_week_pretty(&self) -> String {
        let day = self.current_day_of_week();
        match day {
            DayOfWeek::Mon => "Monday",
            DayOfWeek::Tue => "Tuesday",
            DayOfWeek::Wed => "Wednesday",
            DayOfWeek::Thu => "Thursday",
            DayOfWeek::Fri => "Friday",
            DayOfWeek::Sat => "Saturday",
            DayOfWeek::Sun => "Sunday",
            _ => "Unknown",
        }
        .to_string()
    }

    pub fn current_time_mins(&self) -> u32 {
        let now = Local::now();
        (now.hour() * 60 + now.minute()) as u32
    }

    pub fn current_time_pretty(&self) -> String {
        let now = Local::now();
        let mut hours = now.hour();
        let minutes = now.minute();
        let ampm = if hours >= 12 { "pm" } else { "am" };

        hours = hours % 12;
        if hours == 0 {
            hours = 12;
        }

        format!("{}:{:02} {}", hours, minutes, ampm)
    }

    pub fn current_activity_index(&self) -> Result<i32> {
        let current = self.current_time_mins();
        if self.daily_routine.is_empty() {
            return Ok(-1);
        }

        let first_start = self.daily_routine[0].start_minutes()?.unwrap_or(0);
        let mut cursor = TimeCursor::new(first_start);

        for (index, routine) in self.daily_routine.iter().enumerate() {
            if let Some(activity_start) = routine.start_minutes()? {
                cursor.set(activity_start);
            }

            if cursor.cursor > current {
                return Ok(index as i32 - 1);
            }

            cursor.add(routine.duration()? as u32);
        }

        Ok(-1)
    }
}
