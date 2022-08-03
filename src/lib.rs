#![no_std]
extern crate alloc;

use core::panic;

use alloc::string::ToString;
use auth::{check_auth, PublicKeyTrait};
use messages::TicTacToeMessage;
use soroban_sdk::{contractimpl, contracttype, vec, Env, FixedBinary};
use soroban_token_contract as token;
use token::public_types::KeyedAuthorization;

mod auth;
mod messages;

#[contracttype]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Sign {
    None,
    X,
    O,
}

#[contracttype]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Started,
    Finished,
}

#[contracttype]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Winner {
    None,
    Player1,
    Player2,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub player_one: FixedBinary<32>,
    pub player_two: FixedBinary<32>,
    pub turn: bool,
    pub tile_0: Sign,
    pub tile_1: Sign,
    pub tile_2: Sign,
    pub tile_3: Sign,
    pub tile_4: Sign,
    pub tile_5: Sign,
    pub tile_6: Sign,
    pub tile_7: Sign,
    pub tile_8: Sign,
    pub status: Status,
    pub winner: Winner,
}

pub fn get_game(e: &Env, game_id: u32) -> Game {
    if !e.contract_data().has(game_id) {
        panic!("Game id {} does not exist", game_id.to_string())
    }

    let game: Game = e.contract_data().get(game_id).unwrap().unwrap();
    game
}

fn has_won(game: &Game, sign: &Sign) -> bool {
    const ADJACENT_TILES_TO_WIN: u8 = 3;

    let board = [
        [game.tile_0, game.tile_1, game.tile_2],
        [game.tile_3, game.tile_4, game.tile_5],
        [game.tile_6, game.tile_7, game.tile_8],
    ];

    for i in 0..=2 {
        let mut horizontal_sum = 0;
        let mut vertical_sum = 0;

        for j in 0..=2 {
            horizontal_sum += u8::from(board[i][j].eq(sign));
            vertical_sum += u8::from(board[j][i].eq(sign));

            if horizontal_sum == ADJACENT_TILES_TO_WIN || vertical_sum == ADJACENT_TILES_TO_WIN {
                return true;
            }
        }
    }

    let left_to_right_diagonal_sum =
        game.tile_0.eq(sign) as u8 + game.tile_4.eq(sign) as u8 + game.tile_8.eq(sign) as u8;
    if left_to_right_diagonal_sum == ADJACENT_TILES_TO_WIN {
        return true;
    }

    let right_to_left_diagonal_sum =
        game.tile_2.eq(sign) as u8 + game.tile_4.eq(sign) as u8 + game.tile_6.eq(sign) as u8;
    if right_to_left_diagonal_sum == ADJACENT_TILES_TO_WIN {
        return true;
    }

    false
}

pub struct TicTacToeContract;

#[contractimpl(export_if = "export")]
impl TicTacToeContract {
    pub fn setup_game(
        e: Env,
        player_one_auth: KeyedAuthorization,
        player_two_auth: KeyedAuthorization,
        msg: TicTacToeMessage,
    ) -> Game {
        let setup_msg = match &msg {
            TicTacToeMessage::Setup(setup) => setup.clone(),
            _ => panic!("Incorrect message type"),
        };

        check_auth(&e, player_one_auth.clone(), msg.clone());
        check_auth(&e, player_two_auth.clone(), msg.clone());

        let player_one_public_key = player_one_auth.get_public_key(&e);
        let player_two_public_key = player_two_auth.get_public_key(&e);
        if player_one_public_key.eq(&player_two_public_key) {
            panic!("The game requires two distinct players")
        }

        if e.contract_data().has(setup_msg.game_id) {
            panic!("Game id {} already exists", setup_msg.game_id.to_string())
        }

        let game = Game {
            id: setup_msg.game_id,
            player_one: player_one_public_key,
            player_two: player_two_public_key,
            turn: false,
            tile_0: Sign::None,
            tile_1: Sign::None,
            tile_2: Sign::None,
            tile_3: Sign::None,
            tile_4: Sign::None,
            tile_5: Sign::None,
            tile_6: Sign::None,
            tile_7: Sign::None,
            tile_8: Sign::None,
            status: Status::Started,
            winner: Winner::None,
        };

        e.contract_data().set(setup_msg.game_id, game.clone());

        game
    }

    pub fn get_game(e: Env, game_id: u32) -> Game {
        get_game(&e, game_id)
    }

    pub fn play_game(e: Env, player: KeyedAuthorization, msg: TicTacToeMessage) -> Game {
        let play_msg = match &msg {
            TicTacToeMessage::Play(play) => play.clone(),
            _ => panic!("Incorrect message type"),
        };

        check_auth(&e, player.clone(), msg);

        let mut game: Game = get_game(&e, play_msg.game_id);

        if game.status.eq(&Status::Finished) {
            panic!("This game is finished");
        }

        let player_id = player.get_public_key(&e);

        let sign = match &game {
            g if g.player_one.eq(&player_id) & !g.turn => Sign::X,
            g if g.player_two.eq(&player_id) & g.turn => Sign::O,
            _ => panic!("Invalid player or turn"),
        };

        match play_msg.tile {
            0 if game.tile_0.eq(&Sign::None) => game.tile_0 = sign,
            1 if game.tile_1.eq(&Sign::None) => game.tile_1 = sign,
            2 if game.tile_2.eq(&Sign::None) => game.tile_2 = sign,
            3 if game.tile_3.eq(&Sign::None) => game.tile_3 = sign,
            4 if game.tile_4.eq(&Sign::None) => game.tile_4 = sign,
            5 if game.tile_5.eq(&Sign::None) => game.tile_5 = sign,
            6 if game.tile_6.eq(&Sign::None) => game.tile_6 = sign,
            7 if game.tile_7.eq(&Sign::None) => game.tile_7 = sign,
            8 if game.tile_8.eq(&Sign::None) => game.tile_8 = sign,
            _ => panic!("Invalid tile {}", play_msg.tile),
        }

        game.turn = !game.turn;

        if has_won(&game, &sign) {
            game.winner = if sign.eq(&Sign::X) {
                Winner::Player1
            } else {
                Winner::Player2
            };
            game.status = Status::Finished;
        } else {
            let total_plays: u8 = vec![
                &e,
                game.tile_0,
                game.tile_1,
                game.tile_2,
                game.tile_3,
                game.tile_4,
                game.tile_5,
                game.tile_6,
                game.tile_7,
                game.tile_8,
            ]
            .into_iter()
            .flatten()
            .map(|tile| -> u8 {
                if tile.ne(&Sign::None) {
                    1
                } else {
                    0
                }
            })
            .sum();

            const TOTAL_TILES: u8 = 9;
            if total_plays == TOTAL_TILES {
                game.status = Status::Finished;
            }
        }

        e.contract_data().set(game.id, game.clone());

        game
    }
}

mod test;
