/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Buffer Implementation
//!
//! Rust port of `com.cburch.logisim.std.gates.Buffer`
//! TODO: Full implementation needed

use crate::comp::{Component, ComponentId, Pin, UpdateResult};

/// Buffer gate implementation (placeholder)
///
/// A buffer simply passes its input to its output, potentially with some delay.
/// This is commonly used for signal buffering and driving capability.
#[derive(Debug)]
pub struct Buffer {
    id: ComponentId,
}

impl Buffer {
    pub fn new(id: ComponentId) -> Self {
        Buffer { id }
    }
}

impl Component for Buffer {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Buffer"
    }

    fn pins(&self) -> &std::collections::HashMap<String, Pin> {
        todo!("Buffer implementation needed")
    }

    fn pins_mut(&mut self) -> &mut std::collections::HashMap<String, Pin> {
        todo!("Buffer implementation needed")
    }

    fn update(
        &mut self,
        _current_time: crate::signal::Timestamp,
    ) -> UpdateResult {
        todo!("Buffer implementation needed")
    }

    fn reset(&mut self) {
        todo!("Buffer implementation needed")
    }

    fn propagation_delay(&self) -> u64 {
        1
    }
}
