use eth_mapper::settings;
use eth_mapper::providers::alchemy_provider::AlchemyProvider;


#[tokio::main]
async fn main() {
    //establish a new instance of AlchemyProvider
    let settings = settings::Settings::new().unwrap();
    let alchemy_provider = AlchemyProvider::new(settings.alchemy_websocket.url).await;

    let txns = alchemy_provider.get_block_transactions(17500980).await;
    println!("{:?}", txns);
}
