#[macro_use]
extern crate luajit;

use bevy::{
    prelude::{
        default,
        // shape,
        App, Assets, Camera3dBundle, Color, Commands, Mesh, PbrBundle, PointLight,
        PointLightBundle, ResMut, StandardMaterial, Transform, Vec3,
    },
    DefaultPlugins, render::{render_resource::PrimitiveTopology, mesh},
};
use luajit::{c_int, State};

// use bevy::prelude::*;

fn lifes_answer(state: &mut State) -> c_int {
    state.push(42);

    1
}

fn app_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Hello world");

    let mut state = State::new();
    state.open_libs();
    state.do_string(r#"print("Hello world!")"#);

    state.push(lua_fn!(lifes_answer));
    state.set_global("lifes_answer");
    state.do_string(r#"print(lifes_answer())"#);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0., 0., 0.], [1., 2., 1.], [2., 0., 0.]],
    );

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(app_setup)
        .run();
}
