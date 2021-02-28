use gdnative::prelude::*;
use gdnative::api::Camera;

#[derive(Default, NativeClass)]
#[inherit(Camera)]
pub struct MainCamera {
    #[property]
    player: Option<NodePath>
}

#[gdnative::methods]
impl MainCamera {
    fn new(_owner: &Camera) -> Self {
        MainCamera::default()
    }

    fn get_player<'a>(&self, owner: &Camera) -> TRef<'a, Spatial, Shared> {
        let player = self.player.as_ref().unwrap();
        unsafe {
            owner
                .get_node(player.to_godot_string())
                .unwrap()
                .assume_safe()
                .cast::<Spatial>()
                .unwrap()
        }
    }

    #[export]
    fn _ready(&self, _owner: &Camera) {

    }

    #[export]
    fn _process(&self, owner: &Camera, _delta: f64) {
        let player_position = self.get_player(owner).translation();
        let camera_position = Vector3::new(player_position.x, player_position.y + 2.5, player_position.z + 3.0);
        owner.set_translation(camera_position);
    }
}