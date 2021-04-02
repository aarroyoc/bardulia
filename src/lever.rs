use gdnative::prelude::*;

#[derive(Default, NativeClass)]
#[inherit(Spatial)]
pub struct Lever {
    #[property]
    gate: Option<NodePath>,
    #[property]
    player: Option<NodePath>,
    opened: bool,
}

#[gdnative::methods]
impl Lever {
    fn new(_owner: &Spatial) -> Self {
        Lever::default()
    }

    fn get_player<'a>(&self, owner: &Spatial) -> TRef<'a, Spatial, Shared> {
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

    fn get_gate<'a>(&self, owner: &Spatial) -> TRef<'a, Spatial, Shared> {
        let gate = self.gate.as_ref().unwrap();
        unsafe {
            owner
                .get_node(gate.to_godot_string())
                .unwrap()
                .assume_safe()
                .cast::<Spatial>()
                .unwrap()
        }
    }

    fn open_gate(&self, owner: &Spatial) {
        owner.set_rotation_degrees(Vector3::new(-90.0, 90.0, 0.0));
        let gate = self.get_gate(owner);
        gate.set_rotation_degrees(Vector3::new(0.0, 0.0, 0.0));
    }

    fn close_gate(&self, owner: &Spatial) {
        owner.set_rotation_degrees(Vector3::new(90.0, 90.0, 0.0));
        let gate = self.get_gate(owner);
        gate.set_rotation_degrees(Vector3::new(0.0, -90.0, 0.0));
    }

    #[export]
    fn _input(&mut self, owner: &Spatial, event: Ref<InputEvent>) {
        let player = self.get_player(owner);
        let event = unsafe {
            event
                .assume_safe()
                .cast::<InputEvent>()
                .unwrap()
        };
        if event.is_action_pressed(GodotString::from_str("move_lever"), false) {
            let distance = owner.global_transform().origin.distance_to(player.global_transform().origin);
            if distance < 0.4 {
                if self.opened {
                    self.close_gate(owner);
                    self.opened = false;
                } else {
                    self.open_gate(owner);
                    self.opened = true;
                }
            }
        }
    }
}