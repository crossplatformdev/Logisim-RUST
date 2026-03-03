//! Main application struct implementing [`eframe::App`].

use crate::canvas::CircuitCanvas;
use crate::component_panel::ComponentPanel;
use crate::state::{AppState, Tool};
use crate::toolbar::Toolbar;
use egui::{Context, Modifiers};
use logisim_core::{circuit::Circuit, component::ComponentKind, value::BitWidth};
use logisim_file::{parse_circ, write_circ};
use std::io::BufReader;

/// The root application object.
pub struct LogisimApp {
    state: AppState,
    canvas: CircuitCanvas,
    component_panel: ComponentPanel,
    toolbar: Toolbar,
    /// Pending file dialog result (used with rfd async on native).
    _pending_open: Option<std::path::PathBuf>,
    /// Accumulated simulation time since last tick.
    sim_accumulator: f32,
    /// Whether the About dialog is open.
    about_open: bool,
}

impl LogisimApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut state = AppState::new();

        // Create a default "main" circuit.
        let mut main = Circuit::new("main");
        // Place a few example components.
        main.add_component_with_label(
            ComponentKind::Pin {
                is_output: false,
                width: BitWidth::ONE,
            },
            3,
            3,
            "A",
        );
        main.add_component_with_label(
            ComponentKind::Pin {
                is_output: false,
                width: BitWidth::ONE,
            },
            3,
            5,
            "B",
        );
        main.add_component(
            ComponentKind::AndGate {
                inputs: 2,
                width: BitWidth::ONE,
                negate_inputs: vec![false, false],
                negate_output: false,
            },
            8,
            4,
        );
        main.add_component_with_label(
            ComponentKind::Pin {
                is_output: true,
                width: BitWidth::ONE,
            },
            13,
            4,
            "OUT",
        );
        main.add_wire(3, 3, 8, 3);
        main.add_wire(3, 5, 8, 5);
        main.add_wire(10, 4, 13, 4);

        state.project.add_circuit(main);
        state.active_circuit = state
            .project
            .main_circuit_name()
            .unwrap_or("main")
            .to_string();
        state.sync_simulator();

        LogisimApp {
            state,
            canvas: CircuitCanvas::new(),
            component_panel: ComponentPanel::new(),
            toolbar: Toolbar::new(),
            _pending_open: None,
            sim_accumulator: 0.0,
            about_open: false,
        }
    }

    fn handle_keyboard(&mut self, ctx: &Context) {
        ctx.input_mut(|i| {
            // Ctrl+S / Cmd+S → Save
            if i.consume_key(Modifiers::COMMAND, egui::Key::S) {
                self.save_file();
            }
            // Ctrl+O / Cmd+O → Open
            if i.consume_key(Modifiers::COMMAND, egui::Key::O) {
                self.open_file_dialog();
            }
            // Ctrl+N / Cmd+N → New
            if i.consume_key(Modifiers::COMMAND, egui::Key::N) {
                self.new_project();
            }
            // Delete → Delete selected
            if i.consume_key(Modifiers::NONE, egui::Key::Delete) {
                self.delete_selected();
            }
            // Space → toggle simulation
            if i.consume_key(Modifiers::NONE, egui::Key::Space) {
                self.state.running = !self.state.running;
                self.state.status = if self.state.running {
                    "Simulation running".to_string()
                } else {
                    "Simulation stopped".to_string()
                };
            }
            // Escape → select tool
            if i.consume_key(Modifiers::NONE, egui::Key::Escape) {
                self.state.tool = Tool::Select;
                self.state.wire_start = None;
            }
            // Ctrl+Z / Cmd+Z → Undo
            if i.consume_key(Modifiers::COMMAND, egui::Key::Z) {
                self.undo();
            }
            // Ctrl+Y or Ctrl+Shift+Z / Cmd+Shift+Z → Redo
            if i.consume_key(Modifiers::COMMAND, egui::Key::Y)
                || i.consume_key(Modifiers::COMMAND | Modifiers::SHIFT, egui::Key::Z)
            {
                self.redo();
            }
        });
    }

    fn undo(&mut self) {
        if self.state.history.undo(&mut self.state.project) {
            self.state.modified = true;
            self.state.sync_simulator();
            self.state.status = "Undo".to_string();
        }
    }

    fn redo(&mut self) {
        if self.state.history.redo(&mut self.state.project) {
            self.state.modified = true;
            self.state.sync_simulator();
            self.state.status = "Redo".to_string();
        }
    }

    fn delete_selected(&mut self) {
        let name = self.state.active_circuit.clone();
        if let Some(circuit) = self.state.project.circuits.get(&name) {
            let to_remove: Vec<_> = self
                .state
                .selected
                .iter()
                .filter_map(|id| circuit.components.get(id).map(|c| (*id, c.clone())))
                .collect();
            if to_remove.is_empty() {
                return;
            }
            let actions: Vec<_> = to_remove
                .iter()
                .map(
                    |(id, comp)| logisim_core::history::UndoAction::RemoveComponent {
                        circuit_name: name.clone(),
                        id: *id,
                        component: comp.clone(),
                    },
                )
                .collect();
            if actions.len() == 1 {
                self.state.history.push(actions.into_iter().next().unwrap());
            } else {
                self.state
                    .history
                    .push(logisim_core::history::UndoAction::Batch(actions));
            }
        }
        if let Some(circuit) = self.state.project.circuits.get_mut(&name) {
            for id in self.state.selected.drain(..) {
                circuit.remove_component(id);
            }
        }
        self.state.modified = true;
        self.state.sync_simulator();
    }

    fn new_project(&mut self) {
        let mut state = AppState::new();
        let main = Circuit::new("main");
        state.project.add_circuit(main);
        state.active_circuit = "main".to_string();
        state.sync_simulator();
        self.state = state;
    }

    fn open_file_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Logisim Circuit", &["circ"])
            .pick_file()
        {
            self.load_file(path);
        }
    }

    fn load_file(&mut self, path: std::path::PathBuf) {
        match std::fs::File::open(&path) {
            Ok(f) => {
                let reader = BufReader::new(f);
                match parse_circ(reader) {
                    Ok(project) => {
                        let main = project.main_circuit_name().unwrap_or("").to_string();
                        self.state.project = project;
                        self.state.active_circuit = main;
                        self.state.file_path = Some(path);
                        self.state.modified = false;
                        self.state.history.clear();
                        // Reset editor state so stale selections/tool from the
                        // previous project don't bleed into the new one.
                        self.state.selected.clear();
                        self.state.wire_start = None;
                        self.state.tool = crate::state::Tool::Select;
                        self.state.running = false;
                        self.state.sync_simulator();
                        self.state.status = "File opened successfully.".to_string();
                    }
                    Err(e) => {
                        self.state.status = format!("Error opening file: {}", e);
                    }
                }
            }
            Err(e) => {
                self.state.status = format!("Cannot open file: {}", e);
            }
        }
    }

    fn save_file(&mut self) {
        let path = if let Some(p) = self.state.file_path.clone() {
            p
        } else if let Some(p) = rfd::FileDialog::new()
            .add_filter("Logisim Circuit", &["circ"])
            .save_file()
        {
            p
        } else {
            return;
        };

        let mut buf = Vec::new();
        match write_circ(&self.state.project, &mut buf) {
            Ok(()) => {
                if let Err(e) = std::fs::write(&path, &buf) {
                    self.state.status = format!("Error saving file: {}", e);
                } else {
                    self.state.file_path = Some(path);
                    self.state.modified = false;
                    self.state.status = "File saved.".to_string();
                }
            }
            Err(e) => {
                self.state.status = format!("Error serialising: {}", e);
            }
        }
    }

    fn tick_simulation(&mut self, dt: f32) {
        // Handle single-step request (from toolbar Step button).
        if self.state.step_requested {
            self.state.step_requested = false;
            let name = self.state.active_circuit.clone();
            if let Err(e) = self.state.simulator.tick(&name) {
                self.state.status = format!("Simulation error: {}", e);
            }
        }

        if !self.state.running {
            return;
        }
        self.sim_accumulator += dt;
        let period = 1.0 / self.state.sim_hz.max(0.01);
        while self.sim_accumulator >= period {
            self.sim_accumulator -= period;
            let name = self.state.active_circuit.clone();
            if let Err(e) = self.state.simulator.tick(&name) {
                self.state.status = format!("Simulation error: {}", e);
                self.state.running = false;
                break;
            }
        }
    }
}

