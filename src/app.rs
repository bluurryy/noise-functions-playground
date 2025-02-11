use egui_graph_edit::{InputId, NodeResponse};
use egui_snarl::{OutPinId, Snarl};
use noise_functions::Sample;
use serde::{Deserialize, Serialize};

use crate::{
    nodes::{node_to_noise, NodeEditor, NodeEditorResponse, NodeEditorUserState, NodeKinds},
    nodes_snarl,
};

pub struct App {
    settings: Settings,
    preview_value_min: f32,
    preview_value_max: f32,
    preview_texture: egui::TextureHandle,
    preview_texture_size: usize,
    preview_texture_scale: f32,
    last_sampled_node: Option<egui_graph_edit::NodeId>,
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

trait NodeId {
    fn get_noise(self, app: &mut App) -> Option<Box<dyn Sample<2>>>;
}

impl NodeId for egui_graph_edit::NodeId {
    fn get_noise(self, app: &mut App) -> Option<Box<dyn Sample<2>>> {
        let graph = &app.settings.editor.graph;

        let output_id = graph
            .nodes
            .get(self)
            .and_then(|n| n.outputs.first())
            .map(|o| o.1)?;

        app.last_sampled_node = Some(self);

        Some(node_to_noise(graph, output_id))
    }
}

impl NodeId for egui_snarl::NodeId {
    fn get_noise(self, app: &mut App) -> Option<Box<dyn Sample<2>>> {
        let graph = &app.settings.snarl;

        let out_pin = OutPinId {
            node: self,
            output: 0,
        };

        app.last_sampled_node_snarl = Some(self);

        nodes_snarl::node_to_noise(graph, out_pin)
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
            preview_value_min: -1.0,
            preview_value_max: 1.0,
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

    fn set_node_active(&mut self, node_id: egui_graph_edit::NodeId) {
        if !self.settings.editor.graph.nodes.contains_key(node_id) {
            return;
        }

        self.settings.editor.selected_nodes.clear();
        self.settings.editor.selected_nodes.push(node_id);
        self.update_texture_for(node_id);
    }

    fn update_texture_for_selected(&mut self) {
        if self.alt {
            if let Some(node_id) = self.viewer.active_node.or(self.last_sampled_node_snarl) {
                self.update_texture_for(node_id);
            }
        } else {
            if let Some(node_id) = self
                .settings
                .editor
                .selected_nodes
                .first()
                .copied()
                .or(self.last_sampled_node)
            {
                self.update_texture_for(node_id);
            }
        }
    }

    fn update_texture_for(&mut self, node_id: impl NodeId) {
        log::info!("updating texture");

        let Some(noise) = node_id.get_noise(self) else {
            return;
        };

        let mut image = Vec::<egui::Color32>::new();

        image.resize(
            self.preview_texture_size * self.preview_texture_size,
            egui::Color32::PLACEHOLDER,
        );

        let value_min = self.preview_value_min;
        let value_max = self.preview_value_max;
        let value_delta = value_max - value_min;
        let inv_value_delta = 1.0 / value_delta;
        let value_offset = value_min * inv_value_delta;

        let inv_size = 1.0 / self.preview_texture_size as f32;
        let scalar = inv_size * 2.0;

        for y in 0..self.preview_texture_size {
            for x in 0..self.preview_texture_size {
                let i = y * self.preview_texture_size + x;
                let x = (x as f32 * scalar - 1.0) * self.preview_texture_scale;
                let y = (y as f32 * scalar - 1.0) * self.preview_texture_scale;
                let value = noise.sample_with_seed([x, y], 0);
                let value_01 = value * inv_value_delta - value_offset;
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
            if self.alt {
                self.viewer.show(&mut self.settings.snarl, ui);

                if let Some(node) = self.viewer.changed() {
                    self.update_texture_for(node)
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
                    let mut changed = false;

                    let mut input = |value: &mut f32| {
                        changed |= ui
                            .add(egui::DragValue::new(value).speed(0.025).fixed_decimals(1))
                            .changed();
                    };

                    input(&mut self.preview_value_max);
                    input(&mut self.preview_value_min);

                    if changed {
                        self.update_texture_for_selected();
                    }

                    ui.label("Preview Value Range");
                });

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

                if self.alt {
                    ui.label("ℹ Press RMB to delete links / nodes.");
                    ui.label("ℹ Hold LMB to pan.");
                    ui.label("ℹ Tick a node's checkbox to preview.");
                } else {
                    ui.label("ℹ Hold MMB to pan.");
                    ui.label("ℹ Click on node to preview.");
                }
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(", ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(", ");
        ui.hyperlink_to(
            "egui-graph-edit",
            "https://github.com/kamirr/egui-graph-edit",
        );
        ui.label(" and ");
        ui.hyperlink_to("egui-snarl", "https://github.com/zakarumych/egui-snarl");
        ui.label(".");
    });
}
