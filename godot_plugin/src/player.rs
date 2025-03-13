use godot::{
    builtin::Vector2,
    classes::{ISprite2D, Sprite2D},
    global::godot_print,
    obj::{Base, WithBaseField},
    prelude::{godot_api, GodotClass},
};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello World!");
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        {
            self.speed += amount;
            self.base_mut().emit_signal("speed_increased", &[]);
        }
    }

    #[signal]
    fn speed_increased();
}
