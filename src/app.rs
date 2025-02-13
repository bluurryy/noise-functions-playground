use egui_snarl::{OutPinId, Snarl};
use noise_functions::Sample;
use serde::{Deserialize, Serialize};

use crate::nodes_snarl;

pub struct App {
    settings: Settings,
    preview_texture: egui::TextureHandle,
    last_sampled_node: Option<egui_snarl::NodeId>,
    snarl_viewer: nodes_snarl::Viewer,
}

#[derive(Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Settings {
    snarl: Snarl<nodes_snarl::Node>,
    preview_value_min: f32,
    preview_value_max: f32,
    preview_texture_size: usize,
    preview_texture_scale: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            snarl: Default::default(),
            preview_value_min: -1.0,
            preview_value_max: 1.0,
            preview_texture_size: 256,
            preview_texture_scale: 3.0,
        }
    }
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
            last_sampled_node: None,
            snarl_viewer: nodes_snarl::Viewer::default(),
        }
    }

    fn update_texture_for_selected(&mut self) {
        if let Some(node_id) = self.snarl_viewer.active_node.or(self.last_sampled_node) {
            self.update_texture_for(node_id);
        }
    }

    fn update_texture_for(&mut self, node_id: egui_snarl::NodeId) {
        log::info!("updating texture");

        let graph = &self.settings.snarl;

        let out_pin = OutPinId {
            node: node_id,
            output: 0,
        };

        self.last_sampled_node = Some(node_id);

        let Some(noise) = nodes_snarl::node_to_noise(graph, out_pin) else {
            return;
        };

        let mut image = Vec::<egui::Color32>::new();

        image.resize(
            self.settings.preview_texture_size * self.settings.preview_texture_size,
            egui::Color32::PLACEHOLDER,
        );

        let value_min = self.settings.preview_value_min;
        let value_max = self.settings.preview_value_max;
        let value_delta = value_max - value_min;
        let inv_value_delta = 1.0 / value_delta;
        let value_offset = value_min * inv_value_delta;

        let inv_size = 1.0 / self.settings.preview_texture_size as f32;
        let scalar = inv_size * 2.0;

        for y in 0..self.settings.preview_texture_size {
            for x in 0..self.settings.preview_texture_size {
                let i = y * self.settings.preview_texture_size + x;
                let x = (x as f32 * scalar - 1.0) * self.settings.preview_texture_scale;
                let y = (y as f32 * scalar - 1.0) * self.settings.preview_texture_scale;
                let value = noise.sample_with_seed([x, y], 0);
                let value_01 = value * inv_value_delta - value_offset;
                let value_255 = (value_01 * 255.0) as u8;
                let color = egui::Color32::from_gray(value_255);
                image[i] = color;
            }
        }

        self.preview_texture.set(
            egui::ColorImage {
                size: [self.settings.preview_texture_size; 2],
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
            self.snarl_viewer.show(&mut self.settings.snarl, ui);

            if let Some(node) = self.snarl_viewer.changed() {
                self.update_texture_for(node)
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                attribution(ui);
                egui::warn_if_debug_build(ui);
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                let texture =
                    egui::load::SizedTexture::new(&self.preview_texture, egui::Vec2::splat(512.0));
                ui.image(texture);

                ui.horizontal(|ui| {
                    let mut changed = false;

                    let mut input = |ui: &mut egui::Ui, value: &mut f32| {
                        changed |= ui
                            .add(egui::DragValue::new(value).speed(0.025).fixed_decimals(1))
                            .changed();
                    };

                    input(ui, &mut self.settings.preview_value_max);

                    ui.label("..");

                    input(ui, &mut self.settings.preview_value_min);

                    if changed {
                        self.update_texture_for_selected();
                    }

                    ui.label("Preview Value Range");
                });

                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::DragValue::new(&mut self.settings.preview_texture_size)
                                .speed(10.0)
                                .range(8.0..=512.0),
                        )
                        .changed()
                    {
                        self.update_texture_for_selected();
                    }

                    ui.label("Preview Texture Resolution");
                });

                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::DragValue::new(&mut self.settings.preview_texture_scale)
                                .speed(0.1),
                        )
                        .changed()
                    {
                        self.update_texture_for_selected();
                    }

                    ui.label("Preview Texture Scale");
                });

                // tips
                ui.label("ℹ Right click nodes and links to delete them.");
                ui.label("ℹ Hold left mouse button to pan.");
                ui.label("ℹ Tick a node's checkbox to preview.");
            });
        });
    }
}

fn attribution(ui: &mut egui::Ui) {
    const GIT_VERSION: &str = git_version::git_version!();

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;

        ui.hyperlink_to(
            "node-functions-playground",
            "https://github.com/bluurryy/noise-functions-playground",
        );
        ui.label(" ");
        ui.label(egui::RichText::from(GIT_VERSION).text_style(egui::TextStyle::Monospace));
        ui.label(". ");

        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(", ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(" and ");
        ui.hyperlink_to("egui-snarl", "https://github.com/zakarumych/egui-snarl");
        ui.label(".");
    });
}
