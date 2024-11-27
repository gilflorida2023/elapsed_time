# Elapsed Time

A Rust library for measuring and formatting elapsed time in a human-readable format. This library provides simple and intuitive functions to measure code execution time and format time durations.

## Quick Start

Here's how to measure how long your code takes to run:

```rust
use elapsed_time::measure_elapsed_time;

// Example 1: Measure a computation
let result = measure_elapsed_time(|| {
    // Your computation here
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }
    println!("Sum: {}", sum);
});
println!("Computation took: {}", result); // e.g., "32ms"

// Example 2: Measure an I/O operation
let result = measure_elapsed_time(|| {
    std::fs::read_to_string("large_file.txt").unwrap();
});
println!("File read took: {}", result); // e.g., "1.243s"

// Example 3: Measure a network request
let result = measure_elapsed_time(|| {
    // Using reqwest for example
    // reqwest::blocking::get("https://www.rust-lang.org").unwrap();
});
println!("Request took: {}", result); // e.g., "843ms"
```

## Features

- Measure execution time of any code block
- Format durations in human-readable strings
- Intelligent millisecond formatting:
  - For durations under 1 second: displays in milliseconds (e.g., "500ms")
  - For durations of 1 second or more: displays as decimal seconds (e.g., "1.500s")
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
println!("Operation took: {}", elapsed); // e.g., "2.000s"
```

### Formatting Durations

Use `format_duration` to convert a `std::time::Duration` into a human-readable string:

```rust
use std::time::Duration;
use elapsed_time::format_duration;

let duration = Duration::from_secs(90061); // 1 day, 1 hour, 1 minute, 1 second
let formatted = format_duration(duration);
println!("{}", formatted); // "1d 1h 1m 1.000s"
```

## Format Examples

The library automatically chooses the most appropriate units for display:

```rust
use std::time::Duration;
use elapsed_time::format_duration;

// Weeks
format_duration(Duration::from_secs(1_209_600)); // "2w 0d 0h 0m 0.000s"

// Days
format_duration(Duration::from_secs(86400));     // "1d 0h 0m 0.000s"

// Hours
format_duration(Duration::from_secs(3600));      // "1h 0m 0.000s"

// Minutes
format_duration(Duration::from_secs(60));        // "1m 0.000s"

// Seconds with milliseconds
format_duration(Duration::from_millis(1500));    // "1.500s"

// Subsecond durations
format_duration(Duration::from_millis(500));     // "500ms"
format_duration(Duration::from_millis(100));     // "100ms"
format_duration(Duration::from_millis(1));       // "1ms"
```

## Millisecond Display Convention

The library uses an intelligent formatting convention for displaying milliseconds:

- For durations less than 1 second (subsecond durations):
  - Displays only in milliseconds (e.g., "500ms", "100ms", "1ms")
- For durations of 1 second or more:
  - Concatenates milliseconds as decimal seconds (e.g., "1.500s", "2.100s")
  - Always shows three decimal places for consistency

This makes the output both concise and readable, using the most appropriate format for the duration's scale.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
