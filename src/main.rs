#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, ViewportCommand};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_inner_size([400.0, 200.0])
            .with_min_inner_size([400.0, 200.0])
            .with_transparent(true), // To have rounded corners we need transparency

        ..Default::default()
    };
    eframe::run_native(
        "Radar Detection",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    cnfg: bool,
    enable: bool,
    gain_control: bool,
    clutter_removal: bool,
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        custom_window_frame(ctx, "Detection", |ui| {
            ui.horizontal(|ui| {
                // Menu->File
                ui.menu_button("File", |ui| {
                    let _ = ui.button("Open"); //Placeholer for the moment
                    if ui.button("Save").clicked(){
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Screenshot );
                    }; //Placeholer for the moment
                    ui.separator();
                    if ui.button("Close").clicked(){
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    };
                });
                // Menu->Config
                ui.menu_button("Config", |ui| {
                    ui.menu_button("Montion Detection Parameter", |ui| {
                        if ui.button("enable").clicked() {
                            self.enable = true;
                        };
                        if ui.button("gain control").clicked() {
                            self.gain_control = true;
                        };
                    });
                    ui.menu_button("VitalSignsDemo", |ui| {
                        if ui.button("Clutter removal").clicked() {
                            self.clutter_removal = true;
                        };
                    });
                    ui.separator();
                    if ui.button("Config..").clicked() {
                        self.cnfg = true;
                    };
                });
                //Menu->Edit
                ui.menu_button("Edit", |ui| {
                    if ui.button("Copy").clicked(){
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::RequestCopy);
                    }; 
                    if ui.button("Cut").clicked(){
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::RequestCut );
                    };
                    if ui.button("Paste").clicked(){
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::RequestPaste);
                    };
                });
            });
            // Theme control
            ui.vertical(|ui| {
                ui.separator();
                ui.label("theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        //Config window
        if self.cnfg {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("config"),
                egui::ViewportBuilder::default()
                    .with_decorations(false)
                    .with_inner_size([400.0, 200.0])
                    .with_min_inner_size([400.0, 200.0]),
                    |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );
                    // Window Style
                    let panel_frame = egui::Frame::default()
                        .inner_margin(2.0)
                        .fill(ctx.style().visuals.window_fill())
                        .stroke(ctx.style().visuals.widgets.noninteractive.fg_stroke);

                    egui::CentralPanel::default()
                        .frame(panel_frame)
                        .show(ctx, |ui| {
                            // Title Style
                            let app_rect = ui.max_rect();
                            let title_bar_height = 32.0;
                            let title_bar_rect = {
                                let mut rect = app_rect;
                                rect.max.y = rect.min.y + title_bar_height;
                                rect
                            };
                            title_bar_ui(ui, title_bar_rect, "Config");
                            //Window componants
                            ui.horizontal(|ui| {
                                ui.label("");
                                ui.vertical(|ui| {
                                    ui.heading("Montion Detection Parameter");
                                    ui.checkbox(&mut self.enable, "enable");
                                    ui.checkbox(&mut self.gain_control, "gain control");
                                    ui.separator();
                                    ui.heading("VitalSignsDemo");
                                    ui.checkbox(&mut self.clutter_removal, "Clutter removal");
                                });
                            });
                        });
                        //Close Window
                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.cnfg = false;
                    }
                },
            );
        }
    }
}

fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout(), None);
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new(title), Sense::click_and_drag());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.drag_started_by(PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        });
    });
}

/// Show some close/maximize/minimize buttons for the native window.
fn close_maximize_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}
