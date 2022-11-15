#![no_std]
use soroban_sdk::{
    contractimpl, contracttype, Address, Bytes, Env, bytes
};

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum DataKey {
    PENDING,
    RUNNING(u32),
    COUNTER,
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
}

#[contracttype]
struct Game {
    player1: Address,
    player2: Address,
    board: Bytes,
}

mod test;
