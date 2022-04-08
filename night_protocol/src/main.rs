#[macro_use]
extern crate rocket;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Blockchain {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    id: i64,
    previous_hash: String,
    transaction: Transaction,
    block_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    sender: String,
    reciever: String,
    amount: f64,
}

impl Blockchain {
    fn new() -> Self {
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

    pub fn add_block(&mut self, transactions: Transaction) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new(
            prev_block.id + 1,
            prev_block.clone().block_hash,
            transactions,
        );
        self.blocks.push(new_block);
    }

    fn validate_block(&self, block: &Block, previous_block: &Block) -> bool {
        if previous_block.block_hash.trim() == block.previous_hash.trim() {
            true
        } else {
            false
        }
    }

    fn validate_chain(&self) {
        for i in 1..self.blocks.len() {
            let block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1].clone();
            if self.validate_block(block, prev_block) == false {
                println!("block bad");
                return;
            }
        }
        println!("blocks good");
    }

    fn find_block_by_hash(&self, hash: String) -> Block {
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
    fn new(id: i64, previous_hash: String, transaction: Transaction) -> Block {
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

fn main() {
    let mut blocks = Blockchain::new();
    let mut i = 0.0f64;

    // Add some blocks for testing

    loop {
        i = i + 1.0;
        blocks.add_block(Transaction {
            sender: "test".to_string(),
            reciever: "net".to_string(),
            amount: i * 5.0,
        });
        if i == 10.0 {
            break;
        }
    }

    // Example Actions

    for block in &blocks.blocks {
        println!("{:?} \n", block);
    }

    // print all the blocks

    blocks.validate_chain();

    // Finding a block by known hash

    let my_block = blocks.find_block_by_hash(
        "55f2610a693e8babf9f828a6f68af6e8567e601bf32153550dc811d757ff8c91".to_string(),
    );

    println!("{:?}", my_block);

    //  server
    // rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> String {
    "Hello, world!".to_string()
}
