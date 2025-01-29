use std::borrow::Cow;

use egui_graph_edit::InputParamKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeKind {
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

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
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

    Fractal {
        lacunarity: f32,
        octaves: u32,
        gain: f32,
        weighted_strength: f32,
    },
    Frequency {
        frequency: f32,
    },

    TranslateXy {
        x: f32,
        y: f32,
    },

    Abs,
    Neg,
    Ceil,
    Floor,
    Round,

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

    AddSeed {
        seed: i32,
    },
    MulSeed {
        seed: i32,
    },
}

pub struct NodeKinds;

impl egui_graph_edit::NodeTemplateIter for NodeKinds {
    type Item = NodeKind;

    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![
            NodeKind::Value,
            NodeKind::ValueCubic,
            NodeKind::Perlin,
            NodeKind::Simplex,
            NodeKind::OpenSimplex2,
            NodeKind::OpenSimplex2s,
            NodeKind::CellValue,
            NodeKind::CellDistance,
            NodeKind::CellDistanceSq,
            NodeKind::Fractal,
            NodeKind::Frequency,
            NodeKind::TranslateXy,
            NodeKind::Abs,
            NodeKind::Neg,
            NodeKind::Ceil,
            NodeKind::Floor,
            NodeKind::Round,
            NodeKind::Add,
            NodeKind::Sub,
            NodeKind::Mul,
            NodeKind::Div,
            NodeKind::Rem,
            NodeKind::Pow,
            NodeKind::Min,
            NodeKind::Max,
            NodeKind::Lerp,
            NodeKind::Clamp,
            NodeKind::AddSeed,
            NodeKind::MulSeed,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl egui_graph_edit::NodeTemplateTrait for NodeKind {
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
        match *self {
            NodeKind::Value => Node::Value,
            NodeKind::ValueCubic => Node::ValueCubic,
            NodeKind::Perlin => Node::Perlin,
            NodeKind::Simplex => Node::Simplex,
            NodeKind::OpenSimplex2 => Node::OpenSimplex2,
            NodeKind::OpenSimplex2s => Node::OpenSimplex2s,
            NodeKind::CellValue => Node::CellValue { jitter: 1.0 },
            NodeKind::CellDistance => Node::CellDistance { jitter: 1.0 },
            NodeKind::CellDistanceSq => Node::CellDistanceSq { jitter: 1.0 },

            NodeKind::Fractal => Node::Fractal {
                lacunarity: 2.0,
                octaves: 3,
                gain: 0.5,
                weighted_strength: 0.0,
            },
            NodeKind::Frequency => Node::Frequency { frequency: 1.0 },

            NodeKind::TranslateXy => Node::TranslateXy { x: 0.0, y: 0.0 },
            NodeKind::Abs => Node::Abs,
            NodeKind::Neg => Node::Neg,
            NodeKind::Ceil => Node::Ceil,
            NodeKind::Floor => Node::Floor,
            NodeKind::Round => Node::Round,
            NodeKind::Add => Node::Add { lhs: 0.0, rhs: 0.0 },
            NodeKind::Sub => Node::Sub { lhs: 0.0, rhs: 0.0 },
            NodeKind::Mul => Node::Mul { lhs: 0.0, rhs: 0.0 },
            NodeKind::Div => Node::Div { lhs: 0.0, rhs: 0.0 },
            NodeKind::Rem => Node::Rem { lhs: 0.0, rhs: 0.0 },
            NodeKind::Pow => Node::Pow { lhs: 0.0, rhs: 0.0 },
            NodeKind::Min => Node::Min { lhs: 0.0, rhs: 0.0 },
            NodeKind::Max => Node::Max { lhs: 0.0, rhs: 0.0 },
            NodeKind::Lerp => Node::Lerp {
                a: 0.0,
                b: 0.0,
                t: 0.0,
            },
            NodeKind::Clamp => Node::Clamp {
                value: 0.0,
                min: 0.0,
                max: 1.0,
            },
            NodeKind::AddSeed => Node::AddSeed { seed: 1 },
            NodeKind::MulSeed => Node::AddSeed { seed: 10 },
        }
    }

    fn build_node(
        &self,
        graph: &mut egui_graph_edit::Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: egui_graph_edit::NodeId,
    ) {
        let data = self.user_data(user_state);

        let noise = |graph: &mut NodeGraph| {
            graph.add_output_param(node_id, "Output".into(), ValueKind::F32);
        };

        let cell_noise = |graph: &mut NodeGraph, jitter: f32| {
            noise(graph);

            graph.add_input_param(
                node_id,
                "Jitter".into(),
                ValueKind::F32,
                Value::F32(jitter),
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

        let binary = |graph: &mut NodeGraph, lhs: f32, rhs: f32| {
            noise(graph);

            graph.add_input_param(
                node_id,
                "Lhs".into(),
                ValueKind::F32,
                Value::F32(lhs),
                InputParamKind::ConnectionOrConstant,
                true,
            );

            graph.add_input_param(
                node_id,
                "Rhs".into(),
                ValueKind::F32,
                Value::F32(rhs),
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

        match data {
            Node::Value => noise(graph),
            Node::ValueCubic => noise(graph),
            Node::Perlin => noise(graph),
            Node::Simplex => noise(graph),
            Node::OpenSimplex2 => noise(graph),
            Node::OpenSimplex2s => noise(graph),
            Node::CellValue { jitter } => cell_noise(graph, jitter),
            Node::CellDistance { jitter } => cell_noise(graph, jitter),
            Node::CellDistanceSq { jitter } => cell_noise(graph, jitter),

            Node::Fractal {
                lacunarity,
                octaves,
                gain,
                weighted_strength,
            } => {
                modifier(graph);

                graph.add_input_param(
                    node_id,
                    "Octaves".into(),
                    ValueKind::U32,
                    Value::U32(octaves),
                    InputParamKind::ConstantOnly,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Lacunarity".into(),
                    ValueKind::F32,
                    Value::F32(lacunarity),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Gain".into(),
                    ValueKind::F32,
                    Value::F32(gain),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Weighted Strength".into(),
                    ValueKind::F32,
                    Value::F32(weighted_strength),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::Frequency { frequency } => {
                modifier(graph);

                graph.add_input_param(
                    node_id,
                    "Frequency".into(),
                    ValueKind::F32,
                    Value::F32(frequency),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::TranslateXy { x, y } => {
                modifier(graph);

                graph.add_input_param(
                    node_id,
                    "X".into(),
                    ValueKind::F32,
                    Value::F32(x),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Y".into(),
                    ValueKind::F32,
                    Value::F32(y),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::Abs => modifier(graph),
            Node::Neg => modifier(graph),
            Node::Ceil => modifier(graph),
            Node::Floor => modifier(graph),
            Node::Round => modifier(graph),
            Node::Add { lhs, rhs } => binary(graph, lhs, rhs),
            Node::Sub { lhs, rhs } => binary(graph, lhs, rhs),
            Node::Mul { lhs, rhs } => binary(graph, lhs, rhs),
            Node::Div { lhs, rhs } => binary(graph, lhs, rhs),
            Node::Rem { lhs, rhs } => binary(graph, lhs, rhs),
            Node::Pow { lhs, rhs } => binary(graph, lhs, rhs),
            Node::Min { lhs, rhs } => binary(graph, lhs, rhs),
            Node::Max { lhs, rhs } => binary(graph, lhs, rhs),
            Node::Lerp { a, b, t } => {
                noise(graph);

                graph.add_input_param(
                    node_id,
                    "A".into(),
                    ValueKind::F32,
                    Value::F32(a),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "B".into(),
                    ValueKind::F32,
                    Value::F32(b),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "T".into(),
                    ValueKind::F32,
                    Value::F32(t),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::Clamp { value, min, max } => {
                noise(graph);

                graph.add_input_param(
                    node_id,
                    "Value".into(),
                    ValueKind::F32,
                    Value::F32(value),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Min".into(),
                    ValueKind::F32,
                    Value::F32(min),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );

                graph.add_input_param(
                    node_id,
                    "Max".into(),
                    ValueKind::F32,
                    Value::F32(max),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
            Node::AddSeed { seed } => seed_arith(graph, seed),
            Node::MulSeed { seed } => seed_arith(graph, seed),
        }
    }

    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<Self::CategoryType> {
        vec![match self {
            NodeKind::Value => NodeCategory::Base,
            NodeKind::ValueCubic => NodeCategory::Base,
            NodeKind::Perlin => NodeCategory::Base,
            NodeKind::Simplex => NodeCategory::Base,
            NodeKind::OpenSimplex2 => NodeCategory::Base,
            NodeKind::OpenSimplex2s => NodeCategory::Base,
            NodeKind::CellValue => NodeCategory::Base,
            NodeKind::CellDistance => NodeCategory::Base,
            NodeKind::CellDistanceSq => NodeCategory::Base,
            NodeKind::Fractal => NodeCategory::Transform,
            NodeKind::Frequency => NodeCategory::Transform,
            NodeKind::TranslateXy => NodeCategory::Transform,
            NodeKind::Abs => NodeCategory::Math,
            NodeKind::Neg => NodeCategory::Math,
            NodeKind::Ceil => NodeCategory::Math,
            NodeKind::Floor => NodeCategory::Math,
            NodeKind::Round => NodeCategory::Math,
            NodeKind::Add => NodeCategory::Math,
            NodeKind::Sub => NodeCategory::Math,
            NodeKind::Mul => NodeCategory::Math,
            NodeKind::Div => NodeCategory::Math,
            NodeKind::Rem => NodeCategory::Math,
            NodeKind::Pow => NodeCategory::Math,
            NodeKind::Min => NodeCategory::Math,
            NodeKind::Max => NodeCategory::Math,
            NodeKind::Lerp => NodeCategory::Math,
            NodeKind::Clamp => NodeCategory::Math,
            NodeKind::AddSeed => NodeCategory::Seed,
            NodeKind::MulSeed => NodeCategory::Seed,
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
    egui_graph_edit::GraphEditorState<Node, ValueKind, Value, NodeKind, NodeEditorUserState>;
