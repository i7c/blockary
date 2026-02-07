use std::collections::HashMap;

use crate::{block::Block, day_plan};

pub struct TagTime {
    pub tag: String,
    pub minutes: u16,
    pub sub_tags: Vec<TagTime>,
}

pub fn minutes_to_hours_minutes(total_duration_today: u16) -> (u16, u16) {
    let hours = total_duration_today / 60;
    let minutes = total_duration_today % 60;
    (hours, minutes)
}

pub fn time_per_tag(all_blocks: &Vec<&Block>, level: usize) -> Vec<TagTime> {
    let mut groups: HashMap<&str, Vec<&Block>> = HashMap::new();
    for b in all_blocks {
        for tag in &b.tags {
            match tag.tagls.get(level) {
                Some(tagl) => {
                    groups.entry(&tagl).or_insert(Vec::new()).push(&b);
                }
                _ => {}
            }
        }
    }

    let mut timings = Vec::new();
    for (tagl, blocks) in groups {
        let total_accumulated = total_minutes(&blocks);
        timings.push(TagTime {
            tag: tagl.to_string(),
            minutes: total_accumulated,
            sub_tags: time_per_tag(&blocks.into(), level + 1),
        });
    }
    timings
}

pub fn total_minutes(blocks: &Vec<&Block>) -> u16 {
    blocks.iter().fold(0, |total, b| total + b.duration)
}

pub fn total_time_spent(all_of_day: &Vec<day_plan::DayPlan>) -> u16 {
    let total_duration_today = all_of_day.iter().fold(0, |total_duration, dp| {
        total_duration
            + dp.only_original_blocks().iter().fold(0, |acc, b| {
                if b.tags
                    .iter()
                    .any(|tag| match tag.tagls.get(0).map(|s| s.as_ref()) {
                        Some("break") => true,
                        _ => false,
                    })
                {
                    acc
                } else {
                    acc + b.duration
                }
            })
    });
    total_duration_today
}
