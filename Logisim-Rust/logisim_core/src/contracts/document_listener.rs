/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Document event listener contract

/// Document event data
#[derive(Debug, Clone)]
pub struct DocumentEvent {
    pub document_id: u32,
    pub event_type: DocumentEventType,
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub enum DocumentEventType {
    Insert,
    Remove,
    Change,
}

/// Base contract for document listeners with default no-op implementations
pub trait BaseDocumentListenerContract {
    /// Called when text is inserted into the document
    fn insert_update(&mut self, _event: &DocumentEvent) {
        // no-op implementation
    }

    /// Called when text is removed from the document
    fn remove_update(&mut self, _event: &DocumentEvent) {
        // no-op implementation
    }

    /// Called when document attributes change
    fn changed_update(&mut self, _event: &DocumentEvent) {
        // no-op implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener;

    impl BaseDocumentListenerContract for TestListener {}

    #[test]
    fn test_default_implementations() {
        let mut listener = TestListener;
        let event = DocumentEvent {
            document_id: 1,
            event_type: DocumentEventType::Insert,
            offset: 0,
            length: 5,
        };

        // Should not panic - all methods have default implementations
        listener.insert_update(&event);
        listener.remove_update(&event);
        listener.changed_update(&event);
    }
}
