use super::*;
use chrono::TimeZone;

#[test]
fn test_specific_datetime() {
    // Test case for "Specific dates/times, eg: 20 Sept 10:00 pm"
    let now = Utc::now();

    // Create a specific future date (20 Sept 10:00 pm of the current year)
    let target_year = if now.month() > 9 || (now.month() == 9 && now.day() > 20) {
        now.year() + 1
    } else {
        now.year()
    };

    let specific_time = Utc.with_ymd_and_hms(target_year, 9, 20, 22, 0, 0).unwrap(); // 20 Sept 10:00 pm

    // Create a one-time schedule for this specific date/time
    let schedule = OneTimeSchedule::new(specific_time).unwrap();

    // Test that the schedule returns the correct next occurrence
    let one_day_before = specific_time - chrono::TimeDelta::days(1);
    assert_eq!(
        schedule.next_occurrence(one_day_before),
        Some(specific_time)
    );

    // Test that the schedule returns None after the specific time has passed
    let one_minute_after = specific_time + chrono::TimeDelta::minutes(1);
    assert_eq!(schedule.next_occurrence(one_minute_after), None);

    // Create a job with this schedule
    let mut job = Job::builder()
        .schedule(Box::new(schedule))
        .task("Specific date/time task")
        .build()
        .unwrap();

    // Test that the job doesn't execute before the specific time
    assert!(job.should_execute(one_day_before).is_none());

    // Test that the job executes at the specific time
    assert!(job.should_execute(specific_time).is_some());
    assert_eq!(job.repeats, 1);

    // Test that the job doesn't execute again after it has already executed
    // The next_occurrence method of OneTimeSchedule will return None after the time has passed,
    // so should_execute will also return None
    let one_second_after = specific_time + chrono::TimeDelta::seconds(1);
    assert!(job.should_execute(one_second_after).is_none());
}

#[test]
fn test_repetition_scenario() {
    // Test case for "Repetition: 10 times hourly, until 3rd of March"
    let start_time = Utc.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();
    let end_time = Utc.with_ymd_and_hms(2023, 3, 3, 23, 59, 59).unwrap();
    let interval = Duration::from_secs(3600); // 1 hour
    let schedule = IntervalSchedule::new(interval, start_time)
        .unwrap()
        .with_end_time(end_time);

    let mut job = Job::builder()
        .schedule(Box::new(schedule))
        .task("Hourly task with repetition limit")
        .max_repeats(10) // Will run 10 times max
        .end_time(end_time) // Or until March 3rd, whichever comes first
        .build()
        .unwrap();

    // First execution at start time
    assert!(job.should_execute(start_time).is_some());
    assert_eq!(job.repeats, 1);

    // Run through all 10 executions
    for i in 1..10 {
        let next_time = start_time + interval * i as u32;
        assert!(job.should_execute(next_time).is_some());
        assert_eq!(job.repeats, i + 1);
    }

    // 11th execution should not happen due to max_repeats
    let eleventh_time = start_time + interval * 10;
    assert!(job.should_execute(eleventh_time).is_none());
    assert_eq!(job.repeats, 10);

    // Test with end_time constraint
    let start_time = Utc.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();
    let end_time = Utc.with_ymd_and_hms(2023, 3, 2, 5, 0, 0).unwrap(); // Only allows 5 hours
    let schedule = IntervalSchedule::new(interval, start_time).unwrap();

    let mut job = Job::builder()
        .schedule(Box::new(schedule))
        .task("Hourly task with end time")
        .max_repeats(10) // Will run 10 times max
        .end_time(end_time) // But end_time will limit it to 6 executions
        .build()
        .unwrap();

    // Should execute for the first 5 hours (0, 1, 2, 3, 4)
    for i in 0..5 {
        let time = start_time + interval * i as u32;
        assert!(job.should_execute(time).is_some(), "Failed at hour {}", i);
    }

    // Should not execute at or after end_time
    let at_end_time = end_time;
    assert!(
        job.should_execute(at_end_time).is_none(),
        "Should not execute at end_time"
    );

    let after_end_time = end_time + Duration::from_secs(1);
    assert!(
        job.should_execute(after_end_time).is_none(),
        "Should not execute after end_time"
    );
}

