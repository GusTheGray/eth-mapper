use crate::entities::*;
use neo4rs::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct GraphRepository {
    graph: Arc<Graph>,
}

impl GraphRepository {
    // Initialize the graph
    pub async fn new(uri: &str, user: &str, pass: &str) -> Result<Self> {
        let graph = Arc::new(Graph::new(uri, user, pass).await?);
        Ok(GraphRepository { graph })
    }

    // Function to load the transaction into the graph, along with associated nodes for each of the addresses
    // Relationships are between the transaction and the addresses, as well as the from address and the to address
    pub async fn load_transaction(&self, txn: TransactionEntity) -> Result<()> {
        self.merge_node("Transaction", &txn.hash.to_string())
            .await?;
        self.merge_node("Address", &txn.from.address.to_string())
            .await?;
        self.merge_node("Address", &txn.to.address.to_string())
            .await?;

        self.merge_relationship(
            "Transaction",
            "Address",
            "FROM_ADDRESS",
            &txn.hash.to_string(),
            &txn.from.address.to_string(),
        )
        .await?;
        self.merge_relationship(
            "Transaction",
            "Address",
            "TO_ADDRESS",
            &txn.hash.to_string(),
            &txn.to.address.to_string(),
        )
        .await?;
        self.merge_relationship(
            "Address",
            "Address",
            "TO",
            &txn.from.address.to_string(),
            &txn.to.address.to_string(),
        )
        .await?;
        self.merge_relationship(
            "Address",
            "Address",
            "FROM",
            &txn.to.address.to_string(),
            &txn.from.address.to_string(),
        )
        .await?;

        Ok(())
    }

    // Function to merge a node into the graph
    async fn merge_node(&self, node_type: &str, id: &str) -> Result<()> {
        let graph = self.graph.clone();
        graph
            .run(query(&format!("MERGE (n:{} {{id: $id}})", node_type)).param("id", id))
            .await?;
        Ok(())
    }

    // Function to merge a relationship into the graph
    async fn merge_relationship(
        &self,
        node_type1: &str,
        node_type2: &str,
        relationship: &str,
        id1: &str,
        id2: &str,
    ) -> Result<()> {
        let graph = self.graph.clone();
        graph
            .run(
                query(&format!(
                    "MATCH (n1:{} {{id: $id1}}), (n2:{} {{id: $id2}}) MERGE (n1)-[:{}]->(n2)",
                    node_type1, node_type2, relationship
                ))
                .param("id1", id1)
                .param("id2", id2),
            )
            .await?;
        Ok(())
    }
}