impl eframe::App for LogisimApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Tick simulation.
        let dt = ctx.input(|i| i.unstable_dt);
        self.tick_simulation(dt);

        // Handle keyboard shortcuts.
        self.handle_keyboard(ctx);

        // Menu bar.
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New       Ctrl+N").clicked() {
                        self.new_project();
                        ui.close_menu();
                    }
                    if ui.button("Open...   Ctrl+O").clicked() {
                        self.open_file_dialog();
                        ui.close_menu();
                    }
                    if ui.button("Save      Ctrl+S").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    if ui.button("Save As...").clicked() {
                        self.state.file_path = None;
                        self.save_file();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    let can_undo = self.state.history.can_undo();
                    let can_redo = self.state.history.can_redo();
                    if ui
                        .add_enabled(can_undo, egui::Button::new("Undo      Ctrl+Z"))
                        .clicked()
                    {
                        self.undo();
                        ui.close_menu();
                    }
                    if ui
                        .add_enabled(can_redo, egui::Button::new("Redo      Ctrl+Y"))
                        .clicked()
                    {
                        self.redo();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Delete    Del").clicked() {
                        self.delete_selected();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Select All").clicked() {
                        let name = self.state.active_circuit.clone();
                        if let Some(circuit) = self.state.project.circuits.get(&name) {
                            self.state.selected = circuit.components.keys().copied().collect();
                        }
                        ui.close_menu();
                    }
                });

                ui.menu_button("Simulate", |ui| {
                    if ui.button("Run / Stop   Space").clicked() {
                        self.state.running = !self.state.running;
                        ui.close_menu();
                    }
                    if ui.button("Step").clicked() {
                        let name = self.state.active_circuit.clone();
                        if let Err(e) = self.state.simulator.tick(&name) {
                            self.state.status = format!("Simulation error: {}", e);
                            self.state.running = false;
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.label("Speed (Hz):");
                    ui.add(
                        egui::Slider::new(&mut self.state.sim_hz, 0.1..=1000.0).logarithmic(true),
                    );
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.state.show_grid, "Show Grid");
                    ui.separator();
                    if ui.button("Zoom In").clicked() {
                        self.state.zoom = (self.state.zoom * 1.25).min(8.0);
                        ui.close_menu();
                    }
                    if ui.button("Zoom Out").clicked() {
                        self.state.zoom = (self.state.zoom / 1.25).max(0.25);
                        ui.close_menu();
                    }
                    if ui.button("Reset Zoom").clicked() {
                        self.state.zoom = 1.0;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About Logisim-RUST").clicked() {
                        self.about_open = true;
                        ui.close_menu();
                    }
                });
            });
        });

        // Toolbar.
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.toolbar.show(ui, &mut self.state);
        });

        // Status bar.
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let modified_mark = if self.state.modified { "*" } else { "" };
                let title = self
                    .state
                    .file_path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Untitled".to_string());
                ui.label(format!(
                    "{}{} | {}",
                    title, modified_mark, self.state.status
                ));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let run_label = if self.state.running {
                        "▶ Running"
                    } else {
                        "⏹ Stopped"
                    };
                    ui.label(run_label);
                    ui.separator();
                    ui.label(format!("Zoom: {:.0}%", self.state.zoom * 100.0));
                });
            });
        });

        // Left panel: component list + attribute table.
        egui::SidePanel::left("component_panel")
            .resizable(true)
            .min_width(150.0)
            .default_width(200.0)
            .show(ctx, |ui| {
                let avail = ui.available_height();
                // Top portion: component palette (default 60% of left panel height,
                // max 70%). Bottom portion: attribute table for the selected component.
                // This matches the upstream Explorer/Attribute split in Logisim-Evolution.
                egui::TopBottomPanel::top("palette_inner")
                    .resizable(true)
                    .min_height(80.0)
                    .max_height(avail * 0.70)
                    .default_height(avail * 0.60)
                    .frame(egui::Frame::none())
                    .show_inside(ui, |ui| {
                        self.component_panel.show(ui, &mut self.state);
                    });
                // Scrollable attribute table for the selected component.
                egui::ScrollArea::vertical()
                    .id_salt("attr_scroll")
                    .show(ui, |ui| {
                        crate::attr_panel::show_attr_panel(ui, &mut self.state);
                    });
            });

        // Right panel: circuit list.
        egui::SidePanel::right("circuit_panel")
            .resizable(true)
            .min_width(120.0)
            .default_width(160.0)
            .show(ctx, |ui| {
                ui.heading("Circuits");
                ui.separator();
                let circuit_names: Vec<String> = self.state.project.circuit_order.clone();
                for name in &circuit_names {
                    let active = name == &self.state.active_circuit;
                    if ui.selectable_label(active, name).clicked() {
                        self.state.active_circuit = name.clone();
                        self.state.selected.clear();
                    }
                }
                ui.separator();
                if ui.button("+ New Circuit").clicked() {
                    let idx = self.state.project.circuits.len() + 1;
                    let new_name = format!("circuit{}", idx);
                    self.state.project.add_circuit(Circuit::new(&new_name));
                    self.state.active_circuit = new_name;
                    self.state.modified = true;
                    self.state.sync_simulator();
                }
            });

        // Central canvas.
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::WHITE))
            .show(ctx, |ui| {
                self.canvas.show(ui, &mut self.state);
            });

        // About dialog.
        crate::dialogs::show_about(ctx, &mut self.about_open);

        // Request continuous repaint when simulating.
        if self.state.running {
            ctx.request_repaint();
        }
    }
}
