use eth_mapper::entities::TransactionEntity;
use eth_mapper::providers::alchemy_provider::AlchemyProvider;
use eth_mapper::providers::neo4j_repo::GraphRepository;
use eth_mapper::settings;
use ethers::types::Res;
use futures::future::join_all;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() {
    //establish a new instance of AlchemyProvider
    let settings = settings::Settings::new().unwrap();
    let alchemy_provider = AlchemyProvider::new(settings.alchemy_websocket.url).await;

    let starting_block = 17500980;
    let number_of_blocks = 100;

    println!("=== getting {} blocks starting at {} ===", number_of_blocks, starting_block);

    let futures = (starting_block..starting_block + number_of_blocks)
        .map(|block_number| alchemy_provider.get_block_transactions(block_number));
    let all_txns = join_all(futures)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    println!("=== got {} transactions ===", all_txns.len());

    println!("=== connecting to neo4j ===");
    //establish a new instance of GraphRepository
    let graph = GraphRepository::new(
        settings.db_settings.url.as_str(),
        settings.db_settings.user.as_str(),
        settings.db_settings.password.as_str(),
    )
    .await
    .unwrap();

    println!("=== loading transactions into neo4j ===");

    //load the transactions into the graph
    let pb = ProgressBar::new(all_txns.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    let batch_size = 100;

    let txn_stream = stream::iter(all_txns.into_iter());

    txn_stream
        .chunks(batch_size)
        .for_each_concurrent(None,|txn_batch| async {
            let txn_entities: Vec<_> = txn_batch.into_iter().map(|txn| TransactionEntity::from(txn)).collect();
            graph.load_transactions(&txn_entities).await.unwrap_or_else(|e| {
                println!("error loading transactions: {:?}", e);
            });
            pb.inc(txn_entities.len() as u64);
        })
        .await;
    
}
