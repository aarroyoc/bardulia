use gdnative::prelude::*;
use gdnative::api::AnimationPlayer;

const PLAYER_SPEED: f32 = 4.0;
const ROTATE_SPEED: f32 = 3.0;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Player {
    animation: Ref<AnimationPlayer, Unique>,
}

#[gdnative::methods]
impl Player {
    fn new(owner: &Spatial) -> Self {
        Player {
            animation: Self::get_animation_player(owner)
        }
    }

    fn get_animation_player(owner: &Spatial) -> Ref<AnimationPlayer, Unique> {
        let animation = owner.get_node("AnimationPlayer").unwrap();
        unsafe {
            animation
                .assume_unique()
                .cast::<AnimationPlayer>()
                .unwrap()
        }
    } 

    #[export]
    fn _ready(&self, _owner: &Spatial) {
        self.animation.play("Idle", -1.0, 1.0, false);
    }

    #[export]
    fn _process(&self, owner: &Spatial, delta: f64) {
        let delta = delta as f32;
        let input = Input::godot_singleton();
        if Input::is_action_pressed(&input, GodotString::from_str("move_forward")) {
            self.animation.play("Run", -1.0, 1.0, false);
            let rot_x = owner.rotation().x;
            let rot_z = owner.rotation().z;
            let vector = Vector3::new(delta*PLAYER_SPEED*rot_x.sin(), 0.0, delta*PLAYER_SPEED*rot_z.cos());
            owner.translate(vector);
        } else {
            self.animation.play("Idle", -1.0, 1.0, false);
        }

        if Input::is_action_pressed(&input, GodotString::from_str("rotate_right")) {
            owner.rotate_y((delta*ROTATE_SPEED*-1.0).into());
        }

        if Input::is_action_pressed(&input, GodotString::from_str("rotate_left")) {
            owner.rotate_y((delta*ROTATE_SPEED).into());
        }
    }
}