use bevy::prelude::*;

use crate::demo::toggler::IndexTracker;

#[derive(Event)]
pub struct PuzzleEvent {
    pub puzzle_id: u32,
    pub element_id: usize,
    pub element_value: i32,
}

#[derive(Event)]
pub struct PuzzleReset {
    pub puzzle_id: u32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PuzzleSolver {
    pub puzzle_id: u32,
    pub correct_positions: Vec<i32>,
    pub current_positions: Vec<i32>,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IndexTracker>();
    app.init_resource::<IndexTracker>();
    app.add_event::<PuzzleEvent>();
    app.add_event::<PuzzleReset>();
    app.add_systems(Update, update_puzzle_solver);
    app.register_type::<PuzzleSolver>();
}

fn update_puzzle_solver(
    mut puzzle_events: EventReader<PuzzleEvent>,
    mut puzzle_resets: EventWriter<PuzzleReset>,
    mut query: Query<&mut PuzzleSolver>,
    mut index_tracker: ResMut<IndexTracker>,
) {
    for ev in puzzle_events.read() {
        for mut puzzle_solver in query.iter_mut() {
            if puzzle_solver.puzzle_id != ev.puzzle_id {
                continue;
            }
            puzzle_solver.current_positions[ev.element_id] = ev.element_value;

            if puzzle_solver.current_positions == puzzle_solver.correct_positions {
                println!("Puzzle {} solved!", ev.puzzle_id);
            } else if ev.puzzle_id == 2 && index_tracker.current_id == 9 {
                println!("Puzzle {} failed!", ev.puzzle_id);
                index_tracker.current_id = 0;
                puzzle_resets.write(PuzzleReset { puzzle_id: ev.puzzle_id });
                puzzle_solver.current_positions = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
            }
        }
    }
}
