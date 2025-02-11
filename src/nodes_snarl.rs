use egui_snarl::{
    ui::{
        BackgroundPattern, Grid, NodeLayout, PinInfo, PinPlacement, PinShape, SnarlStyle,
        SnarlViewer,
    },
    InPin, InPinId, NodeId, OutPinId, Snarl,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Node {
    Value,
    ValueCubic,
    Perlin,
    Simplex,
    OpenSimplex2,
    OpenSimplex2s,
    CellValue {
        jitter: f32,
    },
    CellDistance {
        jitter: f32,
    },
    CellDistanceSq {
        jitter: f32,
    },

    // misc
    Fractal {
        octaves: u32,
        gain: f32,
        lacunarity: f32,
        weighted_strength: f32,
    },
    Frequency {
        frequency: f32,
    },
    TriangleWave {
        frequency: f32,
    },

    // translate
    TranslateXy {
        x: f32,
        y: f32,
    },

    // unary
    Abs,
    Neg,
    Sqrt,
    Floor,
    Ceil,
    Round,

    // binary
    Add {
        lhs: f32,
        rhs: f32,
    },
    Sub {
        lhs: f32,
        rhs: f32,
    },
    Mul {
        lhs: f32,
        rhs: f32,
    },
    Div {
        lhs: f32,
        rhs: f32,
    },
    Rem {
        lhs: f32,
        rhs: f32,
    },
    Pow {
        lhs: f32,
        rhs: f32,
    },
    Min {
        lhs: f32,
        rhs: f32,
    },
    Max {
        lhs: f32,
        rhs: f32,
    },

    // ternary
    Lerp {
        a: f32,
        b: f32,
        t: f32,
    },
    Clamp {
        value: f32,
        min: f32,
        max: f32,
    },

    // seed
    AddSeed {
        add: i32,
    },
    MulSeed {
        mul: i32,
    },

    // input
    Position,
}

impl Node {}

#[derive(Default)]
pub struct Viewer {
    changed_nodes: egui::ahash::HashSet<NodeId>,
    pub active_node: Option<NodeId>,
    prev_active_node: Option<NodeId>,
}

impl Viewer {
    pub fn show(&mut self, snarl: &mut Snarl<Node>, ui: &mut egui::Ui) {
        self.changed_nodes.clear();
        self.prev_active_node = self.active_node;

        snarl.show(
            self,
            &SnarlStyle {
                node_layout: Some(NodeLayout::Sandwich),
                pin_shape: Some(PinShape::Circle),
                pin_fill: Some(egui::Color32::from_rgb(217, 24, 50)),
                pin_placement: Some(PinPlacement::Edge),
                pin_size: Some(8.0),
                bg_frame: Some(egui::Frame::default()),
                bg_pattern: Some(BackgroundPattern::Grid(Grid::new(
                    egui::Vec2::splat(20.0),
                    0.0,
                ))),
                collapsible: Some(false),
                centering: Some(true),
                wire_width: Some(6.0),
                header_drag_space: Some(egui::Vec2::ZERO),
                ..Default::default()
            },
            "snarl",
            ui,
        );
    }

    pub fn changed(&self) -> Option<NodeId> {
        if self.active_node != self.prev_active_node || !self.changed_nodes.is_empty() {
            self.active_node
        } else {
            None
        }
    }

    pub fn show_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut egui::Ui,
        _scale: f32,
        _src_pins: Option<egui_snarl::ui::AnyPins>,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) {
        const NODES_BY_CATEGORY: &[(&str, &[(&str, Node)])] = &[
            (
                "Noise",
                &[
                    ("Value", Node::Value),
                    ("Value Cubic", Node::ValueCubic),
                    ("Perlin", Node::Perlin),
                    ("Simplex", Node::Simplex),
                    ("OpenSimplex2", Node::OpenSimplex2),
                    ("OpenSimplex2s", Node::OpenSimplex2s),
                    ("Cell Value", Node::CellValue { jitter: 1.0 }),
                    ("Cell Distance", Node::CellDistance { jitter: 1.0 }),
                    (
                        "Cell Distance Squared",
                        Node::CellDistanceSq { jitter: 1.0 },
                    ),
                ],
            ),
            (
                "Transform",
                &[
                    (
                        "Fractal",
                        Node::Fractal {
                            octaves: 3,
                            gain: 0.5,
                            lacunarity: 2.0,
                            weighted_strength: 0.0,
                        },
                    ),
                    ("Frequency", Node::Frequency { frequency: 1.0 }),
                    ("Translate", Node::TranslateXy { x: 0.0, y: 0.0 }),
                ],
            ),
            (
                "Math",
                &[
                    ("Abs", Node::Abs),
                    ("Neg", Node::Neg),
                    ("Sqrt", Node::Sqrt),
                    ("Floor", Node::Floor),
                    ("Ceil", Node::Ceil),
                    ("Round", Node::Round),
                    ("Add", Node::Add { lhs: 0.0, rhs: 0.0 }),
                    ("Sub", Node::Sub { lhs: 0.0, rhs: 0.0 }),
                    ("Mul", Node::Mul { lhs: 1.0, rhs: 1.0 }),
                    ("Div", Node::Mul { lhs: 1.0, rhs: 1.0 }),
                    ("Rem", Node::Mul { lhs: 1.0, rhs: 1.0 }),
                    ("Pow", Node::Pow { lhs: 1.0, rhs: 1.0 }),
                    ("Min", Node::Pow { lhs: 0.0, rhs: 0.0 }),
                    ("Max", Node::Pow { lhs: 0.0, rhs: 0.0 }),
                    (
                        "Lerp",
                        Node::Lerp {
                            a: 0.0,
                            b: 1.0,
                            t: 0.5,
                        },
                    ),
                    (
                        "Clamp",
                        Node::Clamp {
                            value: 0.5,
                            min: 0.0,
                            max: 1.0,
                        },
                    ),
                    ("Triangle Wave", Node::TriangleWave { frequency: 1.0 }),
                ],
            ),
            (
                "Seed",
                &[
                    ("Add Seed", Node::AddSeed { add: 1 }),
                    ("Mul Seed", Node::MulSeed { mul: 1 }),
                ],
            ),
            ("Input", &[("Position", Node::Position)]),
        ];

        ui.label("Add node");

        for &(category, nodes) in NODES_BY_CATEGORY {
            ui.menu_button(category, |ui| {
                for &(name, node) in nodes {
                    if ui.button(name).clicked() {
                        snarl.insert_node(pos, node);
                        ui.close_menu();
                    }
                }
            });
        }
    }
}

