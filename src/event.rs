use std::fmt;

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
pub struct Events<TZ: TimeZone>(pub Vec<Event<TZ>>);

impl<TZ: TimeZone> fmt::Display for Events<TZ> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, event| {
            result.and_then(|_| writeln!(f, "{}", event))
        })
    }
}

#[derive(Debug)]
pub struct Event<TZ: TimeZone> {
    start_time: DateTime<TZ>,
    end_time: DateTime<TZ>,
    title: String,
    participants: Vec<String>,
}

impl<TZ: TimeZone> Event<TZ> {
    pub(crate) fn from_ical_events(
        events: Vec<ical::parser::ical::component::IcalEvent>,
    ) -> Events<Utc> {
        let mut ret = Vec::new();
        for event in events {
            let start_time_prop = event
                .properties
                .clone()
                .into_iter()
                .find(|prop| prop.name == "DTSTART");
            let start_time_string = start_time_prop.clone().unwrap().value.unwrap();
            let start_time = convert_to_date_time(&start_time_string);

            let end_time_prop = event
                .properties
                .clone()
                .into_iter()
                .find(|prop| prop.name == "DTEND");
            let end_time_string = end_time_prop.unwrap().value.unwrap();
            let end_time = convert_to_date_time(&end_time_string);

            let summary_prop = event
                .properties
                .clone()
                .into_iter()
                .find(|prop| prop.name == "SUMMARY");
            let summary_string = summary_prop.unwrap().value.unwrap_or("".to_string());

            let participants: Vec<String> = event
                .properties
                .clone()
                .into_iter()
                .filter(|prop| prop.name == "ATTENDEE")
                .map(|prop| prop.value.unwrap_or("mailto:".to_string()))
                .map(|s| s[7..s.len()].to_owned())
                .collect();

            let e = Event::<Utc> {
                start_time: start_time,
                end_time: end_time,
                participants: participants,
                title: summary_string,
            };

            ret.push(e);
        }

        let now = Utc::now();
        let mut ret = ret
            .into_iter()
            .filter(|event| {
                now.year() == event.start_time.year() && now.ordinal() == event.start_time.ordinal()
            })
            .collect::<Vec<Event<Utc>>>();
        ret.sort_by(|a, b| a.start_time.cmp(&b.start_time));
        Events(ret)
    }
}

impl<TZ: TimeZone> fmt::Display for Event<TZ> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2};{:0>2}:{:0>2};{};{}",
            self.start_time.hour(),
            self.start_time.minute(),
            self.end_time.hour(),
            self.end_time.minute(),
            self.title,
            self.participants.join(",")
        )
    }
}

fn convert_to_date_time(date: &str) -> DateTime<Utc> {
    let year = date[..4].parse::<i32>().unwrap();
    let month = date[4..6].parse::<u32>().unwrap();
    let day = date[6..8].parse::<u32>().unwrap();

    let hour = date[9..11].parse::<u32>().unwrap();
    let min = date[11..13].parse::<u32>().unwrap();
    let sec = date[13..15].parse::<u32>().unwrap();

    Utc.ymd(year, month, day).and_hms(hour, min, sec)
}
