use briefcase::models::config::Frequency;
use briefcase::scheduler::service::SchedulerService;
use chrono::{Duration, Utc};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_service_creation() {
        let _service = SchedulerService;
        // Just check it exists
        assert!(true);
    }

    #[test]
    fn test_is_due_hourly() {
        let last_backup = Some(Utc::now() - Duration::hours(2));
        assert!(SchedulerService::is_backup_due(
            last_backup,
            Frequency::Hourly
        ));
    }

    #[test]
    fn test_is_due_hourly_recent() {
        let last_backup = Some(Utc::now() - Duration::minutes(30));
        assert!(!SchedulerService::is_backup_due(
            last_backup,
            Frequency::Hourly
        ));
    }

    #[test]
    fn test_is_due_daily() {
        let last_backup = Some(Utc::now() - Duration::hours(25));
        assert!(SchedulerService::is_backup_due(
            last_backup,
            Frequency::Daily
        ));
    }

    #[test]
    fn test_is_due_daily_recent() {
        let last_backup = Some(Utc::now() - Duration::hours(12));
        assert!(!SchedulerService::is_backup_due(
            last_backup,
            Frequency::Daily
        ));
    }

    #[test]
    fn test_is_due_weekly() {
        let last_backup = Some(Utc::now() - Duration::days(8));
        assert!(SchedulerService::is_backup_due(
            last_backup,
            Frequency::Weekly
        ));
    }

    #[test]
    fn test_is_due_weekly_recent() {
        let last_backup = Some(Utc::now() - Duration::days(3));
        assert!(!SchedulerService::is_backup_due(
            last_backup,
            Frequency::Weekly
        ));
    }

    #[test]
    fn test_is_due_no_last_backup() {
        let last_backup = None;
        assert!(SchedulerService::is_backup_due(
            last_backup,
            Frequency::Daily
        ));
    }
}
