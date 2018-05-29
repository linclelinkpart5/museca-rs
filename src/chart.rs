use types::{BPM, Offset, TimeSig};
use event::Event;

mod builder {
    use event::Event;

    pub struct ChartBuilder {
        events: Vec<Event>,
        offset: u32,
    }
}

use self::builder::ChartBuilder;

pub struct Chart {
    events: Vec<Event>,
    offset: u32,
    start_bpm: u16,
    start_ts: (u8, u8),
}