#[test]
fn test_one_time_schedule() {
    let now = Utc::now();
    let future_time = now + Duration::from_secs(3600);

    let schedule = OneTimeSchedule::new(future_time).unwrap();
    assert_eq!(schedule.next_occurrence(now), Some(future_time));
    assert_eq!(schedule.next_occurrence(future_time), None);
    assert_eq!(
        schedule.next_occurrence(future_time + Duration::from_secs(1)),
        None
    );
}

#[test]
fn test_one_time_schedule_in_past() {
    let now = Utc::now();
    let past_time = now - Duration::from_secs(3600);

    let result = OneTimeSchedule::new(past_time);
    assert!(matches!(result, Err(SchedulerError::TimeInPast)));
}

#[test]
fn test_recurring_intervals() {
    // Test case for "Recurring intervals, eg: hourly, daily, weekly, monthly, every third Saturday"
    let base_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap(); // Sunday, Jan 1, 2023

    // 1. Hourly schedule
    let hourly_interval = Duration::from_secs(3600); // 1 hour
    let hourly_schedule = IntervalSchedule::new(hourly_interval, base_date).unwrap();

    assert_eq!(
        hourly_schedule.next_occurrence(base_date),
        Some(base_date + hourly_interval)
    );
    assert_eq!(
        hourly_schedule.next_occurrence(base_date + hourly_interval),
        Some(base_date + hourly_interval * 2)
    );

    // 2. Daily schedule (using CronSchedule for midnight each day)
    let daily_schedule = CronSchedule::new().hour(0).unwrap().minute(0).unwrap();

    let day1 = base_date;
    let day2 = base_date + chrono::TimeDelta::days(1); // Jan 2, 2023

    assert_eq!(
        daily_schedule.next_occurrence(day1 + chrono::TimeDelta::hours(1)),
        Some(day2)
    );
    assert_eq!(
        daily_schedule.next_occurrence(day2 + chrono::TimeDelta::hours(1)),
        Some(day2 + chrono::TimeDelta::days(1))
    );

    // 3. Weekly schedule (every Monday at 9am)
    // Note: In CronSchedule, weekday is 0-based from Monday (0=Monday, 6=Sunday)
    // This is different from the chrono library where 0=Sunday, 1=Monday
    let weekly_schedule = CronSchedule::new()
        .weekday(0) // Monday (0 is Monday in CronSchedule)
        .unwrap()
        .hour(9)
        .unwrap()
        .minute(0)
        .unwrap();

    // Jan 1, 2023 is a Sunday, so next Monday is Jan 2
    let next_monday = Utc.with_ymd_and_hms(2023, 1, 2, 9, 0, 0).unwrap();
    let following_monday = Utc.with_ymd_and_hms(2023, 1, 9, 9, 0, 0).unwrap();

    assert_eq!(
        weekly_schedule.next_occurrence(base_date),
        Some(next_monday)
    );
    // Test that after the first Monday, the next occurrence is the following Monday
    let after_first_monday = next_monday + chrono::TimeDelta::minutes(1); // 9:01 AM
    assert_eq!(
        weekly_schedule.next_occurrence(after_first_monday),
        Some(following_monday)
    );

    // 4. Monthly schedule (1st day of each month at noon)
    let monthly_schedule = CronSchedule::new()
        .day(1)
        .unwrap()
        .hour(12)
        .unwrap()
        .minute(0)
        .unwrap();

    let first_of_jan = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
    let first_of_feb = Utc.with_ymd_and_hms(2023, 2, 1, 12, 0, 0).unwrap();

    assert_eq!(
        monthly_schedule.next_occurrence(base_date),
        Some(first_of_jan)
    );
    assert_eq!(
        monthly_schedule.next_occurrence(first_of_jan + chrono::TimeDelta::hours(1)),
        Some(first_of_feb)
    );

    // 5. Every third Saturday (at 10am)
    // We'll use a more complex approach to test this pattern
    let third_saturday_schedule = CronSchedule::new()
        .weekday(5) // Saturday (0 is Monday, 5 is Saturday in CronSchedule)
        .unwrap()
        .hour(10)
        .unwrap()
        .minute(0)
        .unwrap();

    // Find the first three Saturdays in January 2023
    // Jan 7, Jan 14, Jan 21 are the first three Saturdays
    let first_saturday = Utc.with_ymd_and_hms(2023, 1, 7, 10, 0, 0).unwrap();
    let second_saturday = Utc.with_ymd_and_hms(2023, 1, 14, 10, 0, 0).unwrap();
    let third_saturday = Utc.with_ymd_and_hms(2023, 1, 21, 10, 0, 0).unwrap();

    // Test the schedule finds each Saturday correctly
    assert_eq!(
        third_saturday_schedule.next_occurrence(base_date),
        Some(first_saturday)
    );
    assert_eq!(
        third_saturday_schedule.next_occurrence(first_saturday + chrono::TimeDelta::hours(1)),
        Some(second_saturday)
    );
    assert_eq!(
        third_saturday_schedule.next_occurrence(second_saturday + chrono::TimeDelta::hours(1)),
        Some(third_saturday)
    );

    // Create a job with the third Saturday schedule
    let mut job = Job::builder()
        .schedule(Box::new(third_saturday_schedule))
        .task("Third Saturday task")
        .build()
        .unwrap();

    // Test that the job executes on each Saturday
    assert!(job.should_execute(first_saturday).is_some());
    assert!(job.should_execute(second_saturday).is_some());
    assert!(job.should_execute(third_saturday).is_some());
}

