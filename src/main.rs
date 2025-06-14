// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod demo;
#[cfg(feature = "dev")]
mod dev_tools;
mod screens;
mod theme;

use bevy::{
    asset::AssetMetaCheck,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{Atmosphere, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_audio_controller::prelude::AudioControllerPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bevyjam6".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );
        app.insert_resource(DirectionalLightShadowMap { size: 4096 });
        app.add_plugins(AudioControllerPlugin);
        app.add_plugins(MeshPickingPlugin);
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        });
        app.add_plugins(WorldInspectorPlugin::new());
        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            demo::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            screens::plugin,
            theme::plugin,
        ));

        app.add_plugins(PanOrbitCameraPlugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        PanOrbitCamera {
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Middle,
            focus: vec3(0.8056799, 40.183243, -5.5687037),
            //  radius: Some(100.0),
            ..default()
        },
        Camera {
            hdr: true,
            ..default()
        },
        Bloom {
            intensity: 0.1,
            scale: Vec2::new(2.4, 1.0),
            ..default()
        },
        Tonemapping::BlenderFilmic,
        Atmosphere::EARTH,
        Transform::from_xyz(1.909522, 44.110947, -105.49327),
    ));
}
