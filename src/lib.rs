#![no_std]
use soroban_sdk::{
    contractimpl, contracttype, Address, Bytes, Env, bytes, Symbol, BytesN, contracterror, panic_with_error
};

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum DataKey {
    PENDING,
    RUNNING(u32),
    COUNTER,
}

#[contracttype]
struct Game {
    player1: Address,
    player2: Address,
    board: Bytes,
    next: u32
}

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum PlayResult {
    NEXT,
    INVALID(u32),
    WIN,
    NOTFOUND(u32)
}

#[contracterror]
#[derive(Debug, PartialEq, Clone, Copy)]
enum InvalidErrorCode {
    Unknown = 0,
    NotAPlayer = 1,
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
            let mut counter = env.data()
                .get(DataKey::COUNTER)
                .unwrap_or(Ok(0))
                .unwrap();

            // Increment game id
            counter += 1;
            env.data().set(DataKey::COUNTER, counter);

            // Store the new game
            env.data().set(
                DataKey::RUNNING(counter),
                Game {
                    player1: pending,
                    player2: env.invoker(),
                    board: bytes![&env, [0, 0, 0, 0, 0, 0, 0, 0, 0]],
                    next: 0
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

    pub fn play(env: Env, game_id: BytesN<4>, square: Symbol) -> PlayResult {

        let gid_arr = game_id.to_array();
        let gid = ((gid_arr[0] as u32) << 24) + ((gid_arr[1] as u32) << 16) + ((gid_arr[2] as u32) << 8) + ((gid_arr[3] as u32) << 0);

        let optgame: Option<Result<Game, _>> = env.data().get(DataKey::RUNNING(gid));

        let game =  match optgame {
            Some(r) => match r {
                Ok(g) => g,
                Err(_) => {
                    return PlayResult::INVALID(gid);
                }
            },
            None => {
                return PlayResult::NOTFOUND(gid);
            },
        };

        
        Self::is_player(env, game);

        PlayResult::NEXT
    }


    fn is_player(env: Env, game: Game) {

        let player = env.invoker();
        if game.player1 != player && game.player2 != player {
            panic_with_error!(env, InvalidErrorCode::NotAPlayer);
        }
    }
}



mod test;