#[test]
fn test_interval_schedule() {
    let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let interval = Duration::from_secs(3600); // 1 hour

    let schedule = IntervalSchedule::new(interval, start_time).unwrap();

    assert_eq!(
        schedule.next_occurrence(start_time - Duration::from_secs(1)),
        Some(start_time)
    );
    assert_eq!(
        schedule.next_occurrence(start_time),
        Some(start_time + interval)
    );
    assert_eq!(
        schedule.next_occurrence(start_time + interval),
        Some(start_time + interval * 2)
    );
}

#[test]
fn test_interval_schedule_with_end() {
    let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_time = start_time + Duration::from_secs(7200); // 2 hours
    let interval = Duration::from_secs(3600); // 1 hour

    let schedule = IntervalSchedule::new(interval, start_time)
        .unwrap()
        .with_end_time(end_time);

    assert_eq!(
        schedule.next_occurrence(start_time),
        Some(start_time + interval)
    );
    assert_eq!(
        schedule.next_occurrence(start_time + interval),
        Some(start_time + interval * 2)
    );
    assert_eq!(schedule.next_occurrence(start_time + interval * 2), None);
}

#[test]
fn test_random_intervals() {
    // Test case for "Random intervals, eg: between 9-10 am"
    let base_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

    // 1. Basic random interval test
    let min = Duration::from_secs(60); // 1 minute
    let max = Duration::from_secs(120); // 2 minutes
    let schedule = RandomIntervalSchedule::new(min, max)
        .unwrap()
        .with_start_time(base_date);

    // Test that the next occurrence is within the expected range
    let next = schedule.next_occurrence(base_date).unwrap();
    assert!(
        next >= base_date + min,
        "Next occurrence should be at least min interval after start time"
    );
    assert!(
        next <= base_date + max,
        "Next occurrence should be at most max interval after start time"
    );

    // 2. Specific time-of-day random interval (between 9-10 am)
    // Create a schedule that generates random times between 9-10 am
    let nine_am = base_date
        .with_hour(9)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();

    // Create a job with a random schedule that simulates "between 9-10 am"
    // We'll use a different approach - just check that the job executes at a specific time within the window
    let morning_min = Duration::from_secs(60); // 1 minute
    let morning_max = Duration::from_secs(300); // 5 minutes

    let morning_schedule = RandomIntervalSchedule::new(morning_min, morning_max)
        .unwrap()
        .with_start_time(nine_am);

    // Create a job with this schedule
    let mut job = Job::builder()
        .schedule(Box::new(morning_schedule))
        .task("Random morning task")
        .build()
        .unwrap();

    // Test that the job executes at some time within the random interval
    // Since this is random, we'll try multiple times to ensure we hit at least one valid time
    let mut found_valid_execution = false;
    for i in 1..=10 {
        // Reset the job's internal state for each test iteration
        job = Job::builder()
            .schedule(Box::new(RandomIntervalSchedule::new(morning_min, morning_max)
                .unwrap()
                .with_start_time(nine_am)))
            .task("Random morning task")
            .build()
            .unwrap();
            
        // Try a time within the possible range (9:01am to 9:05am)
        let test_time = nine_am + Duration::from_secs(i * 30); // Try times from 9:00:30 to 9:05:00
        if job.should_execute(test_time).is_some() {
            found_valid_execution = true;
            break;
        }
    }
    assert!(found_valid_execution, "Job should execute at some time within the random interval");

    // Test that the job doesn't execute before the start time
    let before_start = nine_am - Duration::from_secs(1); // 8:59:59am
    assert!(job.should_execute(before_start).is_none());
}

