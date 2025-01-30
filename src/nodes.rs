use std::borrow::Cow;

use egui_graph_edit::{InputId, InputParam, InputParamKind, NodeId, OutputId};
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
    Sqrt,
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

    // input
    Position,
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
            Node::Sqrt,
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
            Node::Position,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NodeCategory {
    Noise,
    Transform,
    Math,
    Seed,
    Input,
}

impl egui_graph_edit::CategoryTrait for NodeCategory {
    fn name(&self) -> String {
        format!("{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl Value {
    fn kind(self) -> ValueKind {
        match self {
            Value::F32(_) => ValueKind::F32,
            Value::I32(_) => ValueKind::I32,
            Value::U32(_) => ValueKind::U32,
        }
    }

    fn f32(self) -> Option<f32> {
        match self {
            Value::F32(v) => Some(v),
            _ => None,
        }
    }

    fn i32(self) -> Option<i32> {
        match self {
            Value::I32(v) => Some(v),
            _ => None,
        }
    }

    fn u32(self) -> Option<u32> {
        match self {
            Value::U32(v) => Some(v),
            _ => None,
        }
    }
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
            Node::Sqrt => modifier(graph),
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
            Node::Position => {
                graph.add_output_param(node_id, "X".into(), ValueKind::F32);
                graph.add_output_param(node_id, "Y".into(), ValueKind::F32);
            }
        }
    }

    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<Self::CategoryType> {
        vec![match self {
            Node::Value => NodeCategory::Noise,
            Node::ValueCubic => NodeCategory::Noise,
            Node::Perlin => NodeCategory::Noise,
            Node::Simplex => NodeCategory::Noise,
            Node::OpenSimplex2 => NodeCategory::Noise,
            Node::OpenSimplex2s => NodeCategory::Noise,
            Node::CellValue => NodeCategory::Noise,
            Node::CellDistance => NodeCategory::Noise,
            Node::CellDistanceSq => NodeCategory::Noise,
            Node::Fractal => NodeCategory::Transform,
            Node::Frequency => NodeCategory::Transform,
            Node::TranslateXy => NodeCategory::Transform,
            Node::Abs => NodeCategory::Math,
            Node::Neg => NodeCategory::Math,
            Node::Sqrt => NodeCategory::Math,
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
            Node::Position => NodeCategory::Input,
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
        node_id: egui_graph_edit::NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        let changed = match self {
            Value::F32(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(value).speed(0.05).min_decimals(2))
                        .changed()
                })
                .inner
            }
            Value::U32(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(value)).changed()
                })
                .inner
            }
            Value::I32(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(value)).changed()
                })
                .inner
            }
        };

        if changed {
            vec![NodeEditorResponse::Changed { node_id }]
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NodeEditorResponse {
    Changed { node_id: NodeId },
}

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

pub fn node_to_noise(
    graph: &NodeGraph,
    output_id: OutputId,
) -> Box<dyn noise_functions::Sample<2>> {
    let node_id = graph.outputs[output_id].node;
    let node = &graph.nodes[node_id];

    let output_name = node
        .outputs
        .iter()
        .find_map(|&(ref name, id)| (id == output_id).then_some(name.as_str()))
        .unwrap();

    use noise_functions::Noise;

    let input_param = |name: &str| -> (InputId, &InputParam<ValueKind, Value>) {
        match node.inputs.iter().find(|(n, _)| n == name) {
            Some(&(_, input_id)) => (input_id, &graph.inputs[input_id]),
            None => panic!("can't find input named \"{name}\""),
        }
    };

    let const_input = |name: &str| -> f32 {
        let (_, input) = input_param(name);

        match input.value.f32() {
            Some(value) => value,
            None => panic!(
                "expected \"{name}\" to be a f32 but it is a {:?}",
                input.value.kind()
            ),
        }
    };

    let const_input_u32 = |name: &str| -> u32 {
        let (_, input) = input_param(name);

        match input.value.u32() {
            Some(value) => value,
            None => panic!(
                "expected \"{name}\" to be a u32 but it is a {:?}",
                input.value.kind()
            ),
        }
    };

    let const_input_i32 = |name: &str| -> i32 {
        let (_, input) = input_param(name);

        match input.value.i32() {
            Some(value) => value,
            None => panic!(
                "expected \"{name}\" to be a i32 but it is a {:?}",
                input.value.kind()
            ),
        }
    };

    let input = |name: &str| -> Box<dyn noise_functions::Sample<2>> {
        let (input_id, _) = input_param(name);

        match graph.connection(input_id) {
            Some(output_id) => node_to_noise(graph, output_id),
            None => Box::new(noise_functions::Constant(const_input(name))),
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
            let jitter = input("Jitter");

            Box::new(NoiseFn(move |point: [f32; 2], seed: i32| {
                let jitter = jitter.sample_with_seed(point, seed);
                noise_functions::CellValue { jitter }.sample_with_seed(point, seed)
            }))
        }
        Node::CellDistance => {
            let jitter = input("Jitter");

            Box::new(NoiseFn(move |point: [f32; 2], seed: i32| {
                let jitter = jitter.sample_with_seed(point, seed);
                noise_functions::CellDistance { jitter }.sample_with_seed(point, seed)
            }))
        }
        Node::CellDistanceSq => {
            let jitter = input("Jitter");

            Box::new(NoiseFn(move |point: [f32; 2], seed: i32| {
                let jitter = jitter.sample_with_seed(point, seed);
                noise_functions::CellDistanceSq { jitter }.sample_with_seed(point, seed)
            }))
        }
        Node::Fractal => Box::new(
            input("Noise")
                .fbm(
                    const_input_u32("Octaves"),
                    const_input("Gain"),
                    const_input("Lacunarity"),
                )
                .weighted(const_input("Weighted Strength")),
        ),
        Node::Frequency => Box::new(input("Noise").frequency(input("Frequency"))),
        Node::TranslateXy => Box::new(input("Noise").translate_xy(input("X"), input("Y"))),
        Node::Abs => Box::new(input("Noise").abs()),
        Node::Neg => Box::new(input("Noise").neg()),
        Node::Sqrt => Box::new(input("Noise").sqrt()),
        Node::Ceil => Box::new(input("Noise").ceil()),
        Node::Floor => Box::new(input("Noise").floor()),
        Node::Round => Box::new(input("Noise").round()),
        Node::Add => Box::new(input("Lhs").add(input("Rhs"))),
        Node::Sub => Box::new(input("Lhs").sub(input("Rhs"))),
        Node::Mul => Box::new(input("Lhs").mul(input("Rhs"))),
        Node::Div => Box::new(input("Lhs").div(input("Rhs"))),
        Node::Rem => Box::new(input("Lhs").rem(input("Rhs"))),
        Node::Pow => Box::new(input("Lhs").pow(input("Rhs"))),
        Node::Min => Box::new(input("Lhs").min(input("Rhs"))),
        Node::Max => Box::new(input("Lhs").max(input("Rhs"))),
        Node::Lerp => Box::new(input("A").lerp(input("B"), input("T"))),
        Node::Clamp => Box::new(input("Value").clamp(input("Min"), input("Max"))),
        Node::AddSeed => Box::new(input("Noise").add_seed(const_input_i32("Value"))),
        Node::MulSeed => Box::new(input("Noise").add_seed(const_input_i32("Value"))),
        Node::Position => match output_name {
            "X" => Box::new(NoiseFn(move |point: [f32; 2]| point[0])),
            "Y" => Box::new(NoiseFn(move |point: [f32; 2]| point[1])),
            _ => unreachable!(),
        },
    }
}
