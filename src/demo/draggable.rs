use bevy::prelude::*;
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Draggable {
    pub axis: String,
    pub offset: i32,
    base_position: Vec3,
}

impl Draggable {
    pub fn new(axis: String, offset: i32, base_position: Vec3) -> Self {
        Self {
            axis,
            offset,
            base_position,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Draggable>();
    app.add_observer(on_drag);
}

fn on_drag(
    drag: Trigger<Pointer<Drag>>,
    mut transforms: Query<(&mut Transform, &Draggable)>,
    time: Res<Time>,
) {
    if drag.button != PointerButton::Primary {
        return;
    }

    if let Ok((mut transform, _draggable)) = transforms.get_mut(drag.target) {
        let disp_value = -drag.delta.y.signum() * drag.delta.length() * 0.5;
        let mut target_position =
            transform.translation + Vec3::new(0.0, disp_value * time.delta_secs(), 0.0);

        if target_position.y > _draggable.base_position.y as f32 {
            target_position.y = _draggable.base_position.y as f32;
        }
        if target_position.y < _draggable.base_position.y + _draggable.offset as f32 {
            target_position.y = _draggable.base_position.y + _draggable.offset as f32;
        }

        transform.translation = target_position;
    }
}
