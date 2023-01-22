# Krondor Blog Contract

This implements the smart contract for the Krondor blog. It is written in Solidity and deployed to Polygon.
It can be deployed to any Ethereum network.

## Prerequisites

- hardhat

## Setup

- Copy `.env.example` to `.env` and fill in the values
- Run `yarn install` to install dependencies

## Maintaining

- Run 'yarn prettier' to format the code
- Run 'yarn lint' to lint the code

## Deployment

```bash
yarn build # Compile the contracts
yarn test # Run the tests
yarn deploy <network> # Deploy the contracts to the given network, such as 'matic' or 'mumbai'
yarn verify <address> <network> # Verify the contract on the network's Etherscan using the given address
```
