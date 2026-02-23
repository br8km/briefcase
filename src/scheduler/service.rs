use crate::models::config::Frequency;
use chrono::{DateTime, Duration, Utc};

pub struct SchedulerService;

impl SchedulerService {
    pub fn is_backup_due(last_backup: Option<DateTime<Utc>>, frequency: Frequency) -> bool {
        let Some(last) = last_backup else {
            return true; // Never backed up
        };

        let now = Utc::now();
        let duration_since_last = now.signed_duration_since(last);

        match frequency {
            Frequency::Hourly => duration_since_last >= Duration::hours(1),
            Frequency::Daily => duration_since_last >= Duration::days(1),
            Frequency::Weekly => duration_since_last >= Duration::weeks(1),
        }
    }
}
