use chrono::{Utc};
use serde_derive::{Serialize, Deserialize};
use crypto_hash::{hex_digest, Algorithm};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Transaction {
    pub transaction_id : String,
    pub transaction_timestamp: i64,
    pub transaction_details: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub block_number : u64,
    block_timestamp: i64,
    pub block_nonce: u64,
    pub transaction_list: Vec<Transaction>,
    previous_block: String
}
impl Block {
    pub fn genesis()->Self{
        let transaction = Transaction {transaction_id: String::from("1"), transaction_timestamp: Utc::now().timestamp(), transaction_details: String::from("Dummy transaction")};
        Block { block_number: 1, block_timestamp: Utc::now().timestamp() , block_nonce: 0, transaction_list: vec![transaction], previous_block: String::from("0") }
    }
    pub fn serialize_block(&self) -> String {
        serde_json::to_string(&self).unwrap()
    } 
    pub fn generate_hash(block: &Block) -> String {
        hex_digest(Algorithm::SHA256, block.serialize_block().as_bytes())
    }
    pub fn is_block_valid(hash: &str, prefix: &str)->bool {
        hash.starts_with(prefix)
    }
    pub fn new(transactions: Vec<Transaction>, previous_block: &Block) -> Block {
        Block { block_number: previous_block.block_number + 1, block_timestamp: Utc::now().timestamp(), block_nonce: 1, transaction_list: transactions, previous_block: Self::generate_hash(previous_block) }
    }

    pub fn mine_new_block(block_candidate: &mut Block, prefix:&str) {
        while  !Self::is_block_valid(&Self::generate_hash(block_candidate), prefix) {
            println!("{}", block_candidate.block_nonce);
            block_candidate.block_nonce += 1
        }
    }
}