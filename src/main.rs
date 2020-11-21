extern crate bevy;
extern crate bevy_webgl2;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;

fn main() {
    App::build()
        .add_plugins(bevy_webgl2::DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(meshify()),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(-5.0, -5.0, -5.0)),
            ..Default::default()
        });
}

fn meshify() -> Mesh {
    let size = 5.;
    let vertices = &[
        ([-size, -size, size], [0., 0., size], [0., 0.]),
        ([size, -size, size], [0., 0., size], [size, 0.]),
        ([size, size, size], [0., 0., size], [size, size]),
        ([-size, size, size], [0., 0., size], [0., size]),
        ([-size, size, -size], [0., 0., -size], [size, 0.]),
        ([size, size, -size], [0., 0., -size], [0., 0.]),
        ([size, -size, -size], [0., 0., -size], [0., size]),
        ([-size, -size, -size], [0., 0., -size], [size, size]),
        ([size, -size, -size], [size, 0., 0.], [0., 0.]),
        ([size, size, -size], [size, 0., 0.], [size, 0.]),
        ([size, size, size], [size, 0., 0.], [size, size]),
        ([size, -size, size], [size, 0., 0.], [0., size]),
        ([-size, -size, size], [-size, 0., 0.], [size, 0.]),
        ([-size, size, size], [-size, 0., 0.], [0., 0.]),
        ([-size, size, -size], [-size, 0., 0.], [0., size]),
        ([-size, -size, -size], [-size, 0., 0.], [size, size]),
        ([size, size, -size], [0., size, 0.], [size, 0.]),
        ([-size, size, -size], [0., size, 0.], [0., 0.]),
        ([-size, size, size], [0., size, 0.], [0., size]),
        ([size, size, size], [0., size, 0.], [size, size]),
        ([size, -size, size], [0., -size, 0.], [0., 0.]),
        ([-size, -size, size], [0., -size, 0.], [size, 0.]),
        ([-size, -size, -size], [0., -size, 0.], [size, size]),
        ([size, -size, -size], [0., -size, 0.], [0., size]),
    ];

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    //let mut uvs = Vec::new();
    for (position, normal, _uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        //uvs.push(*_uv);
    }

    let indices = Indices::U32(vec![
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    //mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));
    mesh
}
