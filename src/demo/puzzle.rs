use bevy::prelude::*;

#[derive(Event)]
pub struct PuzzleEvent {
    pub puzzle_id: u32,
    pub slider_id: usize,
    pub slider_position: i32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PuzzleSolver {
    pub puzzle_id: u32,
    pub correct_positions: Vec<i32>,
    pub current_positions: Vec<i32>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<PuzzleEvent>();
    app.add_systems(Update, update_puzzle_solver);
    app.register_type::<PuzzleSolver>();
}

fn update_puzzle_solver(
    mut puzzle_events: EventReader<PuzzleEvent>,
    mut query: Query<&mut PuzzleSolver>,
) {
    for ev in puzzle_events.read() {
        for mut puzzle_solver in query.iter_mut() {
            if puzzle_solver.puzzle_id != ev.puzzle_id {
                continue;
            }
            puzzle_solver.current_positions[ev.slider_id] = ev.slider_position;

            if puzzle_solver.current_positions == puzzle_solver.correct_positions {
                println!("Puzzle {} solved!", ev.puzzle_id);
            }
        }
    }
}
