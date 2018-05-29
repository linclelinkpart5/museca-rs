use types::{BPM, Offset, TimeSig};
use event::Event;

pub struct Chart {
    events: Vec<Event>,
    offset: Offset,
    start_bpm: BPM,
    start_ts: TimeSig,
}
