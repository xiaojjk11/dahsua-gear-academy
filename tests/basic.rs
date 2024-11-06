#![no_std]

use gtest::{Program, System};
use pebbles_game_io::*;

#[test]
fn test_pebbles_game() {
    let sys = System::new();
    let user_id = 111100;

    sys.init_logger();
    sys.mint_to(user_id, 10000000000000);

    // Initialize the program
    let pebbles_game = Program::current(&sys);

    let init_data = PebblesInit {
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    pebbles_game.send(user_id, init_data);

    // Check initial state
    let state: GameState = pebbles_game.read_state(()).expect("Failed to get state");
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 3);
    assert_eq!(state.pebbles_remaining, 8);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);

    // User makes a move
    let action = PebblesAction::Turn(2);
    pebbles_game.send(user_id, action);

    // Check state after user's move
    let state: GameState = pebbles_game.read_state(()).expect("Failed to get state");
    assert_eq!(state.pebbles_remaining, 3);

    // User gives up
    let action = PebblesAction::GiveUp;
    pebbles_game.send(user_id, action);

    // Check state after user gives up
    let state: GameState = pebbles_game.read_state(()).expect("Failed to get state");
    assert_eq!(state.winner, Some(Player::Program));

    // Restart the game
    let action = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 4,
    };
    pebbles_game.send(user_id, action);

    // Check state after restart
    let state: GameState = pebbles_game.read_state(()).expect("Failed to get state");
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.pebbles_remaining, 15);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert_eq!(state.winner, None);
}