impl SnarlViewer<Node> for Viewer {
    fn title(&mut self, node: &Node) -> String {
        match node {
            Node::Value => "Value",
            Node::ValueCubic => "Value Cubic",
            Node::Perlin => "Perlin",
            Node::Simplex => "Simplex",
            Node::OpenSimplex2 => "OpenSimplex2",
            Node::OpenSimplex2s => "OpenSimplex2s",
            Node::CellValue { .. } => "Cell Value",
            Node::CellDistance { .. } => "Cell Distance",
            Node::CellDistanceSq { .. } => "Cell DistanceSq",
            Node::Fractal { .. } => "Fractal",
            Node::Frequency { .. } => "Frequency",
            Node::TriangleWave { .. } => "TriangleWave",
            Node::TranslateXy { .. } => "Translate Xy",
            Node::Abs => "Abs",
            Node::Neg => "Neg",
            Node::Sqrt => "Sqrt",
            Node::Floor => "Floor",
            Node::Ceil => "Ceil",
            Node::Round => "Round",
            Node::Add { .. } => "Add",
            Node::Sub { .. } => "Subtract",
            Node::Mul { .. } => "Multiply",
            Node::Div { .. } => "Divide",
            Node::Rem { .. } => "Modulo",
            Node::Pow { .. } => "Power",
            Node::Min { .. } => "Min",
            Node::Max { .. } => "Max",
            Node::Lerp { .. } => "Lerp",
            Node::Clamp { .. } => "Clamp",
            Node::AddSeed { .. } => "Add Seed",
            Node::MulSeed { .. } => "Multiply Seed",
            Node::Position => "Position",
        }
        .into()
    }

    fn show_header(
        &mut self,
        node: egui_snarl::NodeId,
        _inputs: &[InPin],
        _outputs: &[egui_snarl::OutPin],
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) {
        let mut active = self.active_node == Some(node);

        if ui.add(egui::Checkbox::without_text(&mut active)).changed() {
            if active {
                self.active_node = Some(node);
            } else {
                self.active_node = None;
            }
        }

        ui.add(egui::Label::new(self.title(&snarl[node])).selectable(false));
    }

