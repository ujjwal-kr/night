use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug)]
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
                    sender: "test".to_string(),
                    reciever: "net".to_string(),
                    amount: 500.0,
                },
            )],
        }
    }

    pub fn genesus(&mut self) {
        let genesus = Block::new(0, "genesus".to_string(), Transaction {
            sender: "test".to_string(),
            reciever: "net".to_string(),
            amount: 500.0,
        });

        self.blocks.push(genesus);
    }

    pub fn add_block(&mut self, transactions: Transaction) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new(
            prev_block.id + 1,
            prev_block.clone().block_hash,
            transactions,
        );
        self.blocks.push(new_block);
    }

    pub fn validate_block(&self, block: &Block, previous_block: &Block) -> bool {
        if previous_block.block_hash.trim() == block.previous_hash.trim() {
            true
        } else {
            false
        }
    }

    pub fn validate_chain(&self) {
        for i in 1..self.blocks.len() {
            let block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1].clone();
            if self.validate_block(block, prev_block) == false {
                println!("block bad");
                return;
            }
        }
        println!("Validated All Blocks");
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
