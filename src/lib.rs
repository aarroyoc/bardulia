use gdnative::prelude::*;

mod camera;
mod lever;
mod sign;
mod player;

fn init(handle: InitHandle) {
    println!("Bardulia 0.0.1");

    handle.add_class::<camera::MainCamera>();
    handle.add_class::<lever::Lever>();
    handle.add_class::<sign::Sign>();
    handle.add_class::<player::Player>();
}

godot_init!(init);
