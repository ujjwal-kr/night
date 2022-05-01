use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: i64,
    pub previous_hash: String,
    pub transaction: Transaction,
    pub block_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub reciever: String,
    pub amount: f64,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::new(
                0,
                "genesus".to_string(),
                Transaction {
                    sender: "net".to_string(),
                    reciever: "user".to_string(),
                    amount: 1000.0,
                },
            )],
        }
    }

    pub fn genesus(&mut self) {
        let genesus = Block::new(0, "genesus".to_string(), Transaction {
            sender: "net".to_string(),
            reciever: "user".to_string(),
            amount: 0.0,
        });

        self.blocks.push(genesus);
    }

    pub fn add_block(&mut self, transaction: Transaction) {
        if self.validate_chain() == true {
            let prev_block = &self.blocks[self.blocks.len() - 1];
            let new_block = Block::new(
                prev_block.id + 1,
                prev_block.clone().block_hash,
                transaction,
            );
            self.blocks.push(new_block);
        } else {
            return
        }
    }

    pub fn validate_block(&self, block: &Block, previous_block: &Block) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}-{}-{:?}", previous_block.id, previous_block.previous_hash, previous_block.transaction));
        let previous_block_hash: String = format!("{:x}", hasher.finalize());
        if previous_block_hash.trim() == block.previous_hash.trim() {
            true
        } else {
            false
        }
    }

    pub fn validate_chain(&self) -> bool {
        for i in 1..self.blocks.len() {
            let block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1].clone();
            if self.validate_block(block, prev_block) == false {
                println!("Blockchain validation failed, aborting transaction.");
                return false
            }
        }
        return true
    }

    pub fn find_block_by_hash(&self, hash: String) -> Block {
        for block in self.blocks.clone() {
            if block.block_hash.trim() == hash.trim() {
                return block;
            }
        }

        Block {
            id: 0,
            block_hash: "null".to_string(),
            previous_hash: "null".to_string(),
            transaction: Transaction {
                sender: "null".to_string(),
                reciever: "null".to_string(),
                amount: 0.00,
            },
        }
    }

    pub fn calculate_balance(&self) -> f64 {
        let mut balance: f64 = 0.0;
        if self.validate_chain() == false {
            return balance
        }
        for block in &self.blocks {
            if block.transaction.sender == "user".to_string() {
                balance = balance - block.transaction.amount;
            } else {
                balance = balance + block.transaction.amount;
            }
        }
        balance
    }
}

impl Block {
    pub fn new(id: i64, previous_hash: String, transaction: Transaction) -> Block {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}-{}-{:?}", id, previous_hash, transaction));
        let block_hash_str: String = format!("{:x}", hasher.finalize());
        Block {
            id: id,
            block_hash: block_hash_str,
            previous_hash: previous_hash,
            transaction: transaction,
        }
    }
}

