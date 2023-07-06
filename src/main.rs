use eth_mapper::entities::TransactionEntity;
use eth_mapper::providers::alchemy_provider::AlchemyProvider;
use eth_mapper::providers::neo4j_repo::GraphRepository;
use eth_mapper::settings;

#[tokio::main]
async fn main() {
    //establish a new instance of AlchemyProvider
    let settings = settings::Settings::new().unwrap();
    let alchemy_provider = AlchemyProvider::new(settings.alchemy_websocket.url).await;


    let txns = alchemy_provider.get_block_transactions(17500980).await;
    println!("=== got {} transactions ===", txns.len());

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
    for txn in txns {
        let txn_entity = TransactionEntity::from(txn);
        graph.load_transaction(txn_entity).await.unwrap();
    }
}
