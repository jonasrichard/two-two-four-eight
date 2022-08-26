use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.2, 0.4)))
        .insert_resource(WindowDescriptor {
            title: "2248".to_owned(),
            width: 450.0,
            height: 680.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
