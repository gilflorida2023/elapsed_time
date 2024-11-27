use std::time::Instant;

/// Measures the elapsed time of a given function and returns a formatted string representation.
///
/// This function takes a closure as an argument, executes it, and measures the time it takes to run.
/// The elapsed time is then formatted into a human-readable string.
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
/// let elapsed_time = measure_elapsed_time(|| {
///     sleep(Duration::from_millis(1500));
/// });
/// assert!(elapsed_time == "1s 500ms" || elapsed_time == "1s 501ms");
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
    if components.weeks > 0 {
        format!("{}w {}d {}h {}m {}s {}ms", 
            components.weeks, components.remaining_days, components.remaining_hours, 
            components.minutes, components.seconds, components.milliseconds)
    } else if components.remaining_days > 0 {
        format!("{}d {}h {}m {}s {}ms", 
            components.remaining_days, components.remaining_hours, 
            components.minutes, components.seconds, components.milliseconds)
    } else if components.remaining_hours > 0 {
        format!("{}h {}m {}s {}ms", 
            components.remaining_hours, components.minutes, 
            components.seconds, components.milliseconds)
    } else if components.minutes > 0 {
        format!("{}m {}s {}ms", 
            components.minutes, components.seconds, components.milliseconds)
    } else if components.seconds > 0 {
        format!("{}s {}ms", components.seconds, components.milliseconds)
    } else {
        format!("{}ms", components.milliseconds)
    }
}

/// Formats a Duration into a human-readable string.
///
/// This function takes a Duration and converts it into a string representation
/// with appropriate units (hours, minutes, seconds, milliseconds) based on the duration's length.
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
/// assert_eq!(format_duration(Duration::from_secs(60*60+60+1)), "1h 1m 1s 0ms");
/// assert_eq!(format_duration(Duration::from_secs(61)), "1m 1s 0ms");
/// assert_eq!(format_duration(Duration::from_secs(1)), "1s 0ms");
/// assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
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
                "1w 2d 3h 4m 5s 6ms",
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
                "2d 3h 4m 5s 6ms",
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
                "3h 4m 5s 6ms",
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
        assert!(elapsed_time == "1s 500ms" || elapsed_time == "1s 501ms");
    }

    #[test]
    fn test_format_duration() {
        let test_cases = vec![
            (Duration::from_secs(60*60+60+1), "1h 1m 1s 0ms"),
            (Duration::from_secs(61), "1m 1s 0ms"),
            (Duration::from_secs(1), "1s 0ms"),
            (Duration::from_millis(500), "500ms"),
        ];

        for (duration, expected) in test_cases {
            assert_eq!(format_duration(duration), expected);
        }
    }
}