use godot::{engine::Sprite2D, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct RotatingSprite {
    #[export]
    speed: real,

    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl RotatingSprite {}

#[godot_api]
impl INode2D for RotatingSprite {
    fn init(base: Base<Node2D>) -> Self {
        Self { speed: 0.1, base }
    }

    fn process(&mut self, _delta: f64) {
        let mut sprite = self.base.get_node_as::<Sprite2D>("sprite");
        sprite.rotate(self.speed);
    }
}
