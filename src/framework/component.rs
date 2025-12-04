// Component trait + ComponentId

// pub struct ComponentId {
//     pub name: String,
// }

// impl ComponentId {
//     pub fn new(name: &str) -> Self {
//         Self {
//             name: name.to_string(),
//         }
//     }
// }

use egui::Context;

pub trait Component {
    fn on_init(&mut self) {}

    fn on_visible(&mut self) {}

    fn on_hidden(&mut self) {}

    // fn handle_event(&mut self, event: &AppEvent) -> bool;

    fn ui(&mut self, ctx: &Context);
}
