# Stylus Call Contract PoC

This repository contains playground for calling Solidity contract from a Stylus one.

## Deploy Solidity Contract
Run the following command:

```sh
cd SolidityReceiver
forge create src/SolidityReceiver.sol:SolidityReceiver --rpc-url=<rpc-url> --private-key=<private-key>
```

Your Solidity contract should be deployed with similar output:
```
...
Deployer: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
Deployed to: 0x5FbDB2315678afecb367f032d93F642f64180aa3
Transaction hash: 0x9659a9553e61d6a3161c2b2e73feeab245276f10b5bda0ce53489925de13a585
```

Save `Deployed to:` address as it will be needed in contracts interaction `<solidity-contract-address>`.

## Deploy Stylus Contract

```sh
cd stylus-call
cargo stylus deploy --private-key=<private-key> -e=<rpc-url> --no-verify
```

Your Stylus contract should be deployed with similar output:
```
...
contract size: 17.9 KB
wasm size: 50.9 KB
File used for deployment hash: ./Cargo.lock
File used for deployment hash: ./Cargo.toml
File used for deployment hash: ./rust-toolchain.toml
File used for deployment hash: ./src/lib.rs
File used for deployment hash: ./src/main.rs
project metadata hash computed on deployment: "9fa97fb6275090736ac9b2706c6f747805c4c3a7bc917e0fcd08eeaeacde08fe"
contract size: 17.9 KB
wasm data fee: Ξ0.000095
deployed code at address: 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512
deployment tx hash: 0x974844a5dbf42d960c71f977e70ca924db61e7b630a87688b4e9ae6c895b87db
...
```

Save `deployed code at address:` address as it will be needed in contracts interaction `<stylus-contract-address>`.

## Interact with the smart contracts
```sh
cast send --private-key <private-key> --rpc-url <rpc-url> <stylus-contract-address> "dummy(address,address,uint256,bytes)(bytes4)" <solidity-contract-address> <random-uint256> <random-bytes>
# e.g. cast send --private-key <private-key> --rpc-url http://localhost:8547 0x9a676e781a523b5d0c0e43731313a708cb607508 "dummy(address,uint256,bytes)(bytes4)" 0x959922bE3CAee4b8Cd9a407cc3ac1C251C2007B1 12 0xdeadbeef 
```


