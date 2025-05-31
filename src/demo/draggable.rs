use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
#[derive(Component)]
pub struct Draggable;


fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
)


pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_drag);
}




fn on_drag(drag: Trigger<Pointer<Drag>>,mut mouse_motion:Res<AccumulatedMouseMotion>,time:Res<Time>, mut transforms:Query<(&mut Transform, &Draggable)>)
{
    if let Ok((mut transform, _draggable)) = transforms.get_mut(drag.target)
    {
        transform.translation += Vec3::new(mouse_motion.delta.x * 0.02, 0.0, mouse_motion.delta.y * 0.02);
    }

    let Ok(ray) = camera.viewport_to_world(camera_transform,cursor_position)
    else{
            return;
    };
}


