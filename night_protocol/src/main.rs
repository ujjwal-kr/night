#[macro_use]
extern crate rocket;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Main {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
struct Block {
    block_data: String,
    previous_hash: String,
    transaction_list: Vec<String>,
    block_hash: String,
}

impl Main {
    fn new() -> Self {
        Self {
            blocks: vec![Block::new(
                "genesus".to_string(),
                ["tran1".to_string()].to_vec(),
            )],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<String>) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new(prev_block.clone().block_hash, transactions);
        self.blocks.push(new_block);
    }
}

impl Block {
    fn new(previous_hash: String, transaction_list: Vec<String>) -> Block {
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
    blocks.add_block(["tran2".to_string(), "tran3".to_string()].to_vec());
    blocks.add_block(["tran4".to_string(), "tran5".to_string()].to_vec());
    blocks.add_block(["tran6".to_string(), "tran7".to_string()].to_vec());
    
    for block in blocks.blocks {
        println!("{:?} && {:?}", block.block_hash, block.previous_hash);
    }

    //  server
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> String {
    "Hello, world!".to_string()
}
