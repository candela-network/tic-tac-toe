
use soroban_sdk::{contracterror, contracttype, symbol, Address, Symbol};

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum GameState {
    PENDING,
    RUNNING(u32),
    COUNTER,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Game {
    pub player1: Address,
    pub player2: Address,
    pub board: u32,
    pub next: u32,
}

#[contracttype]
#[derive(Debug, PartialEq, Clone)]
pub enum PlayResult {
    NEXT,
    WINNER,
    DRAW,
}

#[contracterror]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InvalidErrorCode {
    Unknown = 0,
    NotAPlayer = 1,
    GameNotFound = 2,
    NotYourTurn = 3,
    MoveOutOfBound = 4,
    InvalidMove = 5,
}

pub const EVENT_TOPIC: Symbol = symbol!("Event");
#[contracttype]
pub struct TicTacToeEvent {
    pub id: u32,
    pub game: Game,
    pub result: PlayResult,
}
