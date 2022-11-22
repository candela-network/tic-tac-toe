#![no_std]
use soroban_sdk::{
    bytes, contracterror, contractimpl, contracttype, panic_with_error, Address, Bytes, BytesN,
    Env, Symbol, bytesn,
};

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum DataKey {
    PENDING,
    RUNNING(u32),
    COUNTER,
}

#[contracttype]
#[derive(Clone)]
struct Game {
    player1: Address,
    player2: Address,
    board: BytesN<9>,
    next: u32,
}

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum PlayResult {
    NEXT(u32),
    INVALID(u32),
    WIN,
    NOTFOUND(u32),
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
                    board: bytesn![&env, [0, 0, 0, 0, 0, 0, 0, 0, 0]],
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

        let played_move = Self::get_move(&env, m, &game);

        let player_value = (game.next % 2) + 1;
        let mut mgame = game;
        mgame.board.to_array()[played_move] = player_value as u8;
        mgame.next += 1;

        env.data().set(
            DataKey::RUNNING(game_id),
            mgame,
        );


        PlayResult::NEXT(game_id)
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

        if game.next % 2 == 0 && game.player2 == player {
            panic_with_error!(env, InvalidErrorCode::NotYourTurn);
        }


    }

    fn get_square(env: &Env, square: BytesN<2>) -> [u8; 2] {
        let a = square.to_array();
        if a [0] > 2 || a[1] > 2 {
            panic_with_error!(env, InvalidErrorCode::MoveOutOfBound);
        }

        a
    }

    fn get_move(env: &Env, m: [u8; 2],  game: &Game) -> usize {

        let idx = (m[0] + 3 * m[1]) as usize;
        if game.board.to_array()[idx] > 0 {
            panic_with_error!(env, InvalidErrorCode::InvalidMove);
        }

        idx
    }
}

mod test;