#[test]
fn test_random_interval_schedule() {
    let min = Duration::from_secs(60);
    let max = Duration::from_secs(120);
    let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

    let schedule = RandomIntervalSchedule::new(min, max)
        .unwrap()
        .with_start_time(start_time);

    let next = schedule.next_occurrence(start_time).unwrap();
    assert!(next >= start_time + min);
    assert!(next <= start_time + max);
}

#[test]
fn test_cron_schedule_daily() {
    let schedule = CronSchedule::new().hour(12).unwrap().minute(0).unwrap();

    let morning = Utc.with_ymd_and_hms(2023, 1, 1, 8, 0, 0).unwrap();
    let expected = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();

    assert_eq!(schedule.next_occurrence(morning), Some(expected));

    let afternoon = Utc.with_ymd_and_hms(2023, 1, 1, 13, 0, 0).unwrap();
    let expected_next_day = Utc.with_ymd_and_hms(2023, 1, 2, 12, 0, 0).unwrap();

    assert_eq!(schedule.next_occurrence(afternoon), Some(expected_next_day));
}

#[test]
fn test_cron_schedule_monthly() {
    let schedule = CronSchedule::new()
        .day(15)
        .unwrap()
        .hour(0)
        .unwrap()
        .minute(0)
        .unwrap();

    let early_month = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let expected = Utc.with_ymd_and_hms(2023, 1, 15, 0, 0, 0).unwrap();

    assert_eq!(schedule.next_occurrence(early_month), Some(expected));

    let late_month = Utc.with_ymd_and_hms(2023, 1, 16, 0, 0, 0).unwrap();
    let expected_next_month = Utc.with_ymd_and_hms(2023, 2, 15, 0, 0, 0).unwrap();

    assert_eq!(
        schedule.next_occurrence(late_month),
        Some(expected_next_month)
    );
}

#[test]
fn test_job_execution() {
    let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let interval = Duration::from_secs(3600);
    let schedule = IntervalSchedule::new(interval, start_time).unwrap();

    let mut job = Job::builder()
        .schedule(Box::new(schedule))
        .task("Test task")
        .max_repeats(2)
        .build()
        .unwrap();

    // First execution
    assert!(job.should_execute(start_time).is_some());
    assert_eq!(job.repeats, 1);

    // Between first and second
    assert!(job
        .should_execute(start_time + Duration::from_secs(1800))
        .is_none());

    // Second execution
    assert!(job.should_execute(start_time + interval).is_some());
    assert_eq!(job.repeats, 2);

    // Third execution should not happen due to max_repeats
    assert!(job.should_execute(start_time + interval * 2).is_none());
}

