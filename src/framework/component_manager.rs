// pub struct ComponentManager {
//     components: Vec<Box<dyn Component>>,
//     // For fast lookup by type or id
//     type_index: HashMap<TypeId, usize>,
//     id_index: HashMap<ComponentId, usize>,
// }

// impl ComponentManager {
//     pub fn register<C: Component + 'static>(&mut self, component: C) -> ComponentId;
//     pub fn unregister(&mut self, id: ComponentId);
//     pub fn get<C: Component + 'static>(&self) -> Option<&C>;
//     pub fn get_mut<C: Component + 'static>(&mut self) -> Option<&mut C>;
// }
