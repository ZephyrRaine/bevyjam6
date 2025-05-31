use bevy::prelude::*;
#[derive(Component)]
pub struct Draggable;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_drag);
}

fn on_drag(
    drag: Trigger<Pointer<Drag>>,
    mut transforms: Query<(&mut Transform, &Draggable)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, _draggable)) = transforms.get_mut(drag.target) {
        let sign = drag.delta.x.signum() * drag.delta.length() * 0.5;
        transform.translation += Vec3::new(sign * time.delta_secs(), 0.0, 0.0);
    }
}
