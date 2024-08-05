use std::rc::Rc;

use eframe::egui::{CentralPanel, Context};
use egui::{Align, FontId, Layout, RichText, WindowLevel};

#[derive(Clone)]
pub struct ConfirmDialog {
    is_running: Rc<bool>,
    native_options: eframe::NativeOptions,
}

impl ConfirmDialog {
    pub fn new() -> Self {
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

        ConfirmDialog {
            is_running: Rc::new(true),
            native_options,
        }
    }

    pub fn open(&self, on_choice: impl FnMut(bool) -> () + 'static) {
        let ptr = self.is_running.clone();

        let _ = eframe::run_native(
            "Backup Confirmation",
            self.native_options.clone(),
            Box::new(|_cc| Ok(Box::new(DialogApp::new(on_choice, ptr)))),
        );
    }

    pub fn close(&mut self) {
        *Rc::get_mut(&mut self.is_running).unwrap() = false;
    }
}


struct DialogApp {
    description: String,
    on_choice_callback: Box<dyn FnMut(bool) -> ()>,
    is_running: Rc<bool>,
}

impl DialogApp {
    fn new(on_choice: impl FnMut(bool) -> () + 'static, is_running: Rc<bool>) -> Self {
        Self {
            description: "Do you want to proceed with the backup?".to_string(),
            on_choice_callback: Box::new(on_choice),
            is_running,
        }
    }
}

impl eframe::App for DialogApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !*self.is_running {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label(RichText::new(&self.description).font(FontId::proportional(20.0)));
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::LEFT), |ui| {
                        ui.add_space(10.0);

                        if ui.button("OK").clicked() {
                            (&mut self.on_choice_callback)(true);
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        ui.add_space(10.0);

                        if ui.button("Cancel").clicked() {
                            (&mut self.on_choice_callback)(false);
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            });
        });
    }
}
