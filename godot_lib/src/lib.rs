use godot::classes::*;
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Control)]
struct NetworkControl {
    base: Base<Control>,
}

#[godot_api]
impl IControl for NetworkControl {
    fn init(base: Base<Control>) -> Self {
        godot_print!("Hello from rust");

        Self { base }
    }

    fn process(&mut self, _delta: f64) {}
}
