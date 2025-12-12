//! Atomic UI widgets built on the reactive primitives.
//!
//! Widgets are small, reusable UI elements that render into a `&mut Ui`.

mod button;
pub mod calendar;

pub use button::{Button, ButtonOptions};
