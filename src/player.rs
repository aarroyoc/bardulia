use gdnative::prelude::*;
use gdnative::api::AnimationPlayer;

const GRAVITY: f32 = 1.0;

const PLAYER_SPEED: f32 = 1.0;
const ROTATE_SPEED: f32 = 3.0;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player;

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Player
    }

    fn get_animation_player(&self, owner: &KinematicBody) -> Ref<AnimationPlayer, Unique> {
        let animation = owner.get_node("./AnimationPlayer").unwrap();
        unsafe {
            animation
                .assume_unique()
                .cast::<AnimationPlayer>()
                .unwrap()
        }
    } 

    #[export]
    fn _ready(&self, owner: &KinematicBody) {
        let animation = self.get_animation_player(owner);
        animation.play("Idle", -1.0, 1.0, false);
    }

    #[export]
    fn _physics_process(&self, owner: &KinematicBody, delta: f64) {
        let animation = self.get_animation_player(owner);
        let delta = delta as f32;
        let input = Input::godot_singleton();

        if Input::is_action_pressed(&input, GodotString::from_str("move_forward")) {
            animation.play("Run", -1.0, 1.0, false);
            let rot = owner.rotation().y;
            let vector = Vector3::new(PLAYER_SPEED*rot.sin(), -GRAVITY, PLAYER_SPEED*rot.cos());
            owner.move_and_slide(vector, Vector3::new(0.0, 1.0, 0.0), false, 4, 0.78, true);
        } else {
            animation.play("Idle", -1.0, 1.0, false);
        }

        if Input::is_action_pressed(&input, GodotString::from_str("rotate_right")) {
            owner.rotate_y((delta*ROTATE_SPEED*-1.0).into());
        }

        if Input::is_action_pressed(&input, GodotString::from_str("rotate_left")) {
            owner.rotate_y((delta*ROTATE_SPEED).into());
        }
    }
}