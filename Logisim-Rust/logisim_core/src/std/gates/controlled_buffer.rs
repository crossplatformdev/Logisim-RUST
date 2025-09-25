/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Controlled Buffer Implementation
//!
//! Rust port of `com.cburch.logisim.std.gates.ControlledBuffer`
//! TODO: Full implementation needed

use crate::component::{Component, ComponentId};

/// Controlled buffer implementation (placeholder)
///
/// A controlled buffer (tri-state buffer) passes its input to output when enabled,
/// otherwise presents high impedance.
#[derive(Debug)]
pub struct ControlledBuffer {
    id: ComponentId,
}

impl ControlledBuffer {
    pub fn new(id: ComponentId) -> Self {
        ControlledBuffer { id }
    }
}

impl Component for ControlledBuffer {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Controlled Buffer"
    }

    fn pins(&self) -> &std::collections::HashMap<String, crate::component::Pin> {
        todo!("ControlledBuffer implementation needed")
    }

    fn pins_mut(&mut self) -> &mut std::collections::HashMap<String, crate::component::Pin> {
        todo!("ControlledBuffer implementation needed")
    }

    fn update(
        &mut self,
        _current_time: crate::signal::Timestamp,
    ) -> crate::component::UpdateResult {
        todo!("ControlledBuffer implementation needed")
    }

    fn reset(&mut self) {
        todo!("ControlledBuffer implementation needed")
    }

    fn propagation_delay(&self) -> u64 {
        2
    }
}
