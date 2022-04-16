# Night Protocol
A simple blockchain written from scratch in the rust programming language. This is a very simple protocol for me to reseach about how the functioning of a blockchain system works. Hoping to make this reachable to the people who are beginners in blockchain and web3 learn about what goes on under the hood of a chain.

## Stack
- Rust
- Nextjs

## How to start the project
- Make sure to have rust and nodejs installed.
- Clone the repo
- `cd` into the `night_protocol` folder and run `cargo run`
- open another terminal instance and  `cd` into the `client` folder and run `yarn && yarn run dev`

## How it works

It has 2 chains for now, the transaction chain and the master chain. When there are more than 20 blocks in the transaction chain, 30 of them are compressed and sent to a block in the master chain. I implemented this algorithm to make the validations faster. Its not peer to peer, but has all the characteristics of a normal chain. You can make transactions and also hopefully play a gambling game in the chain. I'm trying to make the multiple wallet system for now because currently it has support for only one wallet.
