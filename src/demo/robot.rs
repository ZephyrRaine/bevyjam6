use bevy::{prelude::*};
use bevy_vox_scene::{VoxLoaderSettings, VoxScenePlugin, VoxelInstanceReady};

use crate::{asset_tracking::LoadResource, screens::Screen};


pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        VoxScenePlugin {
            // Using global settings because Bevy's `load_with_settings` has a couple of issues:
            // https://github.com/bevyengine/bevy/issues/12320
            // https://github.com/bevyengine/bevy/issues/11111
            global_settings: Some(VoxLoaderSettings {
                supports_remeshing: true,
                ..default()
            }),
        },
    );
    app.register_type::<RobotAssets>();
    app.load_resource::<RobotAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RobotAssets {
    #[dependency]
    robot: Handle<Scene>,
}

impl FromWorld for RobotAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            robot: assets.load("models/robot.vox"),
        }
    }
}

pub fn spawn_robot(mut commands: Commands, robot_assets: Res<RobotAssets>) {
    commands
        .spawn((
            Name::new("Robot"),
            SceneRoot(robot_assets.robot.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            StateScoped(Screen::Gameplay),
        ))
        .observe(on_voxel_instance_ready);
}

fn on_voxel_instance_ready(trigger: Trigger<VoxelInstanceReady>, mut commands: Commands) {
    let Some(_name) = &trigger.event().model_name else {
        return;
    };
    let mut entity_commands = commands.entity(trigger.event().instance);

    entity_commands
        .observe(|mut trigger: Trigger<Pointer<Click>>| {
            println!("{} was just clicked!", trigger.target());
            // Get the underlying pointer event data
            let _click_event: &Pointer<Click> = trigger.event();
            // Stop the event from bubbling up the entity hierarchy
            trigger.propagate(false);
        })
        .observe(rotate_on_drag);
}

/// An observer to rotate an entity when it is dragged
fn rotate_on_drag(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(drag.target()).unwrap();
    transform.rotate_y(drag.delta.x * 0.02);
    transform.rotate_x(drag.delta.y * 0.02);
}
