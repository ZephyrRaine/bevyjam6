use bevy::prelude::*;
#[derive(Component)]
pub struct Draggable;



pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_drag);
}


fn on_drag(
    drag: Trigger<Pointer<Drag>>,
    mut transforms: Query<(&mut Transform, &Draggable)>,
) {
    if let Ok((mut transform, _draggable)) = transforms.get_mut(drag.target) {
        transform.translation += Vec3::new(drag.delta.x, 0.0, drag.delta.y);
    }
}

