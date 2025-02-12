#![warn(clippy::all)]
#![allow(
    clippy::single_match,
    clippy::collapsible_if,
    clippy::collapsible_else_if
)]

mod app;
mod nodes_graph_edit;
mod nodes_snarl;
pub use app::App;
