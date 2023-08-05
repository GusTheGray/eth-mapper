use crossbeam::channel::bounded;
use eth_mapper::entities::TransactionEntity;
use eth_mapper::providers::alchemy_provider::AlchemyProvider;
use eth_mapper::providers::neo4j_repo::GraphRepository;
use eth_mapper::settings;
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    //establish a new instance of AlchemyProvider
    let settings = settings::Settings::new().expect("Unable to collect configs:");
    let alchemy_provider = AlchemyProvider::new(settings.alchemy_websocket.url)
        .await
        .expect("Unable to init provider");

    let number_of_blocks = 1000;

    //Get latest block number and set to starting_block
    let latest_block = alchemy_provider
        .get_latest_block_number()
        .await
        .expect("Unable to get latest block number");
    let starting_block = latest_block - number_of_blocks;

    print!("=== getting {} blocks, starting with latest block {} and going back ===", number_of_blocks, starting_block);

    let futures = (starting_block..latest_block)
        .map(|block_number| alchemy_provider.get_block_transactions(block_number));

    let all_txns: Vec<TransactionEntity> = join_all(futures)
        .await
        .into_iter()
        .flatten()
        .flatten()
        .collect();

    println!("=== got {} transactions ===", all_txns.len());

    println!("=== loading transactions into neo4j ===");

    //prepare to load the transactions into the graph

    let batch_size = 100;

    let graph = Arc::new(tokio::sync::Mutex::new(
        GraphRepository::new(
            settings.db_settings.url.as_str(),
            settings.db_settings.user.as_str(),
            settings.db_settings.password.as_str(),
        )
        .await
        .unwrap(),
    ));

    //load up the transactions using several parallel streams

    // Send transactions to the thread using the channel

    let (sender, receiver) = bounded::<TransactionEntity>(batch_size);

    //setup progress bar
    let pb = ProgressBar::new(all_txns.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Spawn several threads to process the transactions
    let workers = (0..5)
        .map(|_| {
            let receiver = receiver.clone();
            let graph = graph.clone();
            tokio::spawn(async move {
                while let Ok(txn) = receiver.recv() {
                    let graph = graph.lock().await;
                    graph
                        .load_transaction(&txn)
                        .await
                        .expect(format!("Unable to load transaction: {:?}", txn).as_str());
                }
            })
        })
        .collect::<Vec<_>>();

    // Send the transactions to the threads
    for txn in all_txns {
        sender.send(txn).unwrap();
    }

    // Drop the sender so the threads can finish
    drop(sender);

    // Wait for the threads to finish
    for worker in workers {
        worker.await.unwrap();
    }

    println!("=== done ===");
}
