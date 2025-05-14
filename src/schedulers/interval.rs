use super::*;

// Interval schedule
pub struct IntervalSchedule {
    interval: Duration,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
}

impl IntervalSchedule {
    pub fn new(interval: Duration, start_time: DateTime<Utc>) -> Result<Self, SchedulerError> {
        if interval.as_secs() == 0 {
            return Err(SchedulerError::InvalidDuration);
        }

        Ok(Self {
            interval,
            start_time,
            end_time: None,
        })
    }

    pub fn with_end_time(mut self, end_time: DateTime<Utc>) -> Self {
        self.end_time = Some(end_time);
        self
    }
}

impl Schedule for IntervalSchedule {
    fn next_occurrence(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        if after < self.start_time {
            return Some(self.start_time);
        }

        let since_start = after - self.start_time;
        let intervals_passed =
            (since_start.as_seconds_f32() / self.interval.as_secs() as f32) as u64;
        let next_time = self.start_time + self.interval * (intervals_passed + 1) as u32;

        match self.end_time {
            Some(end) if next_time > end => None,
            _ => Some(next_time),
        }
    }
}
