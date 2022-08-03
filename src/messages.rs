use soroban_sdk::{contracttype};

#[derive(Clone)]
#[contracttype]
pub enum TicTacToeMessage {
    Setup(SetupMessage),
    Play(PlayMessage),
}

#[derive(Clone)]
#[contracttype]
pub struct SetupMessage {
    pub game_id: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct PlayMessage {
    pub game_id: u32,
    pub tile: u32,
}
