pub enum EventKind {
    Note,  // 0
    Hold,  // 1
    LargeSpinner,  // 2
    LargeSpinnerLeft,  // 3
    LargeSpinnerRight,  // 4
    SmallSpinner,  // 5
    SmallSpinnerLeft,  // 6
    SmallSpinnerRight,  // 7
    MeasureMarker,  // 11
    BeatMarker,  // 12
    GraficaSectionStart,  // 14
    GraficaSectionClose,  // 15
}

pub struct Event {
    kind: EventKind,
    time: u32,
}
