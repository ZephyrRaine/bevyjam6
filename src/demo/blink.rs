use bevy::prelude::*;

use crate::demo::synchronized::{SyncTracks, Synchronized};

#[derive(Component)]
pub struct Blink {
    pub on_material: Handle<StandardMaterial>,
    pub off_material: Handle<StandardMaterial>,
    pub is_on: bool,
}

impl Blink {
    fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }

    fn material(&self) -> &Handle<StandardMaterial> {
        match self.is_on {
            true => &self.on_material,
            false => &self.off_material,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, blink);
}
pub fn blink(
    mut query: Query<(Entity, &mut Blink, &Synchronized)>,
    blink_tracks: Res<SyncTracks>,
    mut commands: Commands,
) {
    for (entity, mut blink, sync) in query.iter_mut() {
        if blink_tracks.timer_tracks[sync.track].finished() {
            blink.toggle();
            commands
                .entity(entity)
                .insert(MeshMaterial3d(blink.material().clone()));
        }
    }
}
