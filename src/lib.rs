use regex::Regex;
use std::time::Duration;

pub fn parse_duration(dur: &str) -> Result<Duration, String> {
    let re = Regex::new(r"([\d\.]{1,})\s?([a-z])").unwrap();
    let mut duration = Duration::new(0, 0);
    for cap in re.captures_iter(dur) {
        let num: f32 = cap[1]
            .parse()
            .map_err(|_| format!("{} should be a number", cap[1].to_owned()))?;
        match &cap[2] {
            "d" | "day" | "days" => duration += days(num),
            "h" | "hour" | "hours" => duration += hours(num),
            "m" | "minute" | "minutes" => duration += minutes(num),
            "s" | "second" | "seconds" => duration += seconds(num),
            unknown => return Err(format!("unknown unit: {}", unknown)),
        }
    }

    Ok(duration)
}

fn days(d: f32) -> Duration {
    hours(d * 24.0)
}

fn hours(h: f32) -> Duration {
    minutes(h * 60.0)
}

fn minutes(m: f32) -> Duration {
    seconds(m * 60.0)
}

fn seconds(s: f32) -> Duration {
    Duration::from_secs(s as u64)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_day() -> Result<(), String> {
        assert_eq!(days(1.0), parse_duration("1 day")?);
        assert_eq!(days(1.0), parse_duration("1d")?);
        assert_eq!(days(1.0), parse_duration("1 days")?);
        assert_eq!(days(2.0), parse_duration("2 days")?);
        assert_eq!(hours(12.0), parse_duration("0.5 day")?);
        assert_eq!(hours(60.0), parse_duration("2.5 days")?);
        Ok(())
    }

    #[test]
    fn parse_hour() -> Result<(), String> {
        assert_eq!(hours(1.0), parse_duration("1 hour")?);
        assert_eq!(hours(1.0), parse_duration("1h")?);
        assert_eq!(minutes(30.0), parse_duration("0.5 hours")?);
        assert_eq!(minutes(30.0), parse_duration("0.5h")?);
        assert_eq!(hours(2.0), parse_duration("2 hours")?);
        assert_eq!(minutes(150.0), parse_duration("2.5 hours")?);
        assert_eq!(minutes(150.0), parse_duration("2.5h")?);
        assert_eq!(hours(72.0), parse_duration("72h")?);
        Ok(())
    }

    #[test]
    fn parse_minute() -> Result<(), String> {
        assert_eq!(minutes(1.0), parse_duration("1 minute")?);
        assert_eq!(minutes(1.0), parse_duration("1m")?);
        assert_eq!(seconds(30.0), parse_duration("0.5 minutes")?);
        assert_eq!(seconds(30.0), parse_duration("0.5m")?);
        assert_eq!(minutes(2.0), parse_duration("2 minutes")?);
        assert_eq!(seconds(150.0), parse_duration("2.5 minute")?);
        assert_eq!(seconds(150.0), parse_duration("2.5m")?);
        assert_eq!(minutes(150.0), parse_duration("150m")?);
        Ok(())
    }

    #[test]
    fn parse_second() -> Result<(), String> {
        assert_eq!(seconds(1.0), parse_duration("1 second")?);
        assert_eq!(seconds(1.0), parse_duration("1s")?);
        assert_eq!(seconds(2.0), parse_duration("2 seconds")?);
        assert_eq!(hours(1.0), parse_duration("3600s")?);
        Ok(())
    }

    #[test]
    fn parse_mixed_duration() -> Result<(), String> {
        assert_eq!(
            seconds(102190.0),
            parse_duration("1 day 4 hours 23 minutes 10 seconds")?
        );
        assert_eq!(seconds(102190.0), parse_duration("1d 4h 23m 10s")?);
        assert_eq!(seconds(14410.0), parse_duration("4hours 10s")?);
        assert_eq!(seconds(100800.0), parse_duration("1d 4 hours")?);
        assert_eq!(seconds(1439.0), parse_duration("23 minutes 59 seconds")?);
        Ok(())
    }
}
