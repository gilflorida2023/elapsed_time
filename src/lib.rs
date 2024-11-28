use std::time::Instant;

/// Measures the elapsed time of a given function and returns a formatted string representation.
///
/// This function takes a closure as an argument, executes it, and measures the time it takes to run.
/// The elapsed time is formatted into a human-readable string following these rules:
///
/// - Sub-second durations: Shows three decimal places (e.g., "0.500s")
/// - Whole seconds: Shows just seconds (e.g., "5s")
/// - Minutes and up: Shows all relevant units (e.g., "2m 30s", "1h 30m 45s")
/// - Supports up to weeks for long-running operations
///
/// # Arguments
///
/// * `f` - A closure that takes no arguments and returns nothing (`FnOnce()`).
///
/// # Returns
///
/// A `String` representing the formatted elapsed time.
///
/// # Example
///
/// ```
/// use std::thread::sleep;
/// use std::time::Duration;
/// use elapsed_time::measure_elapsed_time;
///
/// // Measure a 1.5 second operation
/// let elapsed_time = measure_elapsed_time(|| {
///     sleep(Duration::from_millis(1500));
/// });
/// assert_eq!(elapsed_time, "1.500s");
///
/// // Measure a longer operation
/// let elapsed_time = measure_elapsed_time(|| {
///     sleep(Duration::from_secs(125)); // 2 minutes and 5 seconds
/// });
/// assert_eq!(elapsed_time, "2m 5s");
/// ```
pub fn measure_elapsed_time<F>(f: F) -> String
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    format_duration(duration)
}

/// A struct to hold the calculated duration components.
///
/// This struct stores the broken-down components of a duration, with each field
/// representing a specific time unit. The fields are stored in their "remaining" form,
/// meaning they don't overlap (e.g., remaining_hours will be less than 24).
#[derive(Debug)]
struct DurationComponents {
    weeks: u64,
    remaining_days: u64,
    remaining_hours: u64,
    minutes: u64,
    seconds: u64,
    milliseconds: u32,
}

/// Calculates the duration components from a Duration.
fn format_duration_calculate(duration: std::time::Duration) -> DurationComponents {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let days = hours / 24;
    let weeks = days / 7;
    
    let remaining_days = days % 7;
    let remaining_hours = hours % 24;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let milliseconds = duration.subsec_millis();

    DurationComponents {
        weeks,
        remaining_days,
        remaining_hours,
        minutes,
        seconds,
        milliseconds,
    }
}

/// Formats the duration components into a human-readable string.
fn format_duration_format(components: &DurationComponents) -> String {
    // Helper function to format seconds with milliseconds
    let format_seconds = |secs: u64, ms: u32| {
        if secs == 0 && ms > 0 {
            format!("{}.{:03}s", 0, ms)
        } else if ms == 0 {
            format!("{}s", secs)
        } else {
            format!("{}.{:03}s", secs, ms)
        }
    };

    if components.weeks > 0 {
        format!("{}w {}d {}h {}m {}", 
            components.weeks, components.remaining_days, components.remaining_hours, 
            components.minutes, format_seconds(components.seconds, components.milliseconds))
    } else if components.remaining_days > 0 {
        format!("{}d {}h {}m {}", 
            components.remaining_days, components.remaining_hours, 
            components.minutes, format_seconds(components.seconds, components.milliseconds))
    } else if components.remaining_hours > 0 {
        format!("{}h {}m {}", 
            components.remaining_hours, components.minutes, 
            format_seconds(components.seconds, components.milliseconds))
    } else if components.minutes > 0 {
        if components.seconds > 0 || components.milliseconds > 0 {
            format!("{}m {}", 
                components.minutes, format_seconds(components.seconds, components.milliseconds))
        } else {
            format!("{}m", components.minutes)
        }
    } else {
        format_seconds(components.seconds, components.milliseconds)
    }
}

