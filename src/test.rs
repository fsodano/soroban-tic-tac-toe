#![cfg(test)]

use super::{setup_game, get_game, Game, TicTacToeContract};
use soroban_sdk::{Env, FixedBinary};


#[test]
#[should_panic(expected = "Game id 1 already exists")]
fn test_game_start() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, TicTacToeContract);

    let player_one_bytes: &[u8; 56] = b"GC77XXRLAI6K47NE5ULNTWHLZOADIHEV4HXC52XC522D3LSX644D425C".try_into().unwrap();
    let player_two_bytes: &[u8; 56] = b"GBXX6JVZKZDZBZNPNB3H3XWX27AYOID66YA576RE2YVM52WPAUK3KND2".try_into().unwrap();
    let game_id = setup_game::invoke(
        &env,
        &contract_id,
        &FixedBinary::from_array(&env, *player_one_bytes),
        &FixedBinary::from_array(&env, *player_two_bytes),
        &1u32
    );
    assert_eq!(game_id, 1u32);

    let game_id = setup_game::invoke(
        &env,
        &contract_id,
        &FixedBinary::from_array(&env, *player_one_bytes),
        &FixedBinary::from_array(&env, *player_two_bytes),
        &1u32
    );

    assert_eq!(game_id, 1u32);
}


#[test]
#[should_panic(expected = "Game id 2 does not exist")]
fn test_get_game() {
    let env = Env::default();
    let contract_id = FixedBinary::from_array(&env, [0; 32]);
    env.register_contract(&contract_id, TicTacToeContract);

    const VALID_GAME_ID: u32 = 1;
    let player_one_bytes: &[u8; 56] = b"GC77XXRLAI6K47NE5ULNTWHLZOADIHEV4HXC52XC522D3LSX644D425C".try_into().unwrap();
    let player_two_bytes: &[u8; 56] = b"GBXX6JVZKZDZBZNPNB3H3XWX27AYOID66YA576RE2YVM52WPAUK3KND2".try_into().unwrap();
    let player_one_fixed_binary = FixedBinary::from_array(&env, *player_one_bytes);
    let player_two_fixed_binary = FixedBinary::from_array(&env, *player_two_bytes);
    let game_id = setup_game::invoke(
        &env,
        &contract_id,
        &player_one_fixed_binary,
        &player_two_fixed_binary,
        &VALID_GAME_ID
    );
    assert_eq!(game_id, VALID_GAME_ID);

    let game: Game = get_game::invoke(
        &env,
        &contract_id,
        &VALID_GAME_ID
    );
    assert_eq!(game.player_one, player_one_fixed_binary);
    assert_eq!(game.player_two, player_two_fixed_binary);

    const INVALID_GAME_ID: u32 = 2;
    let _game: Game = get_game::invoke(
        &env,
        &contract_id,
        &INVALID_GAME_ID
    );
}
