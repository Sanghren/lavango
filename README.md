# lavango

Lavango means `avalanche` in [Esperanto](https://en.wikipedia.org/wiki/Esperanto). I have the habit to name my projects 
in esperanto ;)

An attempt at implementing a bare bone VM in Rust for Avalanche, using the liner chain API.

This will -maybe- be a project participating in https://gitcoin.co/issue/ava-labs/open-defi-hackathon/3/100026354

## Objectives

My first objective with this is to have a simple VM that can communicate over gRpc with an AvalancheGo node.

My initial plan is to have something similar as the timestampVm.

## How to use it as a avalanchego plugin

It's easy, you should just build the binary : 
```shell
cargo build --release
```

Take the binary created at `target/release/tGas3T58KzdjLHhBDMnH2TvrddhqTji5iZAMZ3RXs2NLpSnhH` (For now am reusing the 
base 58 name of the timestampVm) and put it in the `build/plugins` folder of your avalanchego node. (Location could vary
depending of how you run the node).

Launch your avalanchego node and it should ... not crash upon loading `tGas3T58KzdjLHhBDMnH2TvrddhqTji5iZAMZ3RXs2NLpSnhH`.
But it won't do much more than thise for now.

Bonus, you can debug lavango by doing this little trick: 
- in `src/rpcchainvm/server.rs` , change the port at this line to something else (eg. 10001) `println!("1|5|tcp|127.0.0.1:10002|grpc");`
- `cargo build --release` and put the resulting binary in the plugin folder of your avalanchego node.
- in `src/rpcchainvm/server.rs` , change the port at this line to 10001 `let addr = "127.0.0.1:10001".parse().unwrap();`
- Run it in debug mode
- Start avalanchego, and now it'll try to contact the version of your VM running in debug mode !


