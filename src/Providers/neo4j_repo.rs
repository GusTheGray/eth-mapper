use crate::entities::*;
use neo4rs::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct GraphRepository {
    graph: Arc<Graph>,
}

impl GraphRepository {
    //Initialize the graph
    pub async fn new(uri: &str, user: &str, pass: &str) -> Result<Self> {
        let graph = Arc::new(Graph::new(uri, user, pass).await?);
        Ok(GraphRepository { graph })
    }

    //Function to load the transaction into the graph, along with associated nodes for each of the addresses
    //Relationships are between the transaction and the addresses, as well as the from address and the to address
    pub async fn load_transaction(&self, txn: TransactionEntity) -> Result<()> {
        let graph = self.graph.clone();
        //Merge the transaction node
        let result = graph
            .run(query("MERGE (t:Transaction {id: $id})").param("id", txn.hash.to_string()))
            .await.unwrap();

        //Merge the address node from the from address
        let result = graph
            .run(query("MERGE (a:Address {id: $id})").param("id", txn.from.address.to_string()))
            .await.unwrap();

        //Merge in the address node from the to address
        let result = graph
            .run(query("MERGE (a:Address {id: $id})").param("id", txn.to.address.to_string()))
            .await.unwrap();

        //Create the relationship from the transaction to the from address
        let result = graph
            .run(query("MATCH (t:Transaction {id: $id1}), (a:Address {id: $id2}) MERGE (t)-[:FROM]->(a)")
                .param("id1", txn.hash.to_string())
                .param("id2", txn.from.address.to_string()))
            .await.unwrap();

        //Create the relationship from the transaction to the to address
        let result = graph
            .run(query("MATCH (t:Transaction {id: $id1}), (a:Address {id: $id2}) MERGE (t)-[:TO]->(a)")
                .param("id1", txn.hash.to_string())
                .param("id2", txn.to.address.to_string()))
            .await.unwrap();

        //Create the relationship from the from address to the to address
        let result = graph
            .run(query("MATCH (a1:Address {id: $id1}), (a2:Address {id: $id2}) MERGE (a1)-[:TO]->(a2)")
                .param("id1", txn.from.address.to_string())
                .param("id2", txn.to.address.to_string()))
            .await.unwrap();

        //Create the relationship from the to address to the from address
        let result = graph
            .run(query("MATCH (a1:Address {id: $id1}), (a2:Address {id: $id2}) MERGE (a1)-[:FROM]->(a2)")
                .param("id1", txn.to.address.to_string())
                .param("id2", txn.from.address.to_string()))
            .await.unwrap();

        Ok(())
    }
}
