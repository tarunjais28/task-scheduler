use super::*;

// Scheduler Error
#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("Invalid schedule configuration")]
    InvalidConfiguration,

    #[error("Time has already passed")]
    TimeInPast,

    #[error("Duration is zero or negative")]
    InvalidDuration,

    #[error("Invalid repetition count")]
    InvalidRepetition,

    #[error("Invalid date/time specification")]
    InvalidDateTime,
}
