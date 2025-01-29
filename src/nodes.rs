use std::borrow::Cow;

use egui_graph_edit::{InputParamKind, NodeId};
use noise_functions::{NoiseFn, Sample};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Node {
    Value,
    ValueCubic,
    Perlin,
    Simplex,
    OpenSimplex2,
    OpenSimplex2s,
    CellValue,
    CellDistance,
    CellDistanceSq,

    // misc
    Fractal,
    Frequency,

    // translate
    TranslateXy,

    // unary
    Abs,
    Neg,
    Ceil,
    Floor,
    Round,

    // binary
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    Min,
    Max,

    // ternary
    Lerp,
    Clamp,

    // seed
    AddSeed,
    MulSeed,
}

pub struct NodeKinds;

impl egui_graph_edit::NodeTemplateIter for NodeKinds {
    type Item = Node;

    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![
            Node::Value,
            Node::ValueCubic,
            Node::Perlin,
            Node::Simplex,
            Node::OpenSimplex2,
            Node::OpenSimplex2s,
            Node::CellValue,
            Node::CellDistance,
            Node::CellDistanceSq,
            Node::Fractal,
            Node::Frequency,
            Node::TranslateXy,
            Node::Abs,
            Node::Neg,
            Node::Ceil,
            Node::Floor,
            Node::Round,
            Node::Add,
            Node::Sub,
            Node::Mul,
            Node::Div,
            Node::Rem,
            Node::Pow,
            Node::Min,
            Node::Max,
            Node::Lerp,
            Node::Clamp,
            Node::AddSeed,
            Node::MulSeed,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NodeCategory {
    Base,
    Transform,
    Math,
    Seed,
}

impl egui_graph_edit::CategoryTrait for NodeCategory {
    fn name(&self) -> String {
        format!("{self:?}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueKind {
    F32,
    I32,
    U32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Value {
    F32(f32),
    I32(i32),
    U32(u32),
}

impl Default for Value {
    fn default() -> Self {
        Value::F32(0.0)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NodeEditorUserState;

impl egui_graph_edit::DataTypeTrait<NodeEditorUserState> for ValueKind {
    fn data_type_color(&self, _user_state: &mut NodeEditorUserState) -> egui::Color32 {
        match *self {
            ValueKind::F32 => egui::Color32::from_hex("#D91832").unwrap(),
            ValueKind::U32 => egui::Color32::GRAY,
            ValueKind::I32 => egui::Color32::from_hex("#1F9BDB").unwrap(),
        }
    }

    fn name(&self) -> Cow<str> {
        Cow::Borrowed(match *self {
            ValueKind::F32 => "f32",
            ValueKind::U32 => "u32",
            ValueKind::I32 => "i32",
        })
    }
}

impl egui_graph_edit::NodeTemplateTrait for Node {
    type NodeData = Node;
    type DataType = ValueKind;
    type ValueType = Value;
    type UserState = NodeEditorUserState;
    type CategoryType = NodeCategory;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<str> {
        format!("{self:?}").into()
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        *self
        // match *self {
        //     NodeKind::Value => Node::Value,
        //     NodeKind::ValueCubic => Node::ValueCubic,
        //     NodeKind::Perlin => Node::Perlin,
        //     NodeKind::Simplex => Node::Simplex,
        //     NodeKind::OpenSimplex2 => Node::OpenSimplex2,
        //     NodeKind::OpenSimplex2s => Node::OpenSimplex2s,
        //     NodeKind::CellValue => Node::CellValue { jitter: 1.0 },
        //     NodeKind::CellDistance => Node::CellDistance { jitter: 1.0 },
        //     NodeKind::CellDistanceSq => Node::CellDistanceSq { jitter: 1.0 },

        //     NodeKind::Fractal => Node::Fractal {
        //         lacunarity: 2.0,
        //         octaves: 3,
        //         gain: 0.5,
        //         weighted_strength: 0.0,
        //     },
        //     NodeKind::Frequency => Node::Frequency { frequency: 1.0 },

        //     NodeKind::TranslateXy => Node::TranslateXy { x: 0.0, y: 0.0 },
        //     NodeKind::Abs => Node::Abs,
        //     NodeKind::Neg => Node::Neg,
        //     NodeKind::Ceil => Node::Ceil,
        //     NodeKind::Floor => Node::Floor,
        //     NodeKind::Round => Node::Round,
        //     NodeKind::Add => Node::Add { lhs: 0.0, rhs: 0.0 },
        //     NodeKind::Sub => Node::Sub { lhs: 0.0, rhs: 0.0 },
        //     NodeKind::Mul => Node::Mul { lhs: 0.0, rhs: 0.0 },
        //     NodeKind::Div => Node::Div { lhs: 0.0, rhs: 0.0 },
        //     NodeKind::Rem => Node::Rem { lhs: 0.0, rhs: 0.0 },
        //     NodeKind::Pow => Node::Pow { lhs: 0.0, rhs: 0.0 },
        //     NodeKind::Min => Node::Min { lhs: 0.0, rhs: 0.0 },
        //     NodeKind::Max => Node::Max { lhs: 0.0, rhs: 0.0 },
        //     NodeKind::Lerp => Node::Lerp {
        //         a: 0.0,
        //         b: 0.0,
        //         t: 0.0,
        //     },
        //     NodeKind::Clamp => Node::Clamp {
        //         value: 0.0,
        //         min: 0.0,
        //         max: 1.0,
        //     },
        //     NodeKind::AddSeed => Node::AddSeed { seed: 1 },
        //     NodeKind::MulSeed => Node::AddSeed { seed: 10 },
        // }
    }

    fn build_node(
        &self,
        graph: &mut egui_graph_edit::Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: egui_graph_edit::NodeId,
    ) {
        let noise = |graph: &mut NodeGraph| {
            graph.add_output_param(node_id, "Output".into(), ValueKind::F32);
        };

        let cell_noise = |graph: &mut NodeGraph| {
            noise(graph);

            graph.add_input_param(
                node_id,
                "Jitter".into(),
                ValueKind::F32,
                Value::F32(1.0),
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };

        let modifier = |graph: &mut NodeGraph| {
            noise(graph);

            graph.add_input_param(
                node_id,
                "Noise".into(),
                ValueKind::F32,
                Value::F32(0.0),
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };

        let binary = |graph: &mut NodeGraph| {
            noise(graph);

            graph.add_input_param(
                node_id,
                "Lhs".into(),
                ValueKind::F32,
                Value::F32(0.0),
                InputParamKind::ConnectionOrConstant,
                true,
            );

            graph.add_input_param(
                node_id,
                "Rhs".into(),
                ValueKind::F32,
                Value::F32(0.0),
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };

        let seed_arith = |graph: &mut NodeGraph, arg: i32| {
            modifier(graph);

            graph.add_input_param(
                node_id,
                "Value".into(),
                ValueKind::I32,
                Value::I32(arg),
                InputParamKind::ConstantOnly,
                true,
            );
        };

        match *self {
            Node::Value => noise(graph),
            Node::ValueCubic => noise(graph),
            Node::Perlin => noise(graph),
            Node::Simplex => noise(graph),
            Node::OpenSimplex2 => noise(graph),
            Node::OpenSimplex2s => noise(graph),
            Node::CellValue => cell_noise(graph),
            Node::CellDistance => cell_noise(graph),
            Node::CellDistanceSq => cell_noise(graph),

            Node::Fractal => {
                modifier(graph);

                graph.add_input_param(
                    node_id,
                    "Octaves".into(),
                    ValueKind::U32,
                    Value::U32(2),
                    InputParamKind::ConstantOnly,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Lacunarity".into(),
                    ValueKind::F32,
                    Value::F32(3.0),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Gain".into(),
                    ValueKind::F32,
                    Value::F32(0.5),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Weighted Strength".into(),
                    ValueKind::F32,
                    Value::F32(0.0),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::Frequency => {
                modifier(graph);

                graph.add_input_param(
                    node_id,
                    "Frequency".into(),
                    ValueKind::F32,
                    Value::F32(3.0),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::TranslateXy => {
                modifier(graph);

                graph.add_input_param(
                    node_id,
                    "X".into(),
                    ValueKind::F32,
                    Value::F32(0.0),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Y".into(),
                    ValueKind::F32,
                    Value::F32(0.0),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::Abs => modifier(graph),
            Node::Neg => modifier(graph),
            Node::Ceil => modifier(graph),
            Node::Floor => modifier(graph),
            Node::Round => modifier(graph),
            Node::Add => binary(graph),
            Node::Sub => binary(graph),
            Node::Mul => binary(graph),
            Node::Div => binary(graph),
            Node::Rem => binary(graph),
            Node::Pow => binary(graph),
            Node::Min => binary(graph),
            Node::Max => binary(graph),
            Node::Lerp => {
                noise(graph);

                graph.add_input_param(
                    node_id,
                    "A".into(),
                    ValueKind::F32,
                    Value::F32(0.0),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "B".into(),
                    ValueKind::F32,
                    Value::F32(1.0),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "T".into(),
                    ValueKind::F32,
                    Value::F32(0.5),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::Clamp => {
                noise(graph);

                graph.add_input_param(
                    node_id,
                    "Value".into(),
                    ValueKind::F32,
                    Value::F32(0.5),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Min".into(),
                    ValueKind::F32,
                    Value::F32(0.0),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Max".into(),
                    ValueKind::F32,
                    Value::F32(1.0),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::AddSeed => seed_arith(graph, 1),
            Node::MulSeed => seed_arith(graph, 10),
        }
    }

    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<Self::CategoryType> {
        vec![match self {
            Node::Value => NodeCategory::Base,
            Node::ValueCubic => NodeCategory::Base,
            Node::Perlin => NodeCategory::Base,
            Node::Simplex => NodeCategory::Base,
            Node::OpenSimplex2 => NodeCategory::Base,
            Node::OpenSimplex2s => NodeCategory::Base,
            Node::CellValue => NodeCategory::Base,
            Node::CellDistance => NodeCategory::Base,
            Node::CellDistanceSq => NodeCategory::Base,
            Node::Fractal => NodeCategory::Transform,
            Node::Frequency => NodeCategory::Transform,
            Node::TranslateXy => NodeCategory::Transform,
            Node::Abs => NodeCategory::Math,
            Node::Neg => NodeCategory::Math,
            Node::Ceil => NodeCategory::Math,
            Node::Floor => NodeCategory::Math,
            Node::Round => NodeCategory::Math,
            Node::Add => NodeCategory::Math,
            Node::Sub => NodeCategory::Math,
            Node::Mul => NodeCategory::Math,
            Node::Div => NodeCategory::Math,
            Node::Rem => NodeCategory::Math,
            Node::Pow => NodeCategory::Math,
            Node::Min => NodeCategory::Math,
            Node::Max => NodeCategory::Math,
            Node::Lerp => NodeCategory::Math,
            Node::Clamp => NodeCategory::Math,
            Node::AddSeed => NodeCategory::Seed,
            Node::MulSeed => NodeCategory::Seed,
        }]
    }
}

impl egui_graph_edit::WidgetValueTrait for Value {
    type Response = NodeEditorResponse;
    type UserState = NodeEditorUserState;
    type NodeData = Node;

    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: egui_graph_edit::NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        match self {
            Value::F32(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(value).speed(0.05).min_decimals(2));
                });
            }
            Value::U32(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(value));
                });
            }
            Value::I32(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(value));
                });
            }
        }

        vec![]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NodeEditorResponse;

impl egui_graph_edit::UserResponseTrait for NodeEditorResponse {}

impl egui_graph_edit::NodeDataTrait for Node {
    type Response = NodeEditorResponse;
    type UserState = NodeEditorUserState;
    type DataType = ValueKind;
    type ValueType = Value;

    fn bottom_ui(
        &self,
        _ui: &mut egui::Ui,
        _node_id: egui_graph_edit::NodeId,
        _graph: &egui_graph_edit::Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<egui_graph_edit::NodeResponse<Self::Response, Self>>
    where
        Self::Response: egui_graph_edit::UserResponseTrait,
    {
        vec![]
    }
}

type NodeGraph = egui_graph_edit::Graph<Node, ValueKind, Value>;
pub type NodeEditor =
    egui_graph_edit::GraphEditorState<Node, ValueKind, Value, Node, NodeEditorUserState>;

pub fn node_to_noise(graph: &NodeGraph, node: NodeId) -> Box<dyn noise_functions::Sample<2>> {
    let node = &graph.nodes[node];
    use noise_functions::Noise;

    let const_input = |i: usize| -> f32 {
        let input_id = node.inputs[i].1;

        match graph.inputs[input_id].value {
            Value::F32(value) => value,
            Value::I32(value) => value as f32,
            Value::U32(value) => value as f32,
        }
    };

    let const_input_u32 = |i: usize| -> u32 {
        let input_id = node.inputs[i].1;

        match graph.inputs[input_id].value {
            Value::F32(value) => value as u32,
            Value::I32(value) => value as u32,
            Value::U32(value) => value,
        }
    };

    let const_input_i32 = |i: usize| -> i32 {
        let input_id = node.inputs[i].1;

        match graph.inputs[input_id].value {
            Value::F32(value) => value as i32,
            Value::I32(value) => value,
            Value::U32(value) => value as i32,
        }
    };

    let input = |i: usize| -> Box<dyn noise_functions::Sample<2>> {
        let input_id = node.inputs[i].1;

        match graph.connection(input_id) {
            Some(output_id) => node_to_noise(graph, graph.outputs[output_id].node),
            None => Box::new(noise_functions::Constant(const_input(i))),
        }
    };

    match node.user_data {
        Node::Value => Box::new(noise_functions::Value),
        Node::ValueCubic => Box::new(noise_functions::ValueCubic),
        Node::Perlin => Box::new(noise_functions::Perlin),
        Node::Simplex => Box::new(noise_functions::Simplex),
        Node::OpenSimplex2 => Box::new(noise_functions::OpenSimplex2),
        Node::OpenSimplex2s => Box::new(noise_functions::OpenSimplex2s),
        Node::CellValue => {
            let jitter = input(0);

            Box::new(NoiseFn(move |point: [f32; 2], seed: i32| {
                let jitter = jitter.sample_with_seed(point, seed);
                noise_functions::CellValue { jitter }.sample_with_seed(point, seed)
            }))
        }
        Node::CellDistance => {
            let jitter = input(0);

            Box::new(NoiseFn(move |point: [f32; 2], seed: i32| {
                let jitter = jitter.sample_with_seed(point, seed);
                noise_functions::CellDistance { jitter }.sample_with_seed(point, seed)
            }))
        }
        Node::CellDistanceSq => {
            let jitter = input(0);

            Box::new(NoiseFn(move |point: [f32; 2], seed: i32| {
                let jitter = jitter.sample_with_seed(point, seed);
                noise_functions::CellDistanceSq { jitter }.sample_with_seed(point, seed)
            }))
        }
        Node::Fractal => Box::new(
            input(0)
                .fbm(const_input_u32(1), const_input(2), const_input(3))
                .weighted(const_input(4)),
        ),
        Node::Frequency => Box::new(input(0).frequency(input(1))),
        Node::TranslateXy => Box::new(input(0).translate_xy(input(1), input(2))),
        Node::Abs => Box::new(input(0).abs()),
        Node::Neg => Box::new(input(0).neg()),
        Node::Ceil => Box::new(input(0).ceil()),
        Node::Floor => Box::new(input(0).floor()),
        Node::Round => Box::new(input(0).round()),
        Node::Add => Box::new(input(0).add(input(1))),
        Node::Sub => Box::new(input(0).sub(input(1))),
        Node::Mul => Box::new(input(0).mul(input(1))),
        Node::Div => Box::new(input(0).div(input(1))),
        Node::Rem => Box::new(input(0).rem(input(1))),
        Node::Pow => Box::new(input(0).pow(input(1))),
        Node::Min => Box::new(input(0).min(input(1))),
        Node::Max => Box::new(input(0).max(input(1))),
        Node::Lerp => Box::new(input(0).lerp(input(1), input(2))),
        Node::Clamp => Box::new(input(0).clamp(input(1), input(2))),
        Node::AddSeed => Box::new(input(0).add_seed(const_input_i32(1))),
        Node::MulSeed => Box::new(input(0).add_seed(const_input_i32(1))),
    }
}
