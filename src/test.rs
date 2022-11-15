#![cfg(test)]

use super::{TicTacToeContract, TicTacToeContractClient, DataKey};
use soroban_sdk::{testutils::{Logger, Accounts}, Env, log};

extern crate std;

#[test]
fn test_launch() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    let r1 = client.with_source_account(&user_1).launch();
    let r2 = client.with_source_account(&user_2).launch();
    
    assert_eq!(r1, DataKey::PENDING);
    assert_eq!(r2, DataKey::RUNNING(1));

    std::println!("{}", env.logger().all().join("\n"));
}