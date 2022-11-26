use crate::types::*;
use soroban_sdk::{panic_with_error, BytesN, Env};

const SOLUTION: [u32; 8] = [0b111000000, 0b000111000, 0b000000111, 0b100100100, 0b010010010, 0b001001001, 0b100010001, 0b001010100];

pub struct TicTacToeEngine;

impl TicTacToeEngine {
    pub fn get_game(env: &Env, gid: u32) -> Game {
        let optgame: Option<Result<Game, _>> = env.data().get(GameState::RUNNING(gid));
        match optgame {
            Some(r) => match r {
                Ok(g) => g,
                Err(_) => {
                    panic_with_error!(&env, InvalidErrorCode::Unknown);
                }
            },
            None => {
                panic_with_error!(&env, InvalidErrorCode::GameNotFound);
            }
        }
    }

    pub fn check_player(env: &Env, game: &Game) {
        let player = env.invoker();
        if game.player1 != player && game.player2 != player {
            panic_with_error!(env, InvalidErrorCode::NotAPlayer);
        }

        if (game.next % 2 == 0 && game.player2 == player) || (game.next % 2 == 1 && game.player1 == player) {
            panic_with_error!(env, InvalidErrorCode::NotYourTurn);
        }
    }

    pub fn get_square(env: &Env, square: BytesN<2>) -> [u8; 2] {
        let a = square.to_array();
        if a[0] > 2 || a[1] > 2 {
            panic_with_error!(env, InvalidErrorCode::MoveOutOfBound);
        }

        a
    }

    pub fn check_and_move(env: &Env, m: [u8; 2], game: &Game) -> u32 {
        let mask: u32 = 1 << (8 - (m[0] + 3 * m[1]));
        let b = game.board >> 9 & 0x1ff;
        let c = game.board & 0x1ff;
        if (b | mask == b) || (c | mask == c) {
            panic_with_error!(env, InvalidErrorCode::InvalidMove);
        }

        if game.next % 2 == 0 {
            game.board | (mask << 9)
        } else {
            game.board | mask
        }
    }

    pub fn get_next_action(game: &Game) -> PlayResult {
        let p1 = game.board >> 9 & 0x1ff;
        let p2 = game.board & 0x1ff;
        let b = if game.next % 2 == 0 { p1 } else { p2 };

        for s in SOLUTION {
            if b == s {
                return PlayResult::WINNER;
            }
        }

        if (p1 | p2) == 0x1ff {
            return PlayResult::DRAW;
        }

        PlayResult::NEXT
    }
}