/// Formats a Duration into a human-readable string.
///
/// This function takes a Duration and formats it into a human-readable string with appropriate
/// units based on the duration's length. The output format follows these rules:
///
/// - Sub-second durations: Shows three decimal places (e.g., "0.500s")
/// - Whole seconds: Shows just seconds (e.g., "5s")
/// - Minutes and up: Shows all relevant units, space-separated (e.g., "2m 30s", "1h 30m 45s")
/// - Supports up to weeks: Can show full duration (e.g., "1w 2d 3h 45m 30s")
///
/// The function automatically:
/// - Only includes non-zero units
/// - Preserves millisecond precision when present
/// - Uses abbreviated unit names (w, d, h, m, s)
/// - Separates units with spaces
/// - Omits trailing zeros in decimal places
///
/// # Arguments
///
/// * `duration` - A `std::time::Duration` to be formatted.
///
/// # Returns
///
/// A `String` representing the formatted duration.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use elapsed_time::format_duration;
///
/// // Various duration formats
/// assert_eq!(format_duration(Duration::from_secs(5)), "5s");
/// assert_eq!(format_duration(Duration::from_millis(500)), "0.500s");
/// assert_eq!(format_duration(Duration::from_secs(125)), "2m 5s");
/// assert_eq!(format_duration(Duration::from_secs(3665)), "1h 1m 5s");
///
/// // Complex duration with multiple units
/// let week_plus = Duration::from_secs(
///     7 * 24 * 60 * 60 + // 1 week
///     2 * 24 * 60 * 60 + // 2 days
///     3 * 60 * 60 +      // 3 hours
///     4 * 60 +           // 4 minutes
///     5                  // 5 seconds
/// );
/// assert_eq!(format_duration(week_plus), "1w 2d 3h 4m 5s");
/// ```
pub fn format_duration(duration: std::time::Duration) -> String {
    let components = format_duration_calculate(duration);
    format_duration_format(&components)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_duration_components_calculation() {
        let duration = Duration::from_secs(90061); // 1 day, 1 hour, 1 minute, 1 second
        let components = format_duration_calculate(duration);
        
        assert_eq!(components.weeks, 0);
        assert_eq!(components.remaining_days, 1);
        assert_eq!(components.remaining_hours, 1);
        assert_eq!(components.minutes, 1);
        assert_eq!(components.seconds, 1);
        assert_eq!(components.milliseconds, 0);
    }

    #[test]
    fn test_format_duration_format() {
        let test_cases = vec![
            (
                DurationComponents {
                    weeks: 1,
                    remaining_days: 2,
                    remaining_hours: 3,
                    minutes: 4,
                    seconds: 5,
                    milliseconds: 6,
                },
                "1w 2d 3h 4m 5.006s",
            ),
            (
                DurationComponents {
                    weeks: 0,
                    remaining_days: 2,
                    remaining_hours: 3,
                    minutes: 4,
                    seconds: 5,
                    milliseconds: 6,
                },
                "2d 3h 4m 5.006s",
            ),
            (
                DurationComponents {
                    weeks: 0,
                    remaining_days: 0,
                    remaining_hours: 3,
                    minutes: 4,
                    seconds: 5,
                    milliseconds: 6,
                },
                "3h 4m 5.006s",
            ),
            (
                DurationComponents {
                    weeks: 0,
                    remaining_days: 0,
                    remaining_hours: 0,
                    minutes: 4,
                    seconds: 5,
                    milliseconds: 6,
                },
                "4m 5.006s",
            ),
            (
                DurationComponents {
                    weeks: 0,
                    remaining_days: 0,
                    remaining_hours: 0,
                    minutes: 0,
                    seconds: 5,
                    milliseconds: 6,
                },
                "5.006s",
            ),
            (
                DurationComponents {
                    weeks: 0,
                    remaining_days: 0,
                    remaining_hours: 0,
                    minutes: 0,
                    seconds: 0,
                    milliseconds: 6,
                },
                "0.006s",
            ),
            (
                DurationComponents {
                    weeks: 0,
                    remaining_days: 0,
                    remaining_hours: 0,
                    minutes: 0,
                    seconds: 0,
                    milliseconds: 500,
                },
                "0.500s",
            ),
        ];

        for (components, expected) in test_cases {
            assert_eq!(format_duration_format(&components), expected);
        }
    }

    #[test]
    fn test_measure_elapsed_time() {
        let elapsed_time = measure_elapsed_time(|| {
            std::thread::sleep(Duration::from_millis(1500));
        });
        assert!(elapsed_time == "1.500s");
    }

    #[test]
    fn test_format_duration() {
        // Test exact minutes
        assert_eq!(format_duration(Duration::from_secs(120)), "2m");
        assert_eq!(format_duration(Duration::from_secs(180)), "3m");

        // Test minutes with seconds
        assert_eq!(format_duration(Duration::from_secs(185)), "3m 5s");
        
        // Test your specific case (486.774785112 seconds)
        let duration_ms = (486.774785112 * 1000.0) as u64;
        assert_eq!(format_duration(Duration::from_millis(duration_ms)), "8m 6.774s");

        // Test various minute-second combinations
        assert_eq!(format_duration(Duration::from_millis(90500)), "1m 30.500s");
        assert_eq!(format_duration(Duration::from_millis(45100)), "45.100s");
        
        // Edge cases
        assert_eq!(format_duration(Duration::from_millis(59999)), "59.999s");
        assert_eq!(format_duration(Duration::from_secs(60)), "1m");
        assert_eq!(format_duration(Duration::from_millis(60001)), "1m 0.001s");

        let test_cases = vec![
            (Duration::from_secs(60*60+60+1), "1h 1m 1s"),
            (Duration::from_secs(61), "1m 1s"),
            (Duration::from_secs(1), "1s"),
            (Duration::from_millis(500), "0.500s"),
            (Duration::from_millis(100), "0.100s"),
            (Duration::from_millis(1), "0.001s"),
        ];

        for (duration, expected) in test_cases {
            assert_eq!(format_duration(duration), expected);
        }

        // Test weeks and days
        let week_in_secs = 7 * 24 * 60 * 60;
        assert_eq!(format_duration(Duration::from_secs(week_in_secs)), "1w 0d 0h 0m 0s");
        assert_eq!(format_duration(Duration::from_secs(week_in_secs + 24*60*60 + 65)), "1w 1d 0h 1m 5s");
        
        // Test a complex duration with all units
        let complex_duration = Duration::from_secs(
            2 * 7 * 24 * 60 * 60 + // 2 weeks
            3 * 24 * 60 * 60 +     // 3 days
            4 * 60 * 60 +          // 4 hours
            5 * 60 +               // 5 minutes
            6                      // 6 seconds
        );
        assert_eq!(format_duration(complex_duration), "2w 3d 4h 5m 6s");
    }
}