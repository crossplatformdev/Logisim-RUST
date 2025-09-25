/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! List data event listener contract

/// List data event
#[derive(Debug, Clone)]
pub struct ListDataEvent {
    pub source_id: u32,
    pub event_type: ListDataEventType,
    pub index0: usize,
    pub index1: usize,
}

#[derive(Debug, Clone)]
pub enum ListDataEventType {
    IntervalAdded,
    IntervalRemoved,
    ContentsChanged,
}

/// Base contract for list data listeners with default no-op implementations
pub trait BaseListDataListenerContract {
    /// Called when items are added to the list
    fn interval_added(&mut self, _event: &ListDataEvent) {
        // no-op implementation
    }

    /// Called when items are removed from the list
    fn interval_removed(&mut self, _event: &ListDataEvent) {
        // no-op implementation
    }

    /// Called when list contents change
    fn contents_changed(&mut self, _event: &ListDataEvent) {
        // no-op implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener;

    impl BaseListDataListenerContract for TestListener {}

    #[test]
    fn test_default_implementations() {
        let mut listener = TestListener;
        let event = ListDataEvent {
            source_id: 1,
            event_type: ListDataEventType::IntervalAdded,
            index0: 0,
            index1: 2,
        };

        // Should not panic - all methods have default implementations
        listener.interval_added(&event);
        listener.interval_removed(&event);
        listener.contents_changed(&event);
    }
}
