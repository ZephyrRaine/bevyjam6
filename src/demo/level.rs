//! Spawn the main level.

use bevy::{
    color::palettes::tailwind::*, pbr::CascadeShadowConfigBuilder,
    picking::pointer::PointerInteraction, prelude::*,
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, draw_mesh_intersections);
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 200.0,
            ..default()
        },
        Transform::IDENTITY.looking_to(Vec3::new(2.5, -1., 0.85), Vec3::Y),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 70.0,
            maximum_distance: 1500.0,
            ..default()
        }
        .build(),
    ));

    commands.spawn((
        SpotLight {
            color: Color::LinearRgba(LinearRgba {
                red: 1.0,
                green: 1.0,
                blue: 0.80,
                alpha: 1.0,
            }),
            intensity: 200.0,
            range: 100.0,
            radius: 35.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 250.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::LinearRgba(LinearRgba {
            red: 1.0,
            green: 1.0,
            blue: 0.80,
            alpha: 1.0,
        }),
        brightness: 400.00,
        ..default()
    });
}

/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}
