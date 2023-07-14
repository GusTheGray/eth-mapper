// provider for retrieving data from alchemy.com's eth APIs using the ethers-rs library
// The provider consists of a singleton connection which is used to retrieve data from the API

use ::std::error::Error;
use ethers::{
    providers::{Middleware, Provider, Ws},
    types::Transaction,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AlchemyProvider {
    pub provider: Arc<Mutex<Provider<Ws>>>,
}

//establish a connection to the alchemy websocket API
impl AlchemyProvider {
    pub async fn new(url: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let provider = Provider::<Ws>::connect(url).await?;
        let provider = Arc::new(Mutex::new(provider));
        Ok(AlchemyProvider { provider })
    }

    //Get all transactions associated with a given block number
    pub async fn get_block_transactions(&self, block_number: u64) -> Vec<Transaction> {
        let provider = self.provider.lock().await;
        let block = provider
            .get_block_with_txs(block_number)
            .await
            .unwrap()
            .unwrap();
        let transactions = block.transactions;
        transactions
    }

    //get the latest block number
    pub async fn get_latest_block_number(&self) -> u64 {
        let provider = self.provider.lock().await;
        let block_number = provider.get_block_number().await.unwrap();
        block_number.as_u64()
    }
}
