use std::fmt;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// Utc specifies the UTC time zone.
pub struct Utc {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minute: u64,
    second: u64,
}

impl Utc {
    /// Returns the current time in UTC with no offset applied.
    ///
    /// This is equivalent to calling [`Utc::now_with_offset_hm(0, 0)`],
    /// producing a pure UTC timestamp.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let now = Utc::now(); // exact UTC time
    /// println!("Now: {}", now);
    /// ```
    /// Possible result
    /// ```txt
    /// Now: 2025-11-25 19:05:01 UTC
    /// ```
    #[must_use]
    #[allow(unused)]
    pub fn now() -> Self {
        Self::now_with_offset_hm(0, 0)
    }

    /// Returns the current time with offset in hours and minutes
    /// `offset_hours`: can be negative or positive
    /// `offset_minutes`: typically 0, 15, 30, 45
    ///
    /// example
    /// ```rust,ignore
    /// let brasil = Utc::now_with_offset_hm(-3, 0);  // UTC-3
    /// println!("Brasil: {}", brasil);
    /// ```
    /// Possible result
    /// ```txt
    /// Brasil: 2025-11-25 16:05:01 UTC
    /// ```
    #[must_use]
    #[allow(unused)]
    pub fn now_with_offset_hm(offset_hours: i64, offset_minutes: i64) -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        let secs = i64::try_from(duration.as_secs()).unwrap_or(i64::MAX);
        let total_seconds = secs + offset_hours * 3600 + offset_minutes * 60;

        // avoids dates before 1970
        let seconds = if total_seconds < 0 { 0 } else { total_seconds };
        let days = u64::try_from(seconds / 86_400).unwrap_or_default();
        let secs_of_day = u64::try_from(seconds % 86_400).unwrap_or_default();

        let (year, month, day) = days_to_ymd(days);
        let hour = secs_of_day / 3600;
        let minute = (secs_of_day % 3600) / 60;
        let second = secs_of_day % 60;

        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl fmt::Display for Utc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

/// Converts "days since 1970-01-01" to (year, month, day) UTC.
/// Algorithm based on Zeller /proleptic Gregorian calendar.
#[allow(unused)]
fn days_to_ymd(mut days: u64) -> (u64, u64, u64) {
    let mut year = 1970;

    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let month_lengths = if is_leap(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for &len in &month_lengths {
        if days < len {
            return (year, month, days + 1);
        }
        days -= len;
        month += 1;
    }

    (year, 12, 31)
}

fn is_leap(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

//=====================================================================
// UNIT TESTS
//=====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_year() {
        assert!(is_leap(2000)); // 2000 is a leap year
        assert!(!is_leap(1900)); // 1900 is NOT leap year
        assert!(is_leap(2024)); // 2024 is leap year
    }

    #[test]
    fn test_days_to_ymd_epoch() {
        let (year, month, day) = days_to_ymd(0); // 1970-01-01
        assert_eq!((year, month, day), (1970, 1, 1));
    }

    #[test]
    fn test_time_now_format() {
        let t = Utc {
            year: 2025,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        };
        assert_eq!(t.to_string(), "2025-01-01 00:00:00 UTC");
    }

    // Test day conversion
    #[test]
    fn test_days_to_ymd_specific_date() {
        // 1 de janeiro de 2000: days since 1970 = 10957
        let (year, month, day) = days_to_ymd(10957);
        assert_eq!((year, month, day), (2000, 1, 1));
    }

    // Test date increments
    #[test]
    fn test_days_to_ymd_end_of_month() {
        // 31 de janeiro de 1970: 30 days since epoch
        let (year, month, day) = days_to_ymd(30);
        assert_eq!((year, month, day), (1970, 1, 31));
    }

    #[test]
    fn test_utc_now_no_offset() {
        let utc = Utc::now();
        // Tests whether the time is within a plausible range (year >= 1970)
        assert!(utc.year >= 1970);
        assert!(utc.month >= 1 && utc.month <= 12);
        assert!(utc.day >= 1 && utc.day <= 31);
        assert!(utc.hour <= 23);
        assert!(utc.minute <= 59);
        assert!(utc.second <= 59);
    }

    #[test]
    fn test_utc_offset_positive() {
        let utc = Utc::now();
        let offset = Utc::now_with_offset_hm(2, 30); // UTC+2:30

        // The applied offset must increase hours/minutes
        let mut expected_hour = (utc.hour as i64 + 2) % 24;
        let mut expected_minute = utc.minute as i64 + 30;
        if expected_minute >= 60 {
            expected_minute -= 60;
            expected_hour = (expected_hour + 1) % 24;
        }

        assert_eq!(offset.hour as i64, expected_hour);
        assert_eq!(offset.minute as i64, expected_minute);
    }

    #[test]
    fn test_utc_offset_negative() {
        let utc = Utc::now();
        let offset = Utc::now_with_offset_hm(-3, 0); // UTC-3

        // The applied offset must decrease hours
        let mut expected_hour = utc.hour as i64 - 3;
        let expected_minute = utc.minute as i64;
        if expected_hour < 0 {
            expected_hour += 24;
        }

        assert_eq!(offset.hour as i64, expected_hour);
        assert_eq!(offset.minute as i64, expected_minute);
    }
}
