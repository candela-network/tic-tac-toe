#![no_std]
mod helper;
mod types;
use crate::types::{Game, GameState, PlayResult, TicTacToeEvent, EVENT_TOPIC};
use helper::TicTacToeEngine;
use soroban_sdk::{contractimpl, symbol, Address, BytesN, Env};

pub struct TicTacToeContract;

#[contractimpl]
impl TicTacToeContract {
    /*
        Launch a new game.
        If no one awaits in the lobby, the invoker of launch() will be placed in the lobby (the method return GameState::PENDING)
        If a GameState::PENDING exists, a game is launched and the play() function can be called the with game_id returned in GameState::RUNNING(game_id).
        Else, the invoker of launch() will be placed in the lobby (the function returns GameState::PENDING)
    */
    pub fn launch(env: Env) -> GameState {
        let pending: Address = env.data().get(types::GameState::PENDING).unwrap_or(Ok(env.invoker())).unwrap();

        if pending != env.invoker() {
            let mut counter = env.data().get(GameState::COUNTER).unwrap_or(Ok(0)).unwrap();

            // Increment game id
            counter += 1;
            env.data().set(GameState::COUNTER, counter);

            // Store the new game
            env.data().set(
                GameState::RUNNING(counter),
                Game {
                    player1: pending,
                    player2: env.invoker(),
                    board: 0x00,
                    next: 0,
                },
            );

            // Clean up pending
            env.data().remove(GameState::PENDING);

            return GameState::RUNNING(counter);
        } else {
            env.data().set(GameState::PENDING, pending);
            return GameState::PENDING;
        }
    }

    /*
        Play a move for a given game id.
    */
    pub fn play(env: Env, game_id: u32, square: BytesN<2>) -> PlayResult {
        let mut game = TicTacToeEngine::get_game(&env, game_id);
        let m = TicTacToeEngine::get_square(&env, square);

        TicTacToeEngine::check_player(&env, &game);

        game.board = TicTacToeEngine::check_and_move(&env, m, &game);

        // Get the next game action
        let result = TicTacToeEngine::get_next_action(&game);

        if result == PlayResult::NEXT {
            game.next += 1;
            env.data().set(GameState::RUNNING(game_id), &game);
        } else {
            env.data().remove(GameState::RUNNING(game_id));
        }

        // Publish events
        env.events().publish(
            (EVENT_TOPIC, symbol!("play")),
            TicTacToeEvent {
                id: game_id,
                game: game,
                result: result.clone(),
            },
        );

        result
    }
}

mod test;
