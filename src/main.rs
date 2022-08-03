extern crate ical;
mod event;

use std::{io::BufReader, fs::File};

use event::Event;
fn main() {
    let buf = BufReader::new(File::open("testdata/nikola.ics").unwrap());

    let reader = ical::IcalParser::new(buf);

    for line in reader {
        let calendar = line.unwrap();
        let events = Event::<chrono::Utc>::from_ical_events(calendar.events);
        println!("{}", events)
    }
}
