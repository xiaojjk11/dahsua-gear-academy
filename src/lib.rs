#![no_std]

use gstd::{msg, exec, debug};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

fn program_turn(game_state: &mut GameState) {
    let pebbles_to_remove = match game_state.difficulty {
        DifficultyLevel::Easy => {
            // Random number between 1 and max_pebbles_per_turn
            (get_random_u32() % game_state.max_pebbles_per_turn) + 1
        }
        DifficultyLevel::Hard => {
            // Best strategy for hard mode
            let winning_move = (game_state.pebbles_remaining - 1) % (game_state.max_pebbles_per_turn + 1);
            if winning_move == 0 {
                1 // Take at least one pebble
            } else {
                winning_move
            }
        }
    };

    game_state.pebbles_remaining = game_state.pebbles_remaining.saturating_sub(pebbles_to_remove);
    msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0).expect("Failed to send reply");

    if game_state.pebbles_remaining == 0 {
        game_state.winner = Some(Player::Program);
        msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Failed to send win message");
    }
}

#[no_mangle]
pub extern "C" fn init() {
    debug!("Pebbles game init");
    let init_data: PebblesInit = msg::load().expect("Failed to load PebblesInit");

    // Validate input data
    if init_data.pebbles_count == 0 || init_data.max_pebbles_per_turn == 0 {
        panic!("Invalid game configuration");
    }

    // Choose the first player randomly
    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };
    debug!("First player: {:?}", first_player);
    // Initialize game state
    let mut game_state = GameState {
        pebbles_count: init_data.pebbles_count,
        max_pebbles_per_turn: init_data.max_pebbles_per_turn,
        pebbles_remaining: init_data.pebbles_count,
        difficulty: init_data.difficulty,
        first_player: first_player.clone(),
        winner: None,
    };

    debug!("First player2: {:?}", first_player);

    // Program plays first if it's the first player
    if first_player == Player::Program {
        program_turn(&mut game_state);
    }
    debug!("First player3: {:?}", first_player);

    unsafe {
        PEBBLES_GAME = Some(game_state);
    }
}

#[no_mangle]
pub extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Failed to load PebblesAction");
    let game_state = unsafe { PEBBLES_GAME.as_mut().expect("Game not initialized") };

    match action {
        PebblesAction::Turn(pebbles) => {
            if pebbles == 0 || pebbles > game_state.max_pebbles_per_turn {
                panic!("Invalid move");
            }

            game_state.pebbles_remaining = game_state.pebbles_remaining.saturating_sub(pebbles);
            if game_state.pebbles_remaining == 0 {
                game_state.winner = Some(Player::User);
                msg::reply(PebblesEvent::Won(Player::User), 0).expect("Failed to send win message");
            } else {
                program_turn(game_state);
            }
        }
        PebblesAction::GiveUp => {
            game_state.winner = Some(Player::Program);
            msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Failed to send give up message");
        }
        PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
            game_state.pebbles_count = pebbles_count;
            game_state.max_pebbles_per_turn = max_pebbles_per_turn;
            game_state.pebbles_remaining = pebbles_count;
            game_state.difficulty = difficulty;
            game_state.winner = None;

            let first_player = if get_random_u32() % 2 == 0 {
                Player::User
            } else {
                Player::Program
            };

            game_state.first_player = first_player.clone();
            if first_player == Player::Program {
                program_turn(game_state);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn state() {
    let game_state = unsafe { PEBBLES_GAME.as_ref().expect("Game not initialized") };
    msg::reply(game_state, 0).expect("Failed to reply with game state");
}