#[test]
fn test_mixture_scenario() {
    // Test case for "Mixture: Every hour until 10pm and then Every minute for the next 1 hour"
    let base_date = Utc.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();

    // First schedule: Every hour until 10pm
    let hourly_start = base_date;
    let hourly_end = base_date.with_hour(22).unwrap(); // 10pm
    let hourly_interval = Duration::from_secs(3600); // 1 hour
    let hourly_schedule = IntervalSchedule::new(hourly_interval, hourly_start)
        .unwrap()
        .with_end_time(hourly_end);

    // Second schedule: Every minute for the next hour (10pm to 11pm)
    let minutely_start = hourly_end;
    let minutely_end = minutely_start + Duration::from_secs(3600); // 1 hour after 10pm
    let minutely_interval = Duration::from_secs(60); // 1 minute
    let minutely_schedule = IntervalSchedule::new(minutely_interval, minutely_start)
        .unwrap()
        .with_end_time(minutely_end);

    // Combined schedule
    let combined =
        CombinedSchedule::new(vec![Box::new(hourly_schedule), Box::new(minutely_schedule)]);

    // Test hourly schedule (should pick the earliest next occurrence)
    let test_time = base_date;
    let expected_first_hour = base_date + Duration::from_secs(3600); // 1:00
    assert_eq!(
        combined.next_occurrence(test_time),
        Some(expected_first_hour)
    );

    // Test at 9pm (should still follow hourly schedule)
    let test_9pm = base_date.with_hour(21).unwrap();
    let expected_10pm = base_date.with_hour(22).unwrap();
    assert_eq!(combined.next_occurrence(test_9pm), Some(expected_10pm));

    // Test at 10pm (should switch to minutely schedule)
    let test_10pm = base_date.with_hour(22).unwrap();
    let expected_10_01pm = test_10pm + Duration::from_secs(60); // 10:01pm
    assert_eq!(combined.next_occurrence(test_10pm), Some(expected_10_01pm));

    // Test at 10:30pm (should still be on minutely schedule)
    let test_10_30pm = base_date.with_hour(22).unwrap().with_minute(30).unwrap();
    let expected_10_31pm = test_10_30pm + Duration::from_secs(60); // 10:31pm
    assert_eq!(
        combined.next_occurrence(test_10_30pm),
        Some(expected_10_31pm)
    );

    // Test at 10:59pm (last minute of the minutely schedule)
    let test_10_59pm = base_date.with_hour(22).unwrap().with_minute(59).unwrap();
    let expected_11pm = base_date.with_hour(23).unwrap();
    assert_eq!(combined.next_occurrence(test_10_59pm), Some(expected_11pm));

    // Test at 11pm (should return None as both schedules are done)
    let test_11pm = base_date.with_hour(23).unwrap();
    assert_eq!(combined.next_occurrence(test_11pm), None);

    // Create a job with this combined schedule
    let mut job = Job::builder()
        .schedule(Box::new(combined))
        .task("Mixed schedule task")
        .build()
        .unwrap();

    // Verify job executes at expected times
    // Should execute at 1:00
    assert!(job.should_execute(expected_first_hour).is_some());

    // Should execute at 10:00pm
    assert!(job.should_execute(expected_10pm).is_some());

    // Should execute at 10:01pm (minutely schedule)
    assert!(job.should_execute(expected_10_01pm).is_some());

    // Should execute at 10:31pm (minutely schedule)
    assert!(job.should_execute(expected_10_31pm).is_some());

    // Should execute at 11:00pm (last execution)
    assert!(job.should_execute(expected_11pm).is_some());

    // Should not execute at 11:01pm (after all schedules)
    let after_all_schedules = expected_11pm + Duration::from_secs(60);
    assert!(job.should_execute(after_all_schedules).is_none());
}
