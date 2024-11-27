# Elapsed Time

A Rust library for measuring and formatting elapsed time in a human-readable format. This library provides simple and intuitive functions to measure code execution time and format time durations.

## Features

- Measure execution time of any code block
- Format durations in human-readable strings
- Support for multiple time units:
  - Weeks
  - Days
  - Hours
  - Minutes
  - Seconds
  - Milliseconds
- Automatic selection of most appropriate time units
- Zero dependencies (only uses Rust standard library)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elapsed_time = "0.1.0"
```

## Usage

### Measuring Execution Time

Use `measure_elapsed_time` to measure how long a piece of code takes to execute:

```rust
use elapsed_time::measure_elapsed_time;

let elapsed = measure_elapsed_time(|| {
    // Your code here
    std::thread::sleep(std::time::Duration::from_secs(2));
});
println!("Operation took: {}", elapsed); // e.g., "2s 0ms"
```

### Formatting Durations

Use `format_duration` to convert a `std::time::Duration` into a human-readable string:

```rust
use std::time::Duration;
use elapsed_time::format_duration;

let duration = Duration::from_secs(90061); // 1 day, 1 hour, 1 minute, 1 second
let formatted = format_duration(duration);
println!("{}", formatted); // "1d 1h 1m 1s 0ms"
```

## Format Examples

The library automatically chooses the most appropriate units for display:

```rust
use std::time::Duration;
use elapsed_time::format_duration;

// Weeks
format_duration(Duration::from_secs(1_209_600)); // "2w 0d 0h 0m 0s 0ms"

// Days
format_duration(Duration::from_secs(86400));     // "1d 0h 0m 0s 0ms"

// Hours
format_duration(Duration::from_secs(3600));      // "1h 0m 0s 0ms"

// Minutes
format_duration(Duration::from_secs(60));        // "1m 0s 0ms"

// Seconds
format_duration(Duration::from_secs(1));         // "1s 0ms"

// Milliseconds
format_duration(Duration::from_millis(500));     // "500ms"
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
