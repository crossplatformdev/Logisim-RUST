/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL model event listener interface

use super::hdl_model::HdlModel;

/// Listener interface for HDL model events
pub trait HdlModelListener: Send + Sync {
    /// Called when the content of the given HDL model has been set
    fn content_set(&self, source: &dyn HdlModel);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdl_model::BasicHdlModel;

    struct TestListener {
        pub calls: std::sync::Mutex<usize>,
    }

    impl HdlModelListener for TestListener {
        fn content_set(&self, _source: &dyn HdlModel) {
            let mut calls = self.calls.lock().unwrap();
            *calls += 1;
        }
    }

    #[test]
    fn test_hdl_model_listener() {
        let model = BasicHdlModel::new(
            "test".to_string(),
            "content".to_string(),
            vec![],
            vec![],
        );

        let listener = TestListener {
            calls: std::sync::Mutex::new(0),
        };
        
        listener.content_set(&model);
        
        let calls = listener.calls.lock().unwrap();
        assert_eq!(*calls, 1);
    }
}
