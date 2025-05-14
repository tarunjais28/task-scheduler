use super::*;

// Combined schedule
pub struct CombinedSchedule {
    schedules: Vec<Box<dyn Schedule>>,
}

impl CombinedSchedule {
    pub fn new(schedules: Vec<Box<dyn Schedule>>) -> Self {
        Self { schedules }
    }
}

impl Schedule for CombinedSchedule {
    fn next_occurrence(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>> {
        let mut earliest: Option<DateTime<Utc>> = None;

        for schedule in &self.schedules {
            if let Some(next) = schedule.next_occurrence(after) {
                match earliest {
                    None => earliest = Some(next),
                    Some(current_earliest) => {
                        if next < current_earliest {
                            earliest = Some(next);
                        }
                    }
                }
            }
        }

        earliest
    }
}
