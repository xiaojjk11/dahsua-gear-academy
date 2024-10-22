#![no_std]

use gtest::{System, Program};
use pebbles_game_io::*;

#[test]
fn test_pebbles_game() {
    let sys = System::new();
    sys.init_logger();

    // Initialize the program
    let _ = Program::current(&sys);
    let init_data = PebblesInit {
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        difficulty: DifficultyLevel::Easy,
    };
    let res = pebbles_game.send(100001, init_data);
    assert!(!res.main_failed());

    // Check initial state
    let state: GameState = pebbles_game.read_state(()).expect("Failed to get state");
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 3);
    assert_eq!(state.pebbles_remaining, 10);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);

    // User makes a move
    let action = PebblesAction::Turn(2);
    let res = pebbles_game.send(100001, action);
    assert!(!res.main_failed());

    // Check state after user's move
    let state: GameState = pebbles_game.read_state(()).expect("Failed to get state");
    assert_eq!(state.pebbles_remaining, 5);

    // User gives up
    let action = PebblesAction::GiveUp;
    let res = pebbles_game.send(100001, action);
    assert!(!res.main_failed());

    // Check state after user gives up
    let state: GameState = pebbles_game.read_state(()).expect("Failed to get state");
    assert_eq!(state.winner, Some(Player::Program));

    // Restart the game
    let action = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 4,
    };
    let res = pebbles_game.send(100001, action);
        assert!(!res.main_failed());

    // Check state after restart
    let state: GameState = pebbles_game.read_state(()).expect("Failed to get state");
        assert_eq!(state.pebbles_count, 15);
        assert_eq!(state.max_pebbles_per_turn, 4);
        assert_eq!(state.pebbles_remaining, 11);
        assert_eq!(state.difficulty, DifficultyLevel::Hard);
        assert_eq!(state.winner, None);
}
