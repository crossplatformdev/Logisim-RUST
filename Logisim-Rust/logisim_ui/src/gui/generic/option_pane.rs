//! Option pane and dialog utilities
//!
//! This module provides a unified interface for displaying various types of dialogs,
//! equivalent to the Java OptionPane class.

use log::{error, info, warn};

/// Option pane for displaying dialogs
/// Equivalent to Java's OptionPane class
pub struct OptionPane;

impl OptionPane {
    // Constants matching Java JOptionPane
    pub const YES_NO_OPTION: i32 = 0;
    pub const YES_NO_CANCEL_OPTION: i32 = 1;
    pub const OK_CANCEL_OPTION: i32 = 2;

    pub const YES_OPTION: i32 = 0;
    pub const NO_OPTION: i32 = 1;
    pub const CANCEL_OPTION: i32 = 2;
    pub const OK_OPTION: i32 = 0;
    pub const CLOSED_OPTION: i32 = -1;

    pub const ERROR_MESSAGE: i32 = 0;
    pub const INFORMATION_MESSAGE: i32 = 1;
    pub const WARNING_MESSAGE: i32 = 2;
    pub const QUESTION_MESSAGE: i32 = 3;
    pub const PLAIN_MESSAGE: i32 = -1;

    /// Display a simple message dialog
    /// Equivalent to Java's showMessageDialog(parentComponent, message)
    pub fn show_message_dialog(parent_component: Option<&str>, message: &str) {
        if crate::main::has_gui() {
            #[cfg(feature = "gui")]
            {
                // TODO: Implement actual GUI dialog using egui
                log::info!("Dialog: {}", message);
            }
            #[cfg(not(feature = "gui"))]
            {
                log::info!("Dialog: {}", message);
            }
        } else {
            info!("{}", message);
        }
    }

    /// Display a message dialog with title and message type
    /// Equivalent to Java's showMessageDialog(parentComponent, message, title, messageType)
    pub fn show_message_dialog_with_type(
        parent_component: Option<&str>,
        message: &str,
        title: &str,
        message_type: i32,
    ) {
        if crate::main::has_gui() {
            #[cfg(feature = "gui")]
            {
                // TODO: Implement actual GUI dialog using egui
                log::info!("Dialog [{}]: {}", title, message);
            }
            #[cfg(not(feature = "gui"))]
            {
                log::info!("Dialog [{}]: {}", title, message);
            }
        } else {
            let log_message = format!("{}:{}", title, message);
            match message_type {
                Self::ERROR_MESSAGE => error!("{}", log_message),
                Self::WARNING_MESSAGE => warn!("{}", log_message),
                _ => info!("{}", log_message),
            }
        }
    }

    /// Display a confirmation dialog
    /// Equivalent to Java's showConfirmDialog(parentComponent, message, title, optionType)
    pub fn show_confirm_dialog(
        parent_component: Option<&str>,
        message: &str,
        title: &str,
        option_type: i32,
    ) -> i32 {
        if crate::main::has_gui() {
            #[cfg(feature = "gui")]
            {
                // TODO: Implement actual GUI confirmation dialog using egui
                log::info!("Confirm dialog [{}]: {}", title, message);
                Self::OK_OPTION
            }
            #[cfg(not(feature = "gui"))]
            {
                log::info!("Confirm dialog [{}]: {}", title, message);
                Self::OK_OPTION
            }
        } else {
            Self::CANCEL_OPTION
        }
    }

    /// Display an input dialog
    /// Equivalent to Java's showInputDialog(message)
    pub fn show_input_dialog(message: &str) -> Option<String> {
        if crate::main::has_gui() {
            #[cfg(feature = "gui")]
            {
                // TODO: Implement actual GUI input dialog using egui
                log::info!("Input dialog: {}", message);
                None
            }
            #[cfg(not(feature = "gui"))]
            {
                log::info!("Input dialog: {}", message);
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_pane_constants() {
        assert_eq!(OptionPane::YES_OPTION, 0);
        assert_eq!(OptionPane::NO_OPTION, 1);
        assert_eq!(OptionPane::CANCEL_OPTION, 2);
        assert_eq!(OptionPane::OK_OPTION, 0);
        assert_eq!(OptionPane::CLOSED_OPTION, -1);
    }

    #[test]
    fn test_show_message_dialog() {
        // Test that it doesn't panic
        OptionPane::show_message_dialog(None, "Test message");
        OptionPane::show_message_dialog_with_type(
            None,
            "Test",
            "Title",
            OptionPane::INFORMATION_MESSAGE,
        );
    }
}
