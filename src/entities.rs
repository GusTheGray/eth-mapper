use ethers::types::{Transaction, H160, H256, U256};

//data model and mapper for a transaction to the model being used internal
//data model for address is a string containing the address only

#[derive(Debug)]
pub struct AddressEntity {
    pub address: Option<H160>,
}

//data model for the transaction, indicating the from address, to address, block number, and the value of the transaction
#[derive(Debug)]
pub struct TransactionEntity {
    pub from: AddressEntity,
    pub to: AddressEntity,
    pub block_number: u64,
    pub value: U256,
    pub hash: H256,
}

// implementation of a mapper to go from the AlchemyTransaction to the Transaction model
impl From<Transaction> for TransactionEntity {
    fn from(txn: Transaction) -> Self {
        TransactionEntity {
            from: AddressEntity { address: Some(txn.from) },
            to: AddressEntity {
                address: txn.to,
            },
            block_number: txn.block_number.unwrap().as_u64(),
            value: txn.value,
            hash: txn.hash,
        }
    }
}

// implementation of a mapper to go from the address string to the Address model
impl From<String> for AddressEntity {
    fn from(address: String) -> Self {
        AddressEntity {
            address: address.parse::<H160>().ok(),
        }
    }
}
