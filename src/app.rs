use serde::{Deserialize, Serialize};

use crate::nodes::{node_to_noise, NodeEditor, NodeEditorUserState, NodeKinds};

pub struct App {
    settings: Settings,
    preview_texture: egui::TextureHandle,
    preview_texture_size: usize,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Settings {
    editor: NodeEditor,
    editor_state: NodeEditorUserState,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        App {
            settings: cc
                .storage
                .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
                .unwrap_or_default(),
            preview_texture: cc.egui_ctx.load_texture(
                "preview",
                egui::ColorImage::example(),
                egui::TextureOptions::NEAREST,
            ),
            preview_texture_size: 512,
        }
    }

    fn update_texture(&mut self) {
        let Some(&node_id) = self.settings.editor.selected_nodes.first() else {
            return;
        };

        log::info!("updating texture");
        let noise = node_to_noise(&self.settings.editor.graph, node_id);

        let mut image = Vec::<egui::Color32>::new();
        image.resize(
            self.preview_texture_size * self.preview_texture_size,
            egui::Color32::PLACEHOLDER,
        );

        let scalar = 1.0 / self.preview_texture_size as f32;
        let scalar_times_two = scalar * 2.0;

        for y in 0..self.preview_texture_size {
            for x in 0..self.preview_texture_size {
                let i = y * self.preview_texture_size + x;
                let x = x as f32 * scalar_times_two - 1.0;
                let y = y as f32 * scalar_times_two - 1.0;
                let value = noise.sample_with_seed([x, y], 0);
                let value_01 = value * 0.5 + 0.5;
                let value_255 = (value_01 * 255.0) as u8;
                let color = egui::Color32::from_gray(value_255);
                image[i] = color;
            }
        }

        self.preview_texture.set(
            egui::ColorImage {
                size: [self.preview_texture_size; 2],
                pixels: image,
            },
            egui::TextureOptions::NEAREST,
        );
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = self.settings.editor.draw_graph_editor(
                ui,
                NodeKinds,
                &mut self.settings.editor_state,
                vec![],
            );

            for node_response in response.node_responses {
                match node_response {
                    egui_graph_edit::NodeResponse::SelectNode(_) => self.update_texture(),
                    _ => (),
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                let texture =
                    egui::load::SizedTexture::new(&self.preview_texture, egui::Vec2::splat(512.0));
                ui.image(texture);
                ui.label("preview image goes here");
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
