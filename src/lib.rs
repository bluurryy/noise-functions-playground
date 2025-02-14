#![warn(clippy::all)]
#![allow(
    clippy::single_match,
    clippy::collapsible_if,
    clippy::collapsible_else_if
)]

mod app;
mod message_box;
mod nodes_snarl;
pub use app::App;
