mod kind {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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

    impl EventKind {
        pub fn id(&self) -> u8 {
            match &self {
                &EventKind::Note => 0,
                &EventKind::Hold => 1,
                &EventKind::LargeSpinner => 2,
                &EventKind::LargeSpinnerLeft => 3,
                &EventKind::LargeSpinnerRight => 4,
                &EventKind::SmallSpinner => 5,
                &EventKind::SmallSpinnerLeft => 6,
                &EventKind::SmallSpinnerRight => 7,
                &EventKind::MeasureMarker => 11,
                &EventKind::BeatMarker => 12,
                &EventKind::GraficaSectionStart => 14,
                &EventKind::GraficaSectionClose => 15,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::EventKind;

        #[test]
        pub fn test_id() {
            let inputs_and_expected = vec![
                ((EventKind::Note,), 0u8),
                ((EventKind::Hold,), 1),
                ((EventKind::LargeSpinner,), 2),
                ((EventKind::LargeSpinnerLeft,), 3),
                ((EventKind::LargeSpinnerRight,), 4),
                ((EventKind::SmallSpinner,), 5),
                ((EventKind::SmallSpinnerLeft,), 6),
                ((EventKind::SmallSpinnerRight,), 7),
                ((EventKind::MeasureMarker,), 11),
                ((EventKind::BeatMarker,), 12),
                ((EventKind::GraficaSectionStart,), 14),
                ((EventKind::GraficaSectionClose,), 15),
            ];

            for (input, expected) in inputs_and_expected {
                let (kind,) = input;
                let produced = kind.id();
                assert_eq!(expected, produced);
            }
        }
    }
}

use self::kind::EventKind;
use common::types::Offset;

/// Represents an event in a MUSECA notechart.
/// Events consists of a kind (note, hold, spin, etc.) and an offset.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Event {
    pub offset: Offset,
    pub kind: EventKind,
}

impl Event {
    pub fn new(offset: Offset, kind: EventKind) -> Self {
        Event { offset, kind, }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, Offset, EventKind};

    #[test]
    fn test_new() {
        let inputs_and_expected = vec![
            ((10u32, EventKind::Note), Event{ offset: 10, kind: EventKind::Note, }),
            ((20u32, EventKind::LargeSpinner), Event{ offset: 20, kind: EventKind::LargeSpinner, }),
            ((30u32, EventKind::BeatMarker), Event{ offset: 30, kind: EventKind::BeatMarker, }),
        ];

        for (input, expected) in inputs_and_expected {
            let (offset, kind) = input;
            let produced = Event::new(offset, kind);
            assert_eq!(expected, produced);
        }
    }
}
