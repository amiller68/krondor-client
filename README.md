# Krondor Client

This is a client I wrote for my Web3 blog template. It's coordinates:
- A daemon that monitors the contents of a folder and:
  - Syncs the contents of the folder in with an Estuary node running at a given address using rclone
  - Pins the contents of the folder on an IPFS node running on the same machine
  - Pushes the metadata of the folder to a given Ethereum address that is running a smart contract for the blog
  - Maintains a local cache of the contents of the folder that reflects the blog contents on the blockchain

As a first draft, it will use the following architecture:
- The daemon will be written in Rust
- The backend will be written in solidity and deployed to Polygon
- The IPFS node will be a node accessible via a lite client

## Setup

### Prerequisites
- hardhat
- rust
- docker

### Contracts

See the [contracts](./contracts/README.md) folder for info on configuring and deploying the contracts.
