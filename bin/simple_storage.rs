extern crate rustc_hex;
extern crate web3;

use std::time;
use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::{U256, Address};

fn main() {
    let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().wait().unwrap();

    //Get current balance
    let balance = web3.eth().balance(accounts[0], None).wait().unwrap();

    println!("Balance: {}", balance);

    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("./../src/build/contracts_SimpleStorage_sol_SimpleStorage.bin");
    
    let address = Address::from("0xec0b9Ed45c0357A4539DF79bA7cF1259A2Cf4adD");

    let contract = Contract::from_json(web3.eth(), address, include_bytes!("./../src/build/contracts_SimpleStorage_sol_SimpleStorage.abi")).unwrap();

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
