# Task Scheduler Library

A flexible and ergonomic job scheduling library for Rust that enables scheduling tasks based on various time-based criteria.

## Features

- **Multiple scheduling options**:
  - One-time schedules at specific dates and times
  - Recurring intervals (hourly, daily, weekly, monthly)
  - Cron-style schedules
  - Random intervals within specified ranges
  - Custom schedules
- **Flexible job configuration**:
  - Maximum repeat limits
  - End time constraints
  - Task prioritization
- **Intuitive builder pattern API**
- **Comprehensive test coverage**

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (1.70.0 or later)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

## Installation

### Installing Rust and Cargo

If you don't have Rust installed, you can install it using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation. This will install both Rust and Cargo.

### Adding the Library to Your Project

To use this library in your Rust project, add it to your `Cargo.toml` dependencies:

```toml
[dependencies]
task-scheduler = { git = "https://github.com/yourusername/task-scheduler.git" }
```

Or if you're working directly with the source code:

```bash
git clone https://github.com/yourusername/task-scheduler.git
cd task-scheduler
```

## Usage

Here's a simple example of how to use the library:

```rust
use task_scheduler::{Job, schedulers::SpecificDateTimeSchedule};
use chrono::{Utc, TimeZone};

// Create a one-time schedule for a specific date and time
let specific_time = Utc.with_ymd_and_hms(2025, 5, 15, 10, 0, 0).unwrap();
let schedule = SpecificDateTimeSchedule::new(specific_time);

// Create a job with this schedule
let mut job = Job::builder()
    .schedule(Box::new(schedule))
    .task("Important task")
    .build()
    .unwrap();

// Check if the job should execute at the current time
let current_time = Utc::now();
if let Some(task) = job.should_execute(current_time) {
    println!("Executing task: {}", task);
}
```

## Running Tests

This library includes comprehensive tests for all scheduling functionality. To run the tests:

```bash
cargo test
```

To run a specific test:

```bash
cargo test tests::test_name
```

For example, to run the random intervals test:

```bash
cargo test tests::test_random_intervals
```

To see test output (including println! statements):

```bash
cargo test -- --nocapture
```

To run tests with a backtrace for debugging:

```bash
RUST_BACKTRACE=1 cargo test
```

## Test Coverage

The library includes tests for various scheduling scenarios:

- One-time schedules
- Recurring intervals (hourly, daily, weekly, monthly)
- Cron-style schedules
- Random interval schedules
- Job execution limits
- Mixed scheduling scenarios

## Project Structure

```
src/
├── lib.rs              # Main library entry point and Job implementation
├── error.rs            # Error types for the library
├── schedulers/         # Different scheduler implementations
│   ├── mod.rs          # Scheduler module exports
│   ├── cron.rs         # Cron-style schedules
│   ├── interval.rs     # Regular interval schedules
│   ├── one_time.rs     # One-time schedules
│   ├── random_interval.rs # Random interval schedules
│   └── recurring.rs    # Recurring schedules (daily, weekly, etc.)
└── tests.rs           # Comprehensive test suite
```

## Troubleshooting

### Random Test Failures

If you encounter intermittent failures in tests involving random intervals (like `test_random_intervals`), this is expected behavior due to the random nature of these schedules. The tests have been designed to be robust against this randomness, but occasionally they might still fail.

Solution: Run the specific test again or with multiple iterations:

```bash
cargo test tests::test_random_intervals -- --test-threads=1 --nocapture
```

### Timezone Issues

This library uses UTC for all time calculations. If you're experiencing unexpected behavior with schedules, ensure your local times are properly converted to UTC.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
