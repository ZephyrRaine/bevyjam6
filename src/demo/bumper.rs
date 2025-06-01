use bevy::prelude::*;

#[derive(Component)]
pub struct Bumper {
    pub bump_hover: f32,
    pub bump_pressed: f32,
    pub target_scale: f32,
}

impl Bumper {
    fn set_target_default(&mut self) {
        self.target_scale = 1.0;
    }

    fn set_target_hover(&mut self) {
        self.target_scale = self.bump_hover;
    }
    fn set_target_pressed(&mut self) {
        self.target_scale = self.bump_pressed;
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, scale_update);
    app.add_observer(bump_play_hover);
    app.add_observer(bump_play_pressed);
    app.add_observer(bump_play_released);
    app.add_observer(bump_play_out);
}

// This system will scale any entity with assigned Scaling in each direction
// by cycling through the directions to scale.
fn scale_update(mut bumps: Query<(&mut Transform, &Bumper)>, fixed_time: Res<Time<Fixed>>) {
    for (mut transform, bump) in &mut bumps {
        let a = fixed_time.overstep_fraction();
        transform.scale = transform.scale.lerp(Vec3::ONE * bump.target_scale, a);
    }
}

pub fn bump_play_hover(trigger: Trigger<Pointer<Over>>, mut query: Query<&mut Bumper>) {
    if let Ok(mut bumper) = query.get_mut(trigger.target()) {
        bumper.set_target_hover();
    }
}
pub fn bump_play_pressed(trigger: Trigger<Pointer<Pressed>>, mut query: Query<&mut Bumper>) {
    if let Ok(mut bumper) = query.get_mut(trigger.target()) {
        bumper.set_target_pressed();
    }
}
pub fn bump_play_released(trigger: Trigger<Pointer<Released>>, mut query: Query<&mut Bumper>) {
    if let Ok(mut bumper) = query.get_mut(trigger.target()) {
        bumper.set_target_default();
    }
}
pub fn bump_play_out(trigger: Trigger<Pointer<Out>>, mut query: Query<&mut Bumper>) {
    if let Ok(mut bumper) = query.get_mut(trigger.target()) {
        bumper.set_target_default();
    }
}
