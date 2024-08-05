use std::sync::{Arc, Mutex};

use eframe::egui::{CentralPanel, Context};
use egui::{Align, FontId, Layout, RichText, WindowLevel};

#[derive(Clone)]
pub struct ConfirmDialog {
    is_running: Arc<Mutex<bool>>,
}

impl ConfirmDialog {
    pub fn new() -> Self {
        ConfirmDialog {
            is_running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn open(&self, on_choice: impl FnMut(bool, Arc<Mutex<bool>>) -> () + 'static) {
        let ptr = self.is_running.clone();

        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([450.0, 100.0])
                .with_resizable(false)
                .with_decorations(false)
                .with_window_level(WindowLevel::AlwaysOnTop)
                .with_titlebar_shown(false),
            run_and_return: false,
            ..Default::default()
        };

        let _ = eframe::run_native(
            "Backup Confirmation",
            native_options,
            Box::new(|_cc| Ok(Box::new(DialogApp::new(on_choice, ptr)))),
        );
    }

    pub fn close(&self) {
        let mut is_running = (*self.is_running).lock().unwrap();
        *is_running = false;
    }
}


struct DialogApp {
    description: String,
    on_choice_callback: Box<dyn FnMut(bool, Arc<Mutex<bool>>) -> ()>,
    is_running: Arc<Mutex<bool>>,
}

impl DialogApp {
    fn new(on_choice: impl FnMut(bool, Arc<Mutex<bool>>) -> () + 'static, is_running: Arc<Mutex<bool>>) -> Self {
        Self {
            description: "Do you want to proceed with the backup?".to_string(),
            on_choice_callback: Box::new(on_choice),
            is_running,
        }
    }
}

impl eframe::App for DialogApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let is_running = (*self.is_running).lock().unwrap();
        if !(*is_running) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        drop(is_running);

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label(RichText::new(&self.description).font(FontId::proportional(20.0)));
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::LEFT), |ui| {
                        ui.add_space(10.0);

                        if ui.button("OK").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            (&mut self.on_choice_callback)(true, self.is_running.clone());
                        }

                        ui.add_space(10.0);

                        if ui.button("Cancel").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            (&mut self.on_choice_callback)(false, self.is_running.clone());
                        }
                    });
                });
            });
        });
    }
}
