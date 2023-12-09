use godot::prelude::*;

mod rotating_sprite;

struct GodotActionTest;

#[gdextension]
unsafe impl ExtensionLibrary for GodotActionTest {}
