#[derive(Default)]
pub struct MessageBox {
    is_open: bool,
    title: String,
    message: String,
}

impl MessageBox {
    pub fn show_if_open(&mut self, ctx: &egui::Context) {
        if !self.is_open {
            return;
        }

        egui::Modal::new(egui::Id::new("message-box")).show(ctx, |ui| {
            ui.set_width(250.0);
            ui.heading(&self.title);
            ui.label(&self.message);
            ui.separator();
            egui::Sides::new().show(
                ui,
                |_| {},
                |ui| {
                    if ui.button("Dang it").clicked() {
                        self.is_open = false;
                    }
                },
            );
        });
    }

    #[cfg_attr(not(target_arch = "wasm32"), expect(dead_code))]
    pub fn open(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.is_open = true;
        self.title = title.into();
        self.message = message.into();
    }
}
