// fn dispatch_event(component: &mut dyn Component, event: &AppEvent) -> bool {
//     let mut dirty = component.handle_event(event);
//     for child in component.children() {
//         dirty |= dispatch_event(child.as_mut(), event);
//     }
//     dirty
// }
