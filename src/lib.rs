use gdnative::prelude::*;

mod player;

fn init(handle: InitHandle) {
    println!("Bardulia 0.0.1");

    handle.add_class::<player::Player>();
}

godot_init!(init);