    fn inputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Value => 0,
            Node::ValueCubic => 0,
            Node::Perlin => 0,
            Node::Simplex => 0,
            Node::OpenSimplex2 => 0,
            Node::OpenSimplex2s => 0,
            Node::CellValue { .. } => 1,
            Node::CellDistance { .. } => 1,
            Node::CellDistanceSq { .. } => 1,
            Node::Fractal { .. } => 5,
            Node::Frequency { .. } => 2,
            Node::TriangleWave { .. } => 2,
            Node::TranslateXy { .. } => 3,
            Node::Abs => 1,
            Node::Neg => 1,
            Node::Sqrt => 1,
            Node::Floor => 1,
            Node::Ceil => 1,
            Node::Round => 1,
            Node::Add { .. } => 2,
            Node::Sub { .. } => 2,
            Node::Mul { .. } => 2,
            Node::Div { .. } => 2,
            Node::Rem { .. } => 2,
            Node::Pow { .. } => 2,
            Node::Min { .. } => 2,
            Node::Max { .. } => 2,
            Node::Lerp { .. } => 3,
            Node::Clamp { .. } => 3,
            Node::AddSeed { .. } => 1,
            Node::MulSeed { .. } => 1,
            Node::Position => 0,
        }
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) -> PinInfo {
        let remote = pin.remotes.first();
        let has_remote = remote.is_some();

        let drag_value = |viewer: &mut Viewer,
                          ui: &mut egui::Ui,
                          name: &str,
                          value: &mut f32,
                          speed: f32|
         -> PinInfo {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(name).selectable(false));

                if !has_remote {
                    if ui.add(egui::DragValue::new(value).speed(speed)).changed() {
                        viewer.changed_nodes.insert(pin.id.node);
                    }
                }
            });

            PinInfo::default()
        };

        let input_jitter = |viewer: &mut Viewer, ui: &mut egui::Ui, value: &mut f32| -> PinInfo {
            drag_value(viewer, ui, "Jitter", value, 0.05)
        };

        let input_binary =
            |viewer: &mut Viewer, ui: &mut egui::Ui, lhs: &mut f32, rhs: &mut f32| -> PinInfo {
                match pin.id.input {
                    0 => {
                        drag_value(viewer, ui, "A", lhs, 0.05);
                    }
                    1 => {
                        drag_value(viewer, ui, "B", rhs, 0.05);
                    }
                    _ => (),
                }

                PinInfo::default()
            };

        let noise = |ui: &mut egui::Ui| -> PinInfo {
            ui.add(egui::Label::new("Noise").selectable(false));
            PinInfo::default()
        };

        match &mut snarl[pin.id.node] {
            Node::Value
            | Node::ValueCubic
            | Node::Perlin
            | Node::Simplex
            | Node::OpenSimplex2
            | Node::OpenSimplex2s
            | Node::Position => unreachable!(),
            Node::Abs | Node::Neg | Node::Sqrt | Node::Floor | Node::Ceil | Node::Round => {
                noise(ui)
            }
            Node::CellValue { jitter }
            | Node::CellDistance { jitter }
            | Node::CellDistanceSq { jitter } => input_jitter(self, ui, jitter),
            Node::Fractal {
                octaves,
                gain,
                lacunarity,
                weighted_strength,
            } => {
                match pin.id.input {
                    0 => {
                        noise(ui);
                    }
                    1 => {
                        ui.horizontal(|ui| {
                            ui.add(egui::Label::new("Octaves").selectable(false));
                            if ui.add(egui::DragValue::new(octaves)).changed() {
                                self.changed_nodes.insert(pin.id.node);
                            }
                        });
                    }
                    2 => {
                        ui.horizontal(|ui| {
                            ui.add(egui::Label::new("Gain").selectable(false));
                            if ui.add(egui::DragValue::new(gain).speed(0.05)).changed() {
                                self.changed_nodes.insert(pin.id.node);
                            }
                        });
                    }
                    3 => {
                        ui.horizontal(|ui| {
                            ui.add(egui::Label::new("Lacunarity").selectable(false));
                            if ui
                                .add(egui::DragValue::new(lacunarity).speed(0.05))
                                .changed()
                            {
                                self.changed_nodes.insert(pin.id.node);
                            }
                        });
                    }
                    4 => {
                        ui.horizontal(|ui| {
                            ui.add(egui::Label::new("Weighted Strength").selectable(false));
                            if ui
                                .add(egui::DragValue::new(weighted_strength).speed(0.05))
                                .changed()
                            {
                                self.changed_nodes.insert(pin.id.node);
                            }
                        });
                    }
                    _ => (),
                }

                PinInfo::default()
            }
            Node::Frequency { frequency } | Node::TriangleWave { frequency } => {
                match pin.id.input {
                    0 => {
                        ui.add(egui::Label::new("Noise").selectable(false));
                    }
                    1 => {
                        drag_value(self, ui, "Frequency", frequency, 0.05);
                    }
                    _ => (),
                }

                PinInfo::default()
            }
            Node::TranslateXy { x, y } => {
                match pin.id.input {
                    0 => {
                        noise(ui);
                    }
                    1 => {
                        drag_value(self, ui, "X", x, 0.05);
                    }
                    2 => {
                        drag_value(self, ui, "Y", y, 0.05);
                    }
                    _ => (),
                }

                PinInfo::default()
            }
            Node::Add { lhs, rhs } => input_binary(self, ui, lhs, rhs),
            Node::Sub { lhs, rhs } => input_binary(self, ui, lhs, rhs),
            Node::Mul { lhs, rhs } => input_binary(self, ui, lhs, rhs),
            Node::Div { lhs, rhs } => input_binary(self, ui, lhs, rhs),
            Node::Rem { lhs, rhs } => input_binary(self, ui, lhs, rhs),
            Node::Pow { lhs, rhs } => input_binary(self, ui, lhs, rhs),
            Node::Min { lhs, rhs } => input_binary(self, ui, lhs, rhs),
            Node::Max { lhs, rhs } => input_binary(self, ui, lhs, rhs),
            Node::Lerp { a, b, t } => {
                match pin.id.input {
                    0 => {
                        drag_value(self, ui, "A", a, 0.05);
                    }
                    1 => {
                        drag_value(self, ui, "B", b, 0.05);
                    }
                    2 => {
                        drag_value(self, ui, "T", t, 0.05);
                    }
                    _ => (),
                }

                PinInfo::default()
            }
            Node::Clamp { value, min, max } => {
                match pin.id.input {
                    0 => {
                        drag_value(self, ui, "Value", value, 0.05);
                    }
                    1 => {
                        drag_value(self, ui, "Min", min, 0.05);
                    }
                    2 => {
                        drag_value(self, ui, "Max", max, 0.05);
                    }
                    _ => (),
                }

                PinInfo::default()
            }
            Node::AddSeed { add: value } | Node::MulSeed { mul: value } => {
                if ui.add(egui::DragValue::new(value).speed(1)).changed() {
                    self.changed_nodes.insert(pin.id.node);
                }

                PinInfo::default()
            }
        }
    }

    fn outputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Value
            | Node::ValueCubic
            | Node::Perlin
            | Node::Simplex
            | Node::OpenSimplex2
            | Node::OpenSimplex2s
            | Node::CellValue { .. }
            | Node::CellDistance { .. }
            | Node::CellDistanceSq { .. }
            | Node::Fractal { .. }
            | Node::Frequency { .. }
            | Node::TriangleWave { .. }
            | Node::TranslateXy { .. }
            | Node::Abs
            | Node::Neg
            | Node::Sqrt
            | Node::Floor
            | Node::Ceil
            | Node::Round
            | Node::Add { .. }
            | Node::Sub { .. }
            | Node::Mul { .. }
            | Node::Div { .. }
            | Node::Rem { .. }
            | Node::Pow { .. }
            | Node::Min { .. }
            | Node::Max { .. }
            | Node::Lerp { .. }
            | Node::Clamp { .. }
            | Node::AddSeed { .. }
            | Node::MulSeed { .. } => 1,
            Node::Position => 2,
        }
    }

    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) -> PinInfo {
        if pin.id.output == 0 {
            ui.add_space(10.0);
        }

        match snarl[pin.id.node] {
            Node::Value
            | Node::ValueCubic
            | Node::Perlin
            | Node::Simplex
            | Node::OpenSimplex2
            | Node::OpenSimplex2s
            | Node::CellValue { .. }
            | Node::CellDistance { .. }
            | Node::CellDistanceSq { .. }
            | Node::Fractal { .. }
            | Node::Frequency { .. }
            | Node::TriangleWave { .. }
            | Node::TranslateXy { .. }
            | Node::Abs
            | Node::Neg
            | Node::Sqrt
            | Node::Floor
            | Node::Ceil
            | Node::Round
            | Node::Add { .. }
            | Node::Sub { .. }
            | Node::Mul { .. }
            | Node::Div { .. }
            | Node::Rem { .. }
            | Node::Pow { .. }
            | Node::Min { .. }
            | Node::Max { .. }
            | Node::Lerp { .. }
            | Node::Clamp { .. }
            | Node::AddSeed { .. }
            | Node::MulSeed { .. } => {
                ui.add(egui::Label::new("Output").selectable(false));
                PinInfo::default()
            }
            Node::Position => {
                match pin.id.output {
                    0 => {
                        ui.add(egui::Label::new("X").selectable(false));
                    }
                    1 => {
                        ui.add(egui::Label::new("Y").selectable(false));
                    }
                    _ => (),
                }

                PinInfo::default()
            }
        }
    }

    fn connect(&mut self, from: &egui_snarl::OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        self.changed_nodes.insert(to.id.node);

        snarl.drop_inputs(to.id);
        snarl.connect(from.id, to.id);
    }

    fn disconnect(&mut self, from: &egui_snarl::OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        self.changed_nodes.insert(to.id.node);
        snarl.disconnect(from.id, to.id);
    }

    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut egui_snarl::Snarl<Node>) -> bool {
        true
    }

    fn show_graph_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut egui::Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) {
        self.show_menu(pos, ui, scale, None, snarl);
    }

    fn has_dropped_wire_menu(
        &mut self,
        _src_pins: egui_snarl::ui::AnyPins,
        _snarl: &mut egui_snarl::Snarl<Node>,
    ) -> bool {
        true
    }

    fn show_dropped_wire_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut egui::Ui,
        scale: f32,
        src_pins: egui_snarl::ui::AnyPins,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) {
        self.show_menu(pos, ui, scale, Some(src_pins), snarl);
    }

    fn has_node_menu(&mut self, _node: &Node) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node: egui_snarl::NodeId,
        _inputs: &[InPin],
        _outputs: &[egui_snarl::OutPin],
        ui: &mut egui::Ui,
        _scale: f32,
        snarl: &mut egui_snarl::Snarl<Node>,
    ) {
        ui.label("Node menu");

        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close_menu();
        }
    }
}

