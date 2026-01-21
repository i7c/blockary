use crate::{
    block::Block,
    day_plan::{self, DayPlan, Source},
};
use chrono::{FixedOffset, NaiveDate, NaiveDateTime, Timelike};
use icalendar::{Calendar, CalendarDateTime, Component, DatePerhapsTime, Event};
use std::{collections::HashMap, str::FromStr};

pub fn day_plans_from_ical(ical: &str) -> Result<Vec<DayPlan>, String> {
    let origin = "Calendar";

    let calendar = ical
        .parse::<Calendar>()
        .map_err(|e| format!("Failed to parse the calendar: {}", e))?;

    let single_day_events: Vec<&Event> = calendar
        .components
        .iter()
        .filter_map(|comp| comp.as_event())
        .filter(|event| event_date(event).is_some())
        .collect();

    let mut blocks_per_day: HashMap<NaiveDate, Vec<Block>> = HashMap::new();
    for event in single_day_events {
        match extract_period(event) {
            Some(period_str) => {
                let block = Block {
                    period_str: period_str,
                    origin: origin.to_string(),
                    desc: event
                        .get_description()
                        .unwrap_or_else(|| "Busy")
                        .to_string(),
                };
                blocks_per_day
                    .entry(event_date(event).unwrap())
                    .or_insert(Vec::new())
                    .push(block);
            }
            None => continue,
        }
    }

    let mut day_plans = Vec::new();
    for (day, blocks) in blocks_per_day {
        day_plans.push(DayPlan {
            origin: origin.to_string(),
            blocks: blocks,
            day: Some(day),
            source: Source::ICalendar,
        });
    }

    Ok(day_plans)
}

