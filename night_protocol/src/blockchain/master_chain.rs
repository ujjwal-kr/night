use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::blockchain::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Master {
    pub master_blocks: Vec<MasterBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterBlock {
    pub id: i64,
    pub previous_hash: String,
    pub block_hash: String,
    pub block_data: Vec<Block>,
}

impl Master {
    pub fn new() -> Self {
        Self {
            master_blocks: vec![MasterBlock::new(
                0,
                "genesus".to_string(),
                [Block {
                    id: -1,
                    previous_hash: "abcd".to_string(),
                    block_hash: "efgh".to_string(),
                    transaction: Transaction {
                        sender: "test".to_string(),
                        reciever: "net".to_string(),
                        amount: 0.0,
                    },
                }]
                .to_vec(),
            )],
        }
    }

    pub fn add_master_block(&mut self, block_data: Vec<Block>) {
        if self.validate_chain() == true {
            let prev_block = &self.master_blocks[&self.master_blocks.len() - 1];
            let new_block =
                MasterBlock::new(prev_block.id + 1, prev_block.clone().block_hash, block_data);
            self.master_blocks.push(new_block);
        } else {
            return
        }
    }

    pub fn validate_master_block(&self, block: &MasterBlock, previous_block: &MasterBlock) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}-{}-{:?}", previous_block.id, previous_block.previous_hash, previous_block.block_data));
        let previous_block_hash: String = format!("{:x}", hasher.finalize());

        if block.previous_hash.trim() == previous_block_hash.trim() {
            true
        } else {
            false
        }
    }

    pub fn validate_chain(&self) -> bool {
        for i in 1..self.master_blocks.len() {
            let block = &self.master_blocks[i];
            let prev_block = &self.master_blocks[i - 1].clone();
            if self.validate_master_block(block, prev_block) == false {
                println!("Master chain validation failed, aborting.");
                return false
            }
        }
        println!("Validated All Master Blocks");
        return true
    }

    pub fn find_blocks_by_master_id(&self, id: i64) -> Vec<Block> {
        let mut blocks: Vec<Block> = vec![];
        for master_blocks in &self.master_blocks {
            if id == master_blocks.id {
                for block in &master_blocks.block_data {
                    blocks.push(block.clone())
                }
            }
        }
        blocks
    }

    pub fn find_block_by_hash(&self, hash: String) -> Block {
        for master_block in self.master_blocks.clone() {
            for block in master_block.block_data {
                if hash.trim() == block.block_hash.trim() {
                    return block;
                }
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
        for master_block in &self.master_blocks {
            for block in &master_block.block_data {
                if block.transaction.sender == "user".to_string() {
                    balance = balance - block.transaction.amount;
                } else {
                    balance = balance + block.transaction.amount;
                }
            }
        }

        balance
    }
}

impl MasterBlock {
    pub fn new(id: i64, previous_hash: String, block_data: Vec<Block>) -> MasterBlock {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}-{}-{:?}", id, previous_hash, block_data));
        let block_hash_str: String = format!("{:x}", hasher.finalize());
        MasterBlock {
            id: id,
            previous_hash: previous_hash,
            block_hash: block_hash_str,
            block_data: block_data,
        }
    }
}
