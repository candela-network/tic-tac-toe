#![cfg(test)]

use crate::PlayResult;

use super::{DataKey, TicTacToeContract, TicTacToeContractClient};
use soroban_sdk::{
    bytesn,
    testutils::{Accounts, Logger},
    Env,
};

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

    std::println!("Log {}", env.logger().all().join("\n"));
}

#[test]
fn test_play() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    let r = client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 0]));

    assert_eq!(r, PlayResult::NEXT);

    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [2, 2]));

    std::println!("{:?}", r);
    let logs = env.logger().all();
    std::println!("{}", logs.join("\n"));
}

#[test]
fn test_play_user1_win() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 0]));
    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [2, 2]));
    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 1]));
    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [2, 1]));
    let r = client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 2]));

    let logs = env.logger().all();
    std::println!("{}", logs.join("\n"));
    assert_eq!(r, PlayResult::WINNER);

    std::println!("{:?}", r);
}

#[test]
fn test_play_user2_win() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 0]));
    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [2, 2]));
    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 1]));
    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [2, 1]));
    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [1, 0]));

    let r = client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [2, 0]));

    let logs = env.logger().all();
    std::println!("{}", logs.join("\n"));
    assert_eq!(r, PlayResult::WINNER);

    std::println!("{:?}", r);
}

#[test]
fn test_play_draw() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 0]));
    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [0, 1]));
    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 2]));
    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [1, 0]));
    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [1, 1]));

    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [1, 2]));

    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [2, 0]));

    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [2, 1]));

    let r = client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [2, 2]));

    let logs = env.logger().all();
    std::println!("{}", logs.join("\n"));
    assert_eq!(r, PlayResult::DRAW);

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

    client
        .with_source_account(&user_3)
        .play(&1, &bytesn!(&env, [0, 0]));
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    client
        .with_source_account(&user_1)
        .play(&42, &bytesn!(&env, [0, 0]));
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]
fn test_not_your_turn1() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [0, 1]));

    // client
    //     .with_source_account(&user_1)
    //     .play(&1, &bytesn!(&env, [1, 0]));
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]
fn test_not_your_turn2() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 1]));

    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [1, 0]));
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_move_out_bound1() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [3, 0]));
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_move_out_bound2() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 3]));
}

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn test_invalid_move() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicTacToeContract);
    let client = TicTacToeContractClient::new(&env, &contract_id);

    let user_1 = env.accounts().generate();
    let user_2 = env.accounts().generate();

    client.with_source_account(&user_1).launch();
    client.with_source_account(&user_2).launch();

    client
        .with_source_account(&user_1)
        .play(&1, &bytesn!(&env, [0, 1]));

    client
        .with_source_account(&user_2)
        .play(&1, &bytesn!(&env, [0, 1]));
}
