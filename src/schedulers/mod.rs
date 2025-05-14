use super::*;

pub use self::{combined::*, cron::*, interval::*, one_time::*, random_interval::*};

mod combined;
mod cron;
mod interval;
mod one_time;
mod random_interval;

// Schedule Trait
pub trait Schedule {
    fn next_occurrence(&self, after: DateTime<Utc>) -> Option<DateTime<Utc>>;
}
