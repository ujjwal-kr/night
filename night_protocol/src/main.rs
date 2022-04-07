#[macro_use]
extern crate rocket;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Main {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    block_data: String,
    previous_hash: String,
    transaction_list: Vec<Transaction>,
    block_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transaction {
    sender: String,
    reciever: String,
    amount: f64,
}

impl Main {
    fn new() -> Self {
        Self {
            blocks: vec![Block::new(
                "genesus".to_string(),
                [Transaction {
                    sender: "test".to_string(),
                    reciever: "net".to_string(),
                    amount: 500.0,
                }]
                .to_vec(),
            )],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new(prev_block.clone().block_hash, transactions);
        self.blocks.push(new_block);
    }
}

impl Block {
    fn new(previous_hash: String, transaction_list: Vec<Transaction>) -> Block {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}--{:?}", previous_hash, transaction_list));
        let block_hash_str: String = format!("{:x}", hasher.finalize());
        Block {
            block_data: format!("{}+{:?}", previous_hash, transaction_list),
            block_hash: block_hash_str,
            previous_hash: previous_hash,
            transaction_list: transaction_list,
        }
    }
}

#[launch]
fn rocket() -> _ {
    let mut blocks = Main::new();

    blocks.add_block(
        [Transaction {
            sender: "test".to_string(),
            reciever: "net".to_string(),
            amount: 505.0,
        }]
        .to_vec(),
    );

    blocks.add_block(
        [Transaction {
            sender: "testtwo".to_string(),
            reciever: "net".to_string(),
            amount: 500.0,
        }]
        .to_vec(),
    );

    blocks.add_block(
        [Transaction {
            sender: "testthree".to_string(),
            reciever: "net".to_string(),
            amount: 500.0,
        }]
        .to_vec(),
    );

    for block in blocks.blocks {
        println!("{:?} \n", block);
    }

    //  server
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> String {
    "Hello, world!".to_string()
}
