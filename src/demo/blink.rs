use bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct BlinkTracks {
    pub timer_tracks: [Timer; 3],
}

impl FromWorld for BlinkTracks {
    fn from_world(_world: &mut World) -> Self {
        Self {
            timer_tracks: [
                Timer::from_seconds(0.1, TimerMode::Repeating),
                Timer::from_seconds(0.5, TimerMode::Repeating),
                Timer::from_seconds(1.0, TimerMode::Repeating),
            ],
        }
    }
}

//TODO: Extract track component into separate component
#[derive(Component)]
pub struct Blink {
    pub track: usize,
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
    app.register_type::<BlinkTracks>();
    app.init_resource::<BlinkTracks>();
    app.add_systems(Update, update_timers);
    app.add_systems(Update, blink);
}

pub fn update_timers(mut blink_tracks: ResMut<BlinkTracks>, time: Res<Time>) {
    for timer in blink_tracks.timer_tracks.iter_mut() {
        timer.tick(time.delta());
    }
}

pub fn blink(
    mut query: Query<(Entity, &mut Blink)>,
    blink_tracks: Res<BlinkTracks>,
    mut commands: Commands,
) {
    for (entity, mut blink) in query.iter_mut() {
        if blink_tracks.timer_tracks[blink.track].finished() {
            blink.toggle();
            commands
                .entity(entity)
                .insert(MeshMaterial3d(blink.material().clone()));
        }
    }
}
