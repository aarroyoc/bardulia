use gdnative::prelude::*;
use gdnative::api::RichTextLabel;

#[derive(Default, NativeClass)]
#[inherit(Spatial)]
pub struct Sign {
    #[property]
    text: String,
    #[property]
    player: Option<NodePath>,
    #[property]
    label: Option<NodePath>,
}

#[gdnative::methods]
impl Sign {
    fn new(_owner: &Spatial) -> Self {
        Sign::default()
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

    fn get_label<'a>(&self, owner: &Spatial) -> TRef<'a, RichTextLabel, Shared> {
        let label = self.label.as_ref().unwrap();
        unsafe {
            owner
                .get_node(label.to_godot_string())
                .unwrap()
                .assume_safe()
                .cast::<RichTextLabel>()
                .unwrap()
        }
    }

    #[export]
    fn _input(&self, owner: &Spatial, event: Ref<InputEvent>) {
        let player = self.get_player(owner);
        let label = self.get_label(owner);
        let event = unsafe {
            event
                .assume_safe()
                .cast::<InputEvent>()
                .unwrap()
        };
        if event.is_action_pressed(GodotString::from_str("move_lever"), false) {
            let distance = owner.global_transform().origin.distance_to(player.global_transform().origin);
            if distance < 0.4 {
                label.set_text(self.text.clone());
            }
        }
    }
}