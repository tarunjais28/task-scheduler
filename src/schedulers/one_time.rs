use super::*;

// Specific date/time schedule
pub struct OneTimeSchedule {
    time: DateTime<Utc>,
}

impl OneTimeSchedule {
    pub fn new(time: DateTime<Utc>) -> Result<Self, SchedulerError> {
        if time <= Utc::now() {
            return Err(SchedulerError::TimeInPast);
        }
        Ok(Self { time })
    }
}

impl Schedule for OneTimeSchedule {
    fn next_occurrence(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        if after < self.time {
            Some(self.time)
        } else {
            None
        }
    }
}
