//! Spawn the main level.

use bevy::{color::palettes::tailwind::*, picking::pointer::PointerInteraction, prelude::*};

use crate::{asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
    app.add_systems(Update, draw_mesh_intersections);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
    ));

    let ball_mesh = mesh_assets.add(Cuboid::new(10.0, 10.0, 10.0));
    let ball_material = material_assets.add(StandardMaterial {
        base_color: Color::linear_rgb(1.0, 0.0, 1.0),
        ..Default::default()
    });
    println!("I was just spawned!");

    commands
        .spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Mesh3d(ball_mesh.clone()),
            MeshMaterial3d(ball_material),
            StateScoped(Screen::Gameplay),
        ))
        .observe(|mut trigger: Trigger<Pointer<Click>>| {
            println!("I was just clicked!");
            // Get the underlying pointer event data
            let _click_event: &Pointer<Click> = trigger.event();
            // Stop the event from bubbling up the entity hierarchy
            trigger.propagate(false);
        });

    commands.spawn((
        DirectionalLight::default(),
        Transform::IDENTITY.looking_to(Vec3::new(2.5, -1., 0.85), Vec3::Y),
    ));
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
