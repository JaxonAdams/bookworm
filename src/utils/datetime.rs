use chrono::{Local, NaiveDateTime};

/// Converts a UTC timestamp string from SQLite (e.g. "2026-06-30 19:08:00")
/// into a human-friendly local time string (e.g. "Jun 30, 2026 at 1:08 PM").
pub fn format_utc_as_local(utc_str: &str) -> String {
    let Ok(naive) = NaiveDateTime::parse_from_str(utc_str, "%Y-%m-%d %H:%M:%S") else {
        return utc_str.to_string();
    };

    let utc_dt = naive.and_utc();
    let local_dt = utc_dt.with_timezone(&Local);

    local_dt.format("%b %-d, %Y at %-I:%M %p").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_utc_as_local_produces_output() {
        let result = format_utc_as_local("2026-06-30 19:08:00");
        // We can't assert an exact string since it depends on local tz,
        // but we can verify it doesn't return the raw input unchanged
        // (unless the machine is in UTC, in which case format still differs).
        assert!(result.contains("2026"));
        assert!(result.contains("Jun") || result.contains("Jul"));
    }

    #[test]
    fn test_format_utc_as_local_handles_invalid_input() {
        let result = format_utc_as_local("not a date");
        assert_eq!(result, "not a date");
    }
}