pub fn node_to_noise(
    snarl: &Snarl<Node>,
    pin: OutPinId,
) -> Option<Box<dyn noise_functions::Sample<2>>> {
    use noise_functions::{Constant, Noise, NoiseFn, Sample};

    fn input_or(
        snarl: &Snarl<Node>,
        pin: InPinId,
        default: f32,
    ) -> Option<Box<dyn noise_functions::Sample<2>>> {
        if let Some(&pin) = snarl.in_pin(pin).remotes.first() {
            node_to_noise(snarl, pin)
        } else {
            Some(Box::new(Constant(default)))
        }
    }

    fn cell_noise<N: Sample<2>>(
        snarl: &Snarl<Node>,
        node: NodeId,
        default_jitter: f32,
        f: impl Fn(f32) -> N + 'static,
    ) -> Option<Box<dyn noise_functions::Sample<2>>> {
        let pin = InPinId { node, input: 0 };
        let jitter = input_or(snarl, pin, default_jitter)?;

        Some(Box::new(NoiseFn(move |point: [f32; 2], seed: i32| {
            let jitter = jitter.sample_with_seed(point, seed);
            f(jitter).sample_with_seed(point, seed)
        })))
    }

    let input_or = |i: usize, default: f32| -> Option<Box<dyn noise_functions::Sample<2>>> {
        let pin = InPinId {
            node: pin.node,
            input: i,
        };

        if let Some(&pin) = snarl.in_pin(pin).remotes.first() {
            node_to_noise(snarl, pin)
        } else {
            Some(Box::new(Constant(default)))
        }
    };

    match snarl[pin.node] {
        Node::Value => Some(Box::new(noise_functions::Value)),
        Node::ValueCubic => Some(Box::new(noise_functions::ValueCubic)),
        Node::Perlin => Some(Box::new(noise_functions::Perlin)),
        Node::Simplex => Some(Box::new(noise_functions::Simplex)),
        Node::OpenSimplex2 => Some(Box::new(noise_functions::OpenSimplex2)),
        Node::OpenSimplex2s => Some(Box::new(noise_functions::OpenSimplex2s)),
        Node::CellValue { jitter } => cell_noise(snarl, pin.node, jitter, |jitter| {
            noise_functions::CellValue { jitter }
        }),
        Node::CellDistance { jitter } => cell_noise(snarl, pin.node, jitter, |jitter| {
            noise_functions::CellDistance { jitter }
        }),
        Node::CellDistanceSq { jitter } => cell_noise(snarl, pin.node, jitter, |jitter| {
            noise_functions::CellDistanceSq { jitter }
        }),
        Node::Fractal {
            octaves,
            gain,
            lacunarity,
            weighted_strength,
        } => Some(Box::new(
            input_or(0, 0.0)?
                .fbm(octaves, gain, lacunarity)
                .weighted(weighted_strength),
        )),
        Node::Frequency { frequency } => Some(Box::new(
            input_or(0, 0.0)?.frequency(input_or(1, frequency)?),
        )),
        Node::TriangleWave { frequency } => Some(Box::new(
            input_or(0, 0.0)?.triangle_wave(input_or(1, frequency)?),
        )),
        Node::TranslateXy { x, y } => Some(Box::new(
            input_or(0, 0.0)?.translate_xy(input_or(1, x)?, input_or(2, y)?),
        )),
        Node::Abs => Some(Box::new(input_or(0, 0.0)?.abs())),
        Node::Neg => Some(Box::new(input_or(0, 0.0)?.neg())),
        Node::Sqrt => Some(Box::new(input_or(0, 0.0)?.sqrt())),
        Node::Floor => Some(Box::new(input_or(0, 0.0)?.floor())),
        Node::Ceil => Some(Box::new(input_or(0, 0.0)?.ceil())),
        Node::Round => Some(Box::new(input_or(0, 0.0)?.round())),
        Node::Add { lhs, rhs } => Some(Box::new(input_or(0, lhs)?.add(input_or(1, rhs)?))),
        Node::Sub { lhs, rhs } => Some(Box::new(input_or(0, lhs)?.sub(input_or(1, rhs)?))),
        Node::Mul { lhs, rhs } => Some(Box::new(input_or(0, lhs)?.mul(input_or(1, rhs)?))),
        Node::Div { lhs, rhs } => Some(Box::new(input_or(0, lhs)?.div(input_or(1, rhs)?))),
        Node::Rem { lhs, rhs } => Some(Box::new(input_or(0, lhs)?.rem(input_or(1, rhs)?))),
        Node::Pow { lhs, rhs } => Some(Box::new(input_or(0, lhs)?.pow(input_or(1, rhs)?))),
        Node::Min { lhs, rhs } => Some(Box::new(input_or(0, lhs)?.min(input_or(1, rhs)?))),
        Node::Max { lhs, rhs } => Some(Box::new(input_or(0, lhs)?.max(input_or(1, rhs)?))),
        Node::Lerp { a, b, t } => Some(Box::new(
            input_or(0, a)?.clamp(input_or(1, b)?, input_or(2, t)?),
        )),
        Node::Clamp { value, min, max } => Some(Box::new(
            input_or(0, value)?.clamp(input_or(1, min)?, input_or(2, max)?),
        )),
        Node::AddSeed { add } => Some(Box::new(input_or(0, 0.0)?.add_seed(add))),
        Node::MulSeed { mul } => Some(Box::new(input_or(0, 0.0)?.mul_seed(mul))),
        Node::Position => match pin.output {
            0 => Some(Box::new(NoiseFn(move |point: [f32; 2]| point[0]))),
            1 => Some(Box::new(NoiseFn(move |point: [f32; 2]| point[1]))),
            _ => None,
        },
    }
}
