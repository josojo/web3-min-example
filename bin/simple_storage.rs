extern crate rustc_hex;
extern crate web3;

use std::time;
use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::U256;

fn main() {
    let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().wait().unwrap();

    //Get current balance
    let balance = web3.eth().balance(accounts[0], None).wait().unwrap();

    println!("Balance: {}", balance);

    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("./../src/build/contracts_SimpleStorage_sol_SimpleStorage.bin");
    // Deploying a contract
    let contract = Contract::deploy(web3.eth(), include_bytes!("./../src/build/contracts_SimpleStorage_sol_SimpleStorage.abi"))
        .unwrap()
        .confirmations(0)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(6_000_000.into())))
        .execute(bytecode, (), accounts[0])
        .unwrap()
        .wait()
        .unwrap();

    println!("{}", contract.address());


    //Change state of the contract
    let result_set = contract.call("set", (42,), accounts[0], Options::default()).wait().unwrap();
    println!(".call return {}", result_set);

    //interact with the contract
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.wait().unwrap();
    println!(".query return {}", storage);

    //View changes made
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: U256 = result.wait().unwrap();
    println!("{}", storage);
}
