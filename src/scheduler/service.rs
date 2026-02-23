use crate::models::config::Frequency;
use chrono::{DateTime, Duration, Local};

pub struct SchedulerService;

impl SchedulerService {
    pub fn is_backup_due(last_backup: Option<DateTime<Local>>, frequency: Frequency) -> bool {
        let Some(last) = last_backup else {
            return true; // Never backed up
        };

        let now = Local::now();
        let duration_since_last = now.signed_duration_since(last);

        match frequency {
            Frequency::Hourly => duration_since_last >= Duration::hours(1),
            Frequency::Daily => duration_since_last >= Duration::days(1),
            Frequency::Weekly => duration_since_last >= Duration::weeks(1),
        }
    }

    pub fn next_backup_time(
        last_backup: Option<DateTime<Local>>,
        frequency: Frequency,
    ) -> DateTime<Local> {
        let base = last_backup.unwrap_or_else(Local::now);

        match frequency {
            Frequency::Hourly => base + Duration::hours(1),
            Frequency::Daily => base + Duration::days(1),
            Frequency::Weekly => base + Duration::weeks(1),
        }
    }
}
