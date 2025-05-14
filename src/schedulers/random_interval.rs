use super::*;

// Random interval schedule
pub struct RandomIntervalSchedule {
    min_interval: Duration,
    max_interval: Duration,
    last_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
}

impl RandomIntervalSchedule {
    pub fn new(min_interval: Duration, max_interval: Duration) -> Result<Self, SchedulerError> {
        if min_interval.as_secs() == 0 || max_interval.as_secs() == 0 {
            return Err(SchedulerError::InvalidDuration);
        }
        if min_interval > max_interval {
            return Err(SchedulerError::InvalidConfiguration);
        }
        Ok(Self {
            min_interval,
            max_interval,
            last_time: None,
            end_time: None,
        })
    }

    pub fn with_end_time(mut self, end_time: DateTime<Utc>) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn with_start_time(mut self, start_time: DateTime<Utc>) -> Self {
        self.last_time = Some(start_time);
        self
    }

    fn generate_random_interval(&self) -> Duration {
        let mut rng = rand::rng();
        let secs = rng.random_range(self.min_interval.as_secs()..=self.max_interval.as_secs());
        Duration::from_secs(secs)
    }
}

impl Schedule for RandomIntervalSchedule {
    fn next_occurrence(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let last_time = self.last_time.unwrap_or(after);
        let next_time = last_time + self.generate_random_interval();

        match self.end_time {
            Some(end) if next_time > end => None,
            _ => Some(next_time),
        }
    }
}
