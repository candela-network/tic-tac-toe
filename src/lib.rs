#![no_std]
use soroban_sdk::{
    bigint, contracterror, contractimpl, contracttype, panic_with_error, Address, BigInt, BytesN,
    Env,
};

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum DataKey {
    PENDING,
    RUNNING(u32),
    COUNTER,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Game {
    player1: Address,
    player2: Address,
    board: BigInt,
    next: u32,
}

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum PlayResult {
    NEXT,
    WINNER,
    DRAW,
}

#[contracterror]
#[derive(Debug, PartialEq, Clone, Copy)]
enum InvalidErrorCode {
    Unknown = 0,
    NotAPlayer = 1,
    GameNotFound = 2,
    NotYourTurn = 3,
    MoveOutOfBound = 4,
    InvalidMove = 5,
}

const SOLUTION: [u32; 8] = [
    0b111000000,
    0b000111000,
    0b000000111,
    0b100100100,
    0b010010010,
    0b001001001,
    0b100010001,
    0b001010100,
];

pub struct TicTacToeContract;

#[contractimpl]
impl TicTacToeContract {
    pub fn launch(env: Env) -> DataKey {
        let pending: Address = env
            .data()
            .get(DataKey::PENDING)
            .unwrap_or(Ok(env.invoker()))
            .unwrap();

        if pending != env.invoker() {
            let mut counter = env.data().get(DataKey::COUNTER).unwrap_or(Ok(0)).unwrap();

            // Increment game id
            counter += 1;
            env.data().set(DataKey::COUNTER, counter);

            // Store the new game
            env.data().set(
                DataKey::RUNNING(counter),
                Game {
                    player1: pending,
                    player2: env.invoker(),
                    board: bigint![&env, 0x00],
                    next: 0,
                },
            );

            // Clean up pending
            env.data().remove(DataKey::PENDING);

            return DataKey::RUNNING(counter);
        } else {
            env.data().set(DataKey::PENDING, pending);
            return DataKey::PENDING;
        }
    }

    pub fn play(env: Env, game_id: u32, square: BytesN<2>) -> PlayResult {
        let game = Self::get_game(&env, game_id);
        let m = Self::get_square(&env, square);

        Self::check_player(&env, &game);

        let player_id = game.next % 2;
        let played_move = Self::get_move(&env, m, &game);

        let mut mgame = game;
        let mut board = mgame.board.to_u32();

        // Apply the  move
        if player_id == 0 {
            board = board | (played_move << 9);
        } else {
            board = board | played_move;
        };
        mgame.board = BigInt::from_u32(&env, board);

        // Get the next game action
        let result = Self::get_next_action(player_id, board);

        if result == PlayResult::NEXT {
            mgame.next += 1;
            env.data().set(DataKey::RUNNING(game_id), mgame);
        } else {
            env.data().remove(DataKey::RUNNING(game_id));
        }

        result
    }

    fn get_game(env: &Env, gid: u32) -> Game {
        let optgame: Option<Result<Game, _>> = env.data().get(DataKey::RUNNING(gid));
        match optgame {
            Some(r) => match r {
                Ok(g) => g,
                Err(_) => {
                    panic_with_error!(&env, InvalidErrorCode::Unknown);
                }
            },
            None => {
                panic_with_error!(env, InvalidErrorCode::GameNotFound);
            }
        }
    }

    fn check_player(env: &Env, game: &Game) {
        let player = env.invoker();
        if game.player1 != player && game.player2 != player {
            panic_with_error!(env, InvalidErrorCode::NotAPlayer);
        }

        if (game.next % 2 == 0 && game.player2 == player)
            || (game.next % 2 == 1 && game.player1 == player)
        {
            panic_with_error!(env, InvalidErrorCode::NotYourTurn);
        }
    }

    fn get_square(env: &Env, square: BytesN<2>) -> [u8; 2] {
        let a = square.to_array();
        if a[0] > 2 || a[1] > 2 {
            panic_with_error!(env, InvalidErrorCode::MoveOutOfBound);
        }

        a
    }

    fn get_move(env: &Env, m: [u8; 2], game: &Game) -> u32 {
        let idx: u32 = 1 << (8 - (m[0] + 3 * m[1]));
        let b = game.board.to_u32() >> 9 & 0x1ff;
        let c = game.board.to_u32() & 0x1ff;
        if (b | idx == b) || (c | idx == c) {
            panic_with_error!(env, InvalidErrorCode::InvalidMove);
        }

        idx
    }

    fn get_next_action(player_id: u32, board: u32) -> PlayResult {
        let b = if player_id == 0 {
            board >> 9 & 0x1ff
        } else {
            board & 0x1ff
        };

        for s in SOLUTION {
            if b == s {
                return PlayResult::WINNER;
            }
        }

        if (board >> 9 & 0x1ff | board & 0x1ff) == 0x1ff {
            return PlayResult::DRAW;
        }

        PlayResult::NEXT
    }
}

mod test;
