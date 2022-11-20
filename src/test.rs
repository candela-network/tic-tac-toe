#![cfg(test)]

use crate::PlayResult;

use super::{TicTacToeContract, TicTacToeContractClient, DataKey};
use soroban_sdk::{testutils::{Logger, Accounts}, Env, log, symbol, bytesn};

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

#[test]
fn test_play() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let id: [u8; 4] = 42_u32.to_be_bytes();
    let r = client.play(&bytesn!(&env, [id[0], id[1], id[2], id[3]]), &symbol!("test"));

    assert_eq!(r, PlayResult::NOTFOUND(42));

    std::println!("{:?}", r);
}

#[test]
fn test_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    let id: [u8; 4] = 42_u32.to_be_bytes();
    let r = client.with_source_account(&user_1).play(&bytesn!(&env, [id[0], id[1], id[2], id[3]]), &symbol!("test"));

    assert_eq!(r, PlayResult::NOTFOUND(42));

    std::println!("{:?}", r);
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_invalid_player() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();
    let user_3 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    let id: [u8; 4] = 1_u32.to_be_bytes();
    let r = client.with_source_account(&user_3).play(&bytesn!(&env, [id[0], id[1], id[2], id[3]]), &symbol!("test"));

    assert_eq!(r, PlayResult::NEXT);
}