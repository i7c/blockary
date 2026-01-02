use crate::block::Block;
use chrono::{FixedOffset, NaiveDate, NaiveDateTime, Timelike};
use icalendar::{Calendar, CalendarDateTime, Component, DatePerhapsTime, Event};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CalDayPlan {
    pub origin: String,
    pub blocks: Vec<Block>,
}

impl CalDayPlan {
    pub fn from_icalendar(ical: &str, for_day: NaiveDate) -> Result<CalDayPlan, &'static str> {
        let calendar = match ical.parse::<Calendar>() {
            Ok(result) => result,
            Err(_) => return Err("Failed to parse ical"),
        };

        let mut blocks = Vec::new();

        for component in calendar.components {
            if let Some(event) = component.as_event() {
                let Some(start) = event.get_start() else {
                    continue;
                };
                let Some(start) = date_perhaps_time_to_naive(start) else {
                    continue;
                };
                let Some(end) = event.get_end() else { continue };
                let Some(end) = date_perhaps_time_to_naive(end) else {
                    continue;
                };

                if start.date() != end.date() {
                    continue;
                }

                if start.date() != for_day {
                    continue;
                }

                if let Some(period) = extract_period(event) {
                    blocks.push(Block {
                        period_str: period,
                        origin: "Calendar".to_string(),
                        desc: "Busy".to_string(),
                    });
                }
            }
        }

        Ok(CalDayPlan {
            origin: "Calendar".to_string(),
            blocks: blocks,
        })
    }
}

fn date_perhaps_time_to_naive(dpt: DatePerhapsTime) -> Option<NaiveDateTime> {
    if let DatePerhapsTime::DateTime(cdt) = dpt {
        let naive = match cdt {
            CalendarDateTime::Floating(naive) => naive,
            CalendarDateTime::Utc(date_time) => date_time
                .with_timezone(&FixedOffset::from_str("-03:00").unwrap())
                .naive_local(),
            CalendarDateTime::WithTimezone { date_time, tzid } => {
                let timezone = FixedOffset::from_str(&tzid).unwrap();
                date_time
                    .and_local_timezone(timezone)
                    .unwrap()
                    .naive_local()
            }
        };
        return Some(naive);
    } else {
        None
    }
}

fn extract_period(event: &Event) -> Option<String> {
    let start = event.get_start()?;
    let end = event.get_end()?;
    let start = date_perhaps_time_to_naive(start)?;
    let end = date_perhaps_time_to_naive(end)?;

    if start.date() == end.date() {
        return Some(format!(
            "{:02}:{:02} - {:02}:{:02}",
            start.hour(),
            start.minute(),
            end.hour(),
            end.minute()
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDate, Timelike};

    use super::*;

    #[test]
    fn test_load_from_valid_icalendar_string() {
        let ical_str = "BEGIN:VCALENDAR
PRODID:-//Google Inc//Google Calendar 70.9054//EN
VERSION:2.0
CALSCALE:GREGORIAN
METHOD:PUBLISH
X-WR-CALNAME:Privat
X-WR-TIMEZONE:America/Sao_Paulo
BEGIN:VEVENT
DTSTART:20251230T160000Z
DTEND:20251230T200000Z
DTSTAMP:20260101T181800Z
UID:e9im6r31d5miqs35e9pmurj1dgmn6ubecct30eb7ehhm8sjdecpmgrracssnaqrcd1ij8s3
 kd1i74ehh@google.com
ATTENDEE;X-NUM-GUESTS=0:mailto:constantin.weisser@gmail.com
RECURRENCE-ID:20251230T160000Z
SUMMARY:Busy
END:VEVENT
BEGIN:VEVENT
DTSTART:20260101T120000Z
DTEND:20260101T160000Z
DTSTAMP:20260101T181800Z
UID:e9im6r31d5miqs35e9pmurj1dgmn6ubecct36q1melp32s3gchq6qobgchqj4s3bd1h3arh
 j6pnmkehh@google.com
ATTENDEE;X-NUM-GUESTS=0:mailto:constantin.weisser@gmail.com
RECURRENCE-ID:20260101T120000Z
SUMMARY:Busy
END:VEVENT
END:VCALENDAR
";

        let for_day: NaiveDate = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

        let day_plan = CalDayPlan::from_icalendar(ical_str, for_day).unwrap();
        assert_eq!(day_plan.blocks.len(), 1);
        assert_eq!(day_plan.blocks.get(0).unwrap().origin, "Calendar");
        assert_eq!(day_plan.blocks.get(0).unwrap().period_str, "09:00 - 13:00");
        assert_eq!(day_plan.blocks.get(0).unwrap().desc, "Busy");
    }

    #[test]
    fn test_extract_time_from_date_perhaps_time() {
        let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8)
            .unwrap()
            .and_hms_opt(9, 10, 0)
            .unwrap();

        let dpt = DatePerhapsTime::from(dt);

        let naive = date_perhaps_time_to_naive(dpt).unwrap();

        assert_eq!(naive.day(), 8);
        assert_eq!(naive.month(), 7);
        assert_eq!(naive.year(), 2016);
        assert_eq!(naive.hour(), 9);
        assert_eq!(naive.minute(), 10);
    }
}
