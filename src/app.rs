use egui_graph_edit::{InputId, NodeId, NodeResponse};
use egui_snarl::{InPinId, OutPinId, Snarl};
use serde::{Deserialize, Serialize};

use crate::{
    nodes::{node_to_noise, NodeEditor, NodeEditorResponse, NodeEditorUserState, NodeKinds},
    nodes_snarl,
};

pub struct App {
    settings: Settings,
    preview_texture: egui::TextureHandle,
    preview_texture_size: usize,
    preview_texture_scale: f32,
    last_sampled_node: Option<NodeId>,
    last_sampled_node_snarl: Option<egui_snarl::NodeId>,
    alt: bool,
    viewer: nodes_snarl::Viewer,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Settings {
    snarl: Snarl<nodes_snarl::Node>,
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
            preview_texture_size: 256,
            preview_texture_scale: 3.0,
            last_sampled_node: None,
            last_sampled_node_snarl: None,
            alt: false,
            viewer: nodes_snarl::Viewer::default(),
        }
    }

    fn set_input_active(&mut self, input_id: InputId) {
        let Some(input) = self.settings.editor.graph.inputs.get(input_id) else {
            return;
        };

        self.set_node_active(input.node);
    }

    fn set_node_active(&mut self, node_id: NodeId) {
        if !self.settings.editor.graph.nodes.contains_key(node_id) {
            return;
        }

        self.settings.editor.selected_nodes.clear();
        self.settings.editor.selected_nodes.push(node_id);
        self.update_texture_for(node_id);
    }

    fn update_texture_for_selected(&mut self) {
        let Some(node_id) = self
            .settings
            .editor
            .selected_nodes
            .first()
            .copied()
            .or(self.last_sampled_node)
        else {
            return;
        };

        self.update_texture_for(node_id);
    }

    fn update_texture_for(&mut self, node_id: NodeId) {
        let Some(output_id) = self
            .settings
            .editor
            .graph
            .nodes
            .get(node_id)
            .and_then(|n| n.outputs.first())
            .map(|o| o.1)
        else {
            return;
        };

        log::info!("updating texture");
        let noise = node_to_noise(&self.settings.editor.graph, output_id);

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
                let x = (x as f32 * scalar_times_two - 1.0) * self.preview_texture_scale;
                let y = (y as f32 * scalar_times_two - 1.0) * self.preview_texture_scale;
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

        self.last_sampled_node = Some(node_id);
    }

    fn update_texture_for_snarl(&mut self, node: egui_snarl::NodeId) {
        let out_pin = OutPinId { node, output: 0 };

        log::info!("updating texture");
        let noise = nodes_snarl::node_to_noise(&self.settings.snarl, out_pin)
            .expect("failed to build noise generator");

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
                let x = (x as f32 * scalar_times_two - 1.0) * self.preview_texture_scale;
                let y = (y as f32 * scalar_times_two - 1.0) * self.preview_texture_scale;
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

        self.last_sampled_node_snarl = Some(node);
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.alt {
                self.viewer.show(&mut self.settings.snarl, ui);

                if let Some(node) = self.viewer.changed() {
                    self.update_texture_for_snarl(node)
                }
            } else {
                let response = self.settings.editor.draw_graph_editor(
                    ui,
                    NodeKinds,
                    &mut self.settings.editor_state,
                    vec![],
                );

                for node_response in response.node_responses {
                    match node_response {
                        NodeResponse::SelectNode(_) => self.update_texture_for_selected(),
                        NodeResponse::User(NodeEditorResponse::Changed { node_id }) => {
                            self.update_texture_for(node_id)
                        }
                        NodeResponse::CreatedNode(node_id) => self.set_node_active(node_id),
                        NodeResponse::ConnectEventEnded { output: _, input } => {
                            self.set_input_active(input)
                        }
                        NodeResponse::DisconnectEvent { output: _, input } => {
                            self.set_input_active(input)
                        }
                        _ => (),
                    }
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

                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::DragValue::new(&mut self.preview_texture_size)
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
                        .add(egui::DragValue::new(&mut self.preview_texture_scale).speed(0.1))
                        .changed()
                    {
                        self.update_texture_for_selected();
                    }

                    ui.label("Preview Texture Scale");
                });

                ui.horizontal(|ui| {
                    ui.add(egui::Checkbox::without_text(&mut self.alt));
                    ui.label("Use Alternative Node Editor");
                });
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
