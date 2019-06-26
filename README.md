In order to reproduce the error:

First, run ganache

    ganache-cli -m "hamster coin cup brief quote trick stove draft hobby strong caught unable"

Using this mnemonic makes the static account addresses in the example line up


Do migration with truffle (truffle needs to be install globally - npm install -g truffle)

    cd src
    truffle migrate

Make sure that the contract address SimpleStorage matches one used in the source code 

Now we can run an example

    cargo run --bin simple_storage

It should show that the following line throws:
        let result_set = contract.call("set", (42,), accounts[0], Options::default()).wait().unwrap();

If we switch in the cargo.toml, web3 = "0.7.0", the same line is just printing the tx-hash.
