#![cfg(test)]

use crate::{
    messages::{PlayMessage, SetupMessage, TicTacToeMessage},
    play_game, Winner, Status,
};

use super::{get_game, setup_game, Game, TicTacToeContract};
use alloc::vec::Vec;
use ed25519_dalek::Keypair;
use rand::thread_rng;
use soroban_sdk::{testutils::ed25519::Sign, Env, FixedBinary, IntoVal};
use soroban_token_contract::public_types::{KeyedAuthorization, KeyedEd25519Signature};
extern crate std;

fn generate_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

fn create_test_game(
    env: &Env,
    contract_id: &FixedBinary<32>,
    player1: &Keypair,
    player2: &Keypair,
    game_id: u32,
) -> Game {
    let msg = TicTacToeMessage::Setup(SetupMessage {
        game_id: game_id.clone(),
    });

    let authorized_setup_msg_player_one = KeyedAuthorization::Ed25519(KeyedEd25519Signature {
        public_key: FixedBinary::from_array(env, player1.public.to_bytes()),
        signature: player1.sign(&msg).unwrap().into_val(env),
    });

    let authorized_setup_msg_player_two = KeyedAuthorization::Ed25519(KeyedEd25519Signature {
        public_key: FixedBinary::from_array(env, player2.public.to_bytes()),
        signature: player2.sign(&msg).unwrap().into_val(env),
    });

    setup_game::invoke(
        env,
        contract_id,
        &authorized_setup_msg_player_one,
        &authorized_setup_msg_player_two,
        &msg,
    )
}

fn create_test_play(
    env: &Env,
    contract_id: &FixedBinary<32>,
    player: &Keypair,
    game_id: u32,
    tile: u32,
) -> Game {
    let msg = TicTacToeMessage::Play(PlayMessage { game_id, tile });

    let authorized_play_msg = KeyedAuthorization::Ed25519(KeyedEd25519Signature {
        public_key: FixedBinary::from_array(&env, player.public.to_bytes()),
        signature: player.sign(&msg).unwrap().into_val(env),
    });

    play_game::invoke(env, contract_id, &authorized_play_msg, &msg)
}

#[test]
#[should_panic(expected = "Game id 1 already exists")]
fn test_setup_duplicate_game_fails() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, TicTacToeContract);

    let player1 = generate_keypair();
    let player2 = generate_keypair();
    const VALID_GAME_ID: u32 = 1;

    let game = create_test_game(&env, &contract_id, &player1, &player2, VALID_GAME_ID);
    assert_eq!(game.id, VALID_GAME_ID);

    // Fail to create a game with the same id
    create_test_game(&env, &contract_id, &player1, &player2, VALID_GAME_ID);
}

#[test]
#[should_panic(expected = "The game requires two distinct players")]
fn test_setup_game_same_player_fails() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, TicTacToeContract);

    let player1 = generate_keypair();
    const VALID_GAME_ID: u32 = 1;
    create_test_game(&env, &contract_id, &player1, &player1, VALID_GAME_ID);
}

#[test]
#[should_panic(expected = "Game id 2 does not exist")]
fn test_get_game() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, TicTacToeContract);

    const INVALID_GAME_ID: u32 = 2;
    let _game: Game = get_game::invoke(&env, &contract_id, &INVALID_GAME_ID);
}

#[test]
#[should_panic(expected = "Invalid player or turn")]
fn test_play_invalid_turn_fails() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, TicTacToeContract);

    let player1 = generate_keypair();
    let player2 = generate_keypair();
    const VALID_GAME_ID: u32 = 1;
    create_test_game(&env, &contract_id, &player1, &player2, VALID_GAME_ID);
    create_test_play(&env, &contract_id, &player1, VALID_GAME_ID, 0);
    // Player 1 cannot play twice
    create_test_play(&env, &contract_id, &player1, VALID_GAME_ID, 1);
}

#[test]
#[should_panic(expected = "Invalid tile 0")]
fn test_play_repeated_tile_fails() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, TicTacToeContract);

    let player1 = generate_keypair();
    let player2 = generate_keypair();
    const VALID_GAME_ID: u32 = 1;
    create_test_game(&env, &contract_id, &player1, &player2, VALID_GAME_ID);
    create_test_play(&env, &contract_id, &player1, VALID_GAME_ID, 0);
    // Cannot play on the same tile again
    create_test_play(&env, &contract_id, &player2, VALID_GAME_ID, 0);
}

#[test]
#[should_panic(expected = "Invalid tile 9")]
fn test_play_invalid_tile_fails() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, TicTacToeContract);

    let player1 = generate_keypair();
    let player2 = generate_keypair();
    const VALID_GAME_ID: u32 = 1;
    create_test_game(&env, &contract_id, &player1, &player2, VALID_GAME_ID);
    create_test_play(&env, &contract_id, &player1, VALID_GAME_ID, 0);
    // Cannot play on a tile that is out of bounds
    create_test_play(&env, &contract_id, &player2, VALID_GAME_ID, 9);
}

#[test]
fn test_play_game() {
    struct Play {
        winner: Winner,
        tiles: Vec<u32>,
    }

    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, TicTacToeContract);

    let player1: Keypair = generate_keypair();
    let player2: Keypair = generate_keypair();
    let mut game_id: u32 = 1;

    let winning_plays = [
        // [X][O][O]
        // [ ][X][ ]
        // [ ][ ][X]
        Play {
            winner: Winner::Player1,
            tiles: Vec::from([0, 1, 4, 2, 8]),
        },
        // [O][O][X]
        // [ ][X][ ]
        // [X][ ][ ]
        Play {
            winner: Winner::Player1,
            tiles: Vec::from([2, 1, 4, 0, 6]),
        },
        // [O][O][ ]
        // [ ][ ][ ]
        // [X][X][X]
        Play {
            winner: Winner::Player1,
            tiles: Vec::from([6, 0, 7, 1, 8]),
        },
        // [O][O][X]
        // [ ][ ][X]
        // [ ][ ][X]
        Play {
            winner: Winner::Player1,
            tiles: Vec::from([2, 0, 5, 1, 8]),
        },
        // [X][O][O]
        // [O][X][X]
        // [X][X][O]
        Play {
            winner: Winner::None,
            tiles: Vec::from([0,1,4,8,5,3,6,2,7]),
        },
        // [ ][ ][O]
        // [ ][X][O]
        // [X][X][O]
        Play {
            winner: Winner::Player2,
            tiles: Vec::from([4, 2, 6, 5, 7, 8]),
        },
    ];

    winning_plays.into_iter().for_each(|winning_play| -> () {
        let mut game = create_test_game(&env, &contract_id, &player1, &player2, game_id);
        let mut current_player = &player1;
        for tile in winning_play.tiles {
            game = create_test_play(&env, &contract_id, current_player, game_id, tile);
            current_player = if current_player.public.eq(&player1.public) {
                &player2
            } else {
                &player1
            };
        }

        assert_eq!(game.status, Status::Finished);
        assert_eq!(game.winner, winning_play.winner);
        game_id += 1;
    });
}
