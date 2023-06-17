// provider for retrieving data from alchemy.com's eth APIs using the ethers-rs library
// The provider consists of a singleton connection which is used to retrieve data from the API

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
    pub async fn new(url: String) -> Self {
        let provider = Provider::<Ws>::connect(url).await.unwrap();
        let provider = Arc::new(Mutex::new(provider));
        AlchemyProvider { provider }
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
}
