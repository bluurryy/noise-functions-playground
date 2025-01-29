use std::borrow::Cow;

use egui_graph_edit::InputParamKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodeKind {
    Foo,
    Bar,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Node {
    Foo,
    Bar { value: f32 },
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueKind {
    F32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Value {
    F32(f32),
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
        egui::Color32::RED
    }

    fn name(&self) -> Cow<str> {
        Cow::Borrowed("f32")
    }
}

impl egui_graph_edit::NodeTemplateTrait for NodeKind {
    type NodeData = Node;
    type DataType = ValueKind;
    type ValueType = Value;
    type UserState = NodeEditorUserState;
    type CategoryType = ();

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<str> {
        Cow::Borrowed(match *self {
            NodeKind::Foo => "Foo",
            NodeKind::Bar => "Bar",
        })
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        match *self {
            NodeKind::Foo => Node::Foo,
            NodeKind::Bar => Node::Bar { value: 3.141 },
        }
    }

    fn build_node(
        &self,
        graph: &mut egui_graph_edit::Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: egui_graph_edit::NodeId,
    ) {
        let data = self.user_data(user_state);

        match data {
            Node::Foo => {
                graph.add_output_param(node_id, "value".into(), ValueKind::F32);
            }
            Node::Bar { value } => {
                graph.add_input_param(
                    node_id,
                    "value".into(),
                    ValueKind::F32,
                    Value::F32(value),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            }
        }
    }
}

pub struct NodeKinds;

impl egui_graph_edit::NodeTemplateIter for NodeKinds {
    type Item = NodeKind;

    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![NodeKind::Foo, NodeKind::Bar]
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

pub type NodeEditor =
    egui_graph_edit::GraphEditorState<Node, ValueKind, Value, NodeKind, NodeEditorUserState>;
