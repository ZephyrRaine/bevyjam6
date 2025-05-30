//! Spawn the main level.

use bevy::prelude::*;
use bevy_vox_scene::VoxScenePlugin;

use crate::{
    asset_tracking::LoadResource,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(VoxScenePlugin::default());
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
    #[dependency]
    module: Handle<Scene>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            module: assets.load("models/module.vox"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay)
    ));

/*    commands.spawn((
        SceneRoot(level_assets.module.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));*/

    let ball_mesh = mesh_assets.add(Cuboid::new(10.0, 10.0, 10.0));
    let ball_material = material_assets.add(StandardMaterial {
        base_color: Color::linear_rgb(1.0, 0.0, 1.0),
        ..Default::default()
    });
    println!("I was just spawned!");

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Mesh3d(ball_mesh.clone()),
        MeshMaterial3d(ball_material),
    )).observe(|mut trigger: Trigger<Pointer<Click>>| {
        println!("I was just clicked!");
        // Get the underlying pointer event data
        let _click_event: &Pointer<Click> = trigger.event();
        // Stop the event from bubbling up the entity hierarchy
        trigger.propagate(false);
    }).observe(rotate_on_drag);


    commands.spawn((
        DirectionalLight::default(),
        Transform::IDENTITY.looking_to(Vec3::new(2.5, -1., 0.85), Vec3::Y),
    ));
}

/// An observer to rotate an entity when it is dragged
fn rotate_on_drag(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(drag.target()).unwrap();
    transform.rotate_y(drag.delta.x * 0.02);
    transform.rotate_x(drag.delta.y * 0.02);
}
