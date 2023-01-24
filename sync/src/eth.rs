use ethers::{
    abi::{Abi, Token, Tokenizable},
    contract::Contract,
    middleware::SignerMiddleware,
    prelude::H256,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, Filter, Log, TransactionRequest, U256, UINT},

};
use ethers_contract_derive::EthEvent;
use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::env;
use dotenv::dotenv;
use std::{
    fs::File,
    io::{Cursor, Read, Seek},
    ops::{Add, Div, Mul, Sub},
};
use cid::Cid;

use manifest::PostEntry;

// Load the contract ABI from Hardhat's artifacts
lazy_static! {
    static ref CONTRACT_ABI: Abi = {
        let contract_abi_ref: &'static str = include_str!("../../contract/artifacts/contracts/blog.sol/Blog.json");
        let contract_abi: serde_json::Value = serde_json::from_str(contract_abi_ref).unwrap();
        let contract_abi = contract_abi["abi"].to_string();
        Abi::try_from(contract_abi).unwrap()
    };
}

impl Tokenizable for Cid {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, ethers::abi::Error> {
        let token = tokens.get(0).ok_or(ethers::abi::Error::InvalidData)?;
        let cid = token.to_string();
        let cid = Cid::try_from(cid)?;
        Ok(cid)
    }

    fn to_tokens(&self) -> Vec<Token> {
        vec![Token::String(self.to_string())]
    }
}

impl Tokenizable for PostEntry {

}

/// Struct emitted when a post is created, deleted, or updated
#[derive(Clone, Debug, Copy, EthEvent)]
pub struct PostMetadata {
    #[ethevent(indexed)]
    pub post_id: UINT,
    #[ethevent(indexed)]
    pub timestamp: U256,
}

/// Enum representing the different actions that can be taken on a post
pub enum PostAction {
    Create,
    Update,
    Delete,
}

/// EthClient - Everything needed to interact with Banyan's Ethereum Stack
pub struct EthClient {
    /// An Eth Provider. This is required to interact with the Ethereum Blockchain.
    provider: Provider<Http>,
    /// The chain ID of the network we're connected to. This is Required for signing transactions.
    chain_id: u64,
    /// An (optional) Eth Signer for singing transactions. This is required for interacting with payable functions.
    signer: Option<SignerMiddleware<Provider<Http>, LocalWallet>>,
    /// A Deployed Solidity Contract Address. This is required to interact with the Banyan Contract.
    contract: Contract<Provider<Http>>,
}

// TODO: Very insecure. Use a proper signer
/// The EthProvider is a wrapper around the ethers-rs Provider that handles all Ethereum
/// interactions.
impl EthClient {
    /// Create a new EthClient - Uses EthClientBuilder::new()
    /// # Arguments
    /// * `api_url` - The URL of the Ethereum API to connect to. This is required to interact with
    ///                 the Ethereum Blockchain.
    /// * `api_key` - The API Key for the Ethereum API. This is required.
    /// * `chain_id` - The (Optional) Chain ID of the network we're connected to.
    ///                 Defaults to 1 (mainnet)
    /// * `private_key` - The (Optional) Private Key for the Ethereum Account we're using to sign.
    ///                 This is required for interacting with payable functions.
    /// * `contract_address` - The (Optional) Deployed Solidity Contract Address to interact with.
    pub fn new(
        api_url: String,
        api_key: String,
        chain_id: Option<u64>,
        private_key: Option<String>,
        contract_address: Address,
    ) -> Result<EthClient, Error> {
        // Determine an API URL and Initialize the Provider
        let url = format!("{}{}", api_url, api_key);
        let provider = Provider::<Http>::try_from(url).expect("Failed to create provider");

        // Get the Chain ID. If None, set to 1 (Eth Mainnet)
        let chain_id = chain_id.unwrap_or(1);

        // Check if we have a private key to set up a Signer
        let signer = if let Some(private_key) = &private_key {
            let wallet = private_key
                .parse::<LocalWallet>()
                .expect("Failed to parse private key");
            Some(SignerMiddleware::new(
                provider.clone(),
                wallet.with_chain_id(chain_id),
            ))
        } else {
            None
        };

        // Check if we have a contract address to set up a Contract
        let contract = Contract::new(contract_address, &CONTRACT_ABI, provider.clone());

        // Determine the timeout as a Duration in seconds, assign default if not provided
        // let timeout = Duration::from_secs(timeout.unwrap_or(15));
        Ok(EthClient {
            provider,
            chain_id,
            signer,
            contract,
        })
    }

    /* State Write Methods */

    /// Write a Post to the Blog backend
    /// # Arguments
    /// * `post_action` - The action to take on the Post. This can be Create, Update, or Delete.
    /// * `post_entry` - The PostEntry to write to the Blog backend.
    /// # Returns
    /// * `PostMetadata` - The PostMetadata emitted by the Blog contract.
    pub async fn write_post(&self, post_action: PostAction, post_entry: PostEntry) -> Result<PostMetadata, Error> {
        // Create the transaction request
        let method = match post_action {
            PostAction::Create => "createPost",
            PostAction::Update => "updatePost",
            PostAction::Delete => "deletePost",
        };
        let tx = self
            .contract
            .method(method, (post_entry.title, post_entry.cid.to_string()))
            .unwrap();
        // Sign the transaction if we have a signer
        let tx = if let Some(signer) = &self.signer {
            signer.sign_transaction(tx, None).await?
        } else {
            tx
        };
        // Send the transaction
        let tx = self.provider.send_transaction(tx, None).await?;
        // Wait for the transaction to be mined
        let tx = self.provider.get_transaction(tx).await?;
        // Get the receipt
        let receipt = self.provider.get_transaction_receipt(tx.hash).await?;
        // Get the PostMetadata from the emitted event
        let post_metadata = PostMetadata::from_log(&receipt.logs[0])?;
        Ok(post_metadata)
    }

    /* State Read Methods */

    /// Get a Post by its post ID
    /// # Arguments
    /// * `post_id` - The post ID of the Post to get
    /// # Returns
    /// * The Post if successful
    /// * Error if not
    pub async fn get_post(&self, post_id: UINT) -> Result<Post, Error> {
        // Create the transaction request
        let tx = self.contract.method("getPost", (post_id,)).unwrap();

        // Send the transaction
        let tx = self.provider.call(tx, None).await?;

        // Get the post from the response
        let post = Post::from_response(tx).unwrap();

        // Return the post
        Ok(post)
    }

    /// Get all Posts
    /// # Returns
    /// * The Posts if successful
}
