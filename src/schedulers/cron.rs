use super::*;

// Cron-like schedule
#[derive(Default)]
pub struct CronSchedule {
    minute: Option<u32>,
    hour: Option<u32>,
    day: Option<u32>,
    month: Option<u32>,
    weekday: Option<u32>,
}

impl CronSchedule {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn minute(mut self, minute: u32) -> Result<Self, SchedulerError> {
        if minute >= 60 {
            return Err(SchedulerError::InvalidConfiguration);
        }
        self.minute = Some(minute);
        Ok(self)
    }

    pub fn hour(mut self, hour: u32) -> Result<Self, SchedulerError> {
        if hour >= 24 {
            return Err(SchedulerError::InvalidConfiguration);
        }
        self.hour = Some(hour);
        Ok(self)
    }

    pub fn day(mut self, day: u32) -> Result<Self, SchedulerError> {
        if day == 0 || day > 31 {
            return Err(SchedulerError::InvalidConfiguration);
        }
        self.day = Some(day);
        Ok(self)
    }

    pub fn month(mut self, month: u32) -> Result<Self, SchedulerError> {
        if month == 0 || month > 12 {
            return Err(SchedulerError::InvalidConfiguration);
        }
        self.month = Some(month);
        Ok(self)
    }

    pub fn weekday(mut self, weekday: u32) -> Result<Self, SchedulerError> {
        if weekday >= 7 {
            return Err(SchedulerError::InvalidConfiguration);
        }
        self.weekday = Some(weekday);
        Ok(self)
    }
}

impl Schedule for CronSchedule {
    fn next_occurrence(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let mut next = after;

        // Add 1 second to ensure we don't get the same time again
        next += Duration::from_secs(1);

        loop {
            // Check month
            if let Some(month) = self.month {
                match next.month().cmp(&month) {
                    std::cmp::Ordering::Less => {
                        next = next
                            .with_month(month)
                            .unwrap()
                            .with_day(1)
                            .unwrap()
                            .with_hour(0)
                            .unwrap()
                            .with_minute(0)
                            .unwrap()
                            .with_second(0)
                            .unwrap();
                        continue;
                    }
                    std::cmp::Ordering::Greater => {
                        next = next
                            .with_year(next.year() + 1)
                            .unwrap()
                            .with_month(1)
                            .unwrap()
                            .with_day(1)
                            .unwrap()
                            .with_hour(0)
                            .unwrap()
                            .with_minute(0)
                            .unwrap()
                            .with_second(0)
                            .unwrap();
                        continue;
                    }
                    std::cmp::Ordering::Equal => {}
                }
            }

            // Check day
            if let Some(day) = self.day {
                match next.day().cmp(&day) {
                    std::cmp::Ordering::Less => {
                        next = next
                            .with_day(day)
                            .unwrap()
                            .with_hour(0)
                            .unwrap()
                            .with_minute(0)
                            .unwrap()
                            .with_second(0)
                            .unwrap();
                        continue;
                    }
                    std::cmp::Ordering::Greater => {
                        next = next
                            .with_month(next.month() + 1)
                            .unwrap()
                            .with_day(1)
                            .unwrap()
                            .with_hour(0)
                            .unwrap()
                            .with_minute(0)
                            .unwrap()
                            .with_second(0)
                            .unwrap();
                        continue;
                    }
                    std::cmp::Ordering::Equal => {}
                }
            }

            // Check weekday
            if let Some(weekday) = self.weekday {
                if next.weekday().num_days_from_monday() != weekday {
                    next = next
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap()
                        + Duration::from_secs(86400);
                    continue;
                }
            }

            // Check hour
            if let Some(hour) = self.hour {
                match next.hour().cmp(&hour) {
                    std::cmp::Ordering::Less => {
                        next = next
                            .with_hour(hour)
                            .unwrap()
                            .with_minute(0)
                            .unwrap()
                            .with_second(0)
                            .unwrap();
                        continue;
                    }
                    std::cmp::Ordering::Greater => {
                        next = next
                            .with_day(next.day() + 1)
                            .unwrap()
                            .with_hour(0)
                            .unwrap()
                            .with_minute(0)
                            .unwrap()
                            .with_second(0)
                            .unwrap();
                        continue;
                    }
                    std::cmp::Ordering::Equal => {}
                }
            }

            // Check minute
            if let Some(minute) = self.minute {
                match next.minute().cmp(&minute) {
                    std::cmp::Ordering::Less => {
                        next = next.with_minute(minute).unwrap().with_second(0).unwrap();
                        continue;
                    }
                    std::cmp::Ordering::Greater => {
                        next = next
                            .with_hour(next.hour() + 1)
                            .unwrap()
                            .with_minute(0)
                            .unwrap()
                            .with_second(0)
                            .unwrap();
                        continue;
                    }
                    std::cmp::Ordering::Equal => {}
                }
            }

            // If we get here, all conditions are satisfied
            return Some(next);
        }
    }
}
