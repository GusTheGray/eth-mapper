// provider for retrieving data from alchemy.com's eth APIs using the ethers-rs library
// The provider consists of a singleton connection which is used to retrieve data from the API

use ::std::error::Error;
use ethers::providers::{Middleware, Provider, Ws};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::entities::TransactionEntity;

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
    pub async fn get_block_transactions(
        &self,
        block_number: u64,
    ) -> Result<Vec<TransactionEntity>, Box<dyn Error + Send + Sync>> {
        let provider = self.provider.lock().await;
        let block = provider
            .get_block_with_txs(block_number)
            .await?
            .ok_or("Block not found")?;
        Ok(block
            .transactions
            .into_iter()
            .map(|tx| TransactionEntity::from(tx))
            .collect())
    }

    //get the latest block number
    pub async fn get_latest_block_number(&self) -> Result<u64, Box<dyn Error + Send + Sync>> {
        let provider = self.provider.lock().await;
        let block_number = provider.get_block_number().await?;
        Ok(block_number.as_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings;

    #[tokio::test]
    async fn test_get_block_transactions() {
        let settings = settings::Settings::new().expect("Unable to collect configs:");
        let alchemy_provider = AlchemyProvider::new(settings.alchemy_websocket.url)
            .await
            .expect("Unable to init provider");
        let block_number = 17500980;
        let txns = alchemy_provider
            .get_block_transactions(block_number)
            .await
            .expect("Unable to get block transactions");
        assert!(!txns.is_empty());
    }

    #[tokio::test]
    async fn test_get_latest_block_number() {
        let settings = settings::Settings::new().expect("Unable to collect configs:");
        let alchemy_provider = AlchemyProvider::new(settings.alchemy_websocket.url)
            .await
            .expect("Unable to init provider");
        let block_number = alchemy_provider.get_latest_block_number().await;
        assert!(block_number.is_ok());
    }
}
