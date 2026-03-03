//! Dialog boxes (About, component properties, etc.)

use egui::Context;

/// Show the About dialog.
pub fn show_about(ctx: &Context, open: &mut bool) {
    egui::Window::new("About Logisim-RUST")
        .open(open)
        .resizable(false)
        .show(ctx, |ui| {
            ui.heading("Logisim-RUST");
            ui.label("Version 1.0.0");
            ui.separator();
            ui.label(
                "A Rust rewrite in progress targeting Logisim-Evolution v4.1.0 compatibility.",
            );
            ui.label("A digital circuit simulator for education and design.");
            ui.separator();
            ui.label("Based on Logisim-Evolution (GPL-3.0)");
            ui.hyperlink_to(
                "logisim-evolution/logisim-evolution",
                "https://github.com/logisim-evolution/logisim-evolution",
            );
            ui.separator();
            ui.label("Rust rewrite: Logisim-RUST contributors");
            ui.hyperlink_to(
                "crossplatformdev/Logisim-RUST",
                "https://github.com/crossplatformdev/Logisim-RUST",
            );
        });
}
