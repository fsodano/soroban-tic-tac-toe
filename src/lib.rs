#![no_std]
extern crate alloc;

use alloc::string::ToString;
use soroban_sdk::{contractimpl, contracttype, Env, FixedBinary};
// use soroban_token_contract as token;
// use token::public_types::{Authorization, Identifier, KeyedAuthorization, U256};

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
    NotStarted,
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    pub player_one: FixedBinary<56>,
    pub player_two: FixedBinary<56>,
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
    pub winner: Winner
}

#[derive(PartialEq, Eq)]
pub enum GameError {
    InvalidRow,
    InvalidColumn,
    TileTaken,
    NotYourTurn,
    Inactive,
    AlreadyActive,
}

pub struct TicTacToeContract;

#[contractimpl(export_if = "export")]
impl TicTacToeContract {
    pub fn setup_game(e: Env, player_one: FixedBinary<56>, player_two: FixedBinary<56>, game_id: u32) -> u32 {
        if e.contract_data().has(game_id) {
            panic!("Game id {} already exists", game_id.to_string())
        }

        let game = Game {
            player_one,
            player_two,
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
            status: Status::NotStarted,
            winner: Winner::None,
        };

        e.contract_data().set(game_id, game);

        game_id
    }

    pub fn get_game(e: Env, game_id: u32) -> Game {
        if !e.contract_data().has(game_id) {
            panic!("Game id {} does not exist", game_id.to_string())
        }

        let game: Game = e.contract_data().get(game_id).unwrap().unwrap();
        game
    }
    //
    // pub fn play(e: Env, player: Identifier, game_id: u32, tile: u8) -> Winner {
    //     Winner::None
    // }
}

mod test;