pub fn from_icalendar(ical: &str, for_day: NaiveDate) -> Result<DayPlan, String> {
    let origin = "Calendar";

    let calendar = ical
        .parse::<Calendar>()
        .map_err(|e| format!("Failed to parse the calendar: {}", e))?;

    let blocks = calendar
        .components
        .iter()
        .filter_map(|comp| comp.as_event())
        .filter_map(|event| {
            let start = date_perhaps_time_to_naive(event.get_start()?)?;
            let end = date_perhaps_time_to_naive(event.get_end()?)?;

            if start.date() != end.date() || start.date() != for_day {
                None
            } else {
                let period = extract_period(event)?;

                Some(Block {
                    period_str: period,
                    origin: origin.to_string(),
                    desc: event
                        .get_description()
                        .unwrap_or_else(|| "Busy")
                        .to_string(),
                })
            }
        })
        .collect();

    Ok(DayPlan {
        origin: origin.to_string(),
        blocks: blocks,
        day: Some(for_day),
        source: day_plan::Source::ICalendar,
    })
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

fn event_date(event: &Event) -> Option<NaiveDate> {
    let start = date_perhaps_time_to_naive(event.get_start()?)?;
    let end = date_perhaps_time_to_naive(event.get_end()?)?;

    if start.date() == end.date() {
        Some(start.date())
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
UID:xxxxxxxxxxxxxqs35e9pmurj1dgmn6ubecct30eb7ehhm8sjdecpmgrracssnaqrcd1ij8s3
 kd1i123123@gmail.com
ATTENDEE;X-NUM-GUESTS=0:mailto:xyzxyzxyzxyz@gmail.com
RECURRENCE-ID:20251230T160000Z
SUMMARY:Busy
END:VEVENT
BEGIN:VEVENT
DTSTART:20260101T120000Z
DTEND:20260101T160000Z
DTSTAMP:20260101T181800Z
UID:xxxxxxxxxxxxxqs35e9pmurj1dgmn6ubecct36q1melp32s3gchq6qobgchqj4s3bd1h3arh
 j6pn123123@gmail.com
ATTENDEE;X-NUM-GUESTS=0:mailto:xyzxyzxyzxyz@gmail.com
RECURRENCE-ID:20260101T120000Z
SUMMARY:Busy
END:VEVENT
END:VCALENDAR
";

        let for_day: NaiveDate = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

        let day_plan = from_icalendar(ical_str, for_day).unwrap();
        assert_eq!(day_plan.blocks.len(), 1);
        assert_eq!(day_plan.blocks.get(0).unwrap().origin, "Calendar");
        assert_eq!(day_plan.blocks.get(0).unwrap().period_str, "09:00 - 13:00");
        assert_eq!(day_plan.blocks.get(0).unwrap().desc, "Busy");
    }

    #[test]
    fn test_load_from_valid_icalendar_string_overlapping_blocks() {
        let ical_str = "BEGIN:VCALENDAR
PRODID:-//Google Inc//Google Calendar 70.9054//EN
VERSION:2.0
CALSCALE:GREGORIAN
METHOD:PUBLISH
X-WR-CALNAME:Privat
X-WR-TIMEZONE:America/Sao_Paulo
BEGIN:VEVENT
DTSTART:20260117T130000Z
DTEND:20260117T134500Z
DTSTAMP:20260117T124851Z
UID:xxxxxxxxxxxxxjtckejqbq123123@gmail.com
ATTENDEE;X-NUM-GUESTS=0:mailto:xyzxyzxyzxyz@gmail.com
SUMMARY:Busy
END:VEVENT
BEGIN:VEVENT
DTSTART:20260117T134500Z
DTEND:20260117T141500Z
DTSTAMP:20260117T124851Z
UID:xxxxxxxxxxxxxrdnmue21k123123@gmail.com
ATTENDEE;X-NUM-GUESTS=0:mailto:xyzxyzxyzxyz@gmail.com
SUMMARY:Busy
END:VEVENT
BEGIN:VEVENT
DTSTART:20251218T150000Z
DTEND:20251218T160000Z
DTSTAMP:20260117T124851Z
UID:xxxxxxxxxxxxx3-4419-BB0F-6A74FC4C37A9
ATTENDEE;X-NUM-GUESTS=0:mailto:xyzxyzxyzxyz@gmail.com
SUMMARY:Busy
END:VEVENT
BEGIN:VEVENT
DTSTART:20260102T210000Z
DTEND:20260102T220000Z
DTSTAMP:20260117T124851Z
UID:xxxxxxxxxxxxxnmrlcmvr0123123@gmail.com
ATTENDEE;X-NUM-GUESTS=0:mailto:xyzxyzxyzxyz@gmail.com
SUMMARY:Busy
END:VEVENT
BEGIN:VEVENT
DTSTART:20251201T130000Z
DTEND:20251201T140000Z
DTSTAMP:20260117T124851Z
UID:xxxxxxxxxxxxx4-425C-80D6-F29E185406F1
ATTENDEE;X-NUM-GUESTS=0:mailto:xyzxyzxyzxyz@gmail.com
SUMMARY:Busy
END:VEVENT
BEGIN:VEVENT
DTSTART:20260117T131500Z
DTEND:20260117T140000Z
DTSTAMP:20260117T124851Z
UID:xxxxxxxxxxxxxm1rmrirnj123123@gmail.com
ATTENDEE;X-NUM-GUESTS=0:mailto:xyzxyzxyzxyz@gmail.com
SUMMARY:Busy
END:VEVENT
END:VCALENDAR";

        let for_day: NaiveDate = NaiveDate::from_ymd_opt(2026, 1, 17).unwrap();

        let day_plan = from_icalendar(ical_str, for_day).unwrap();
        assert_eq!(day_plan.blocks.len(), 3);
        assert_eq!(day_plan.blocks.get(0).unwrap().origin, "Calendar");
        assert_eq!(day_plan.blocks.get(0).unwrap().period_str, "10:00 - 10:45");
        assert_eq!(day_plan.blocks.get(0).unwrap().desc, "Busy");
        assert_eq!(day_plan.blocks.get(0).unwrap().origin, "Calendar");
        assert_eq!(day_plan.blocks.get(0).unwrap().period_str, "10:00 - 10:45");
        assert_eq!(day_plan.blocks.get(0).unwrap().desc, "Busy");
        assert_eq!(day_plan.blocks.get(0).unwrap().origin, "Calendar");
        assert_eq!(day_plan.blocks.get(0).unwrap().period_str, "10:00 - 10:45");
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
