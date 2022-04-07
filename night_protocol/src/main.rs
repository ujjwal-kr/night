#[macro_use]
extern crate rocket;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
struct Block {
    block_data: String,
    previous_hash: String,
    transaction_list: Vec<String>,
    block_hash: String,
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
    let genesus = Block::new(
        "genesus".to_string(),
        ["trans 1".to_string(), "trans 2".to_string()].to_vec(),
    );

    let mut blocks: Vec<Block> = vec![];
    blocks.push(genesus);

    add_block(["trans 3".to_string(), "trans 4".to_string()].to_vec(), &mut blocks);
    add_block(["trans 5".to_string(), "trans 6".to_string()].to_vec(), &mut blocks);
    add_block(["trans 7".to_string(), "trans 8".to_string()].to_vec(), &mut blocks);

    //  server
    rocket::build().mount("/", routes![index])
}

fn add_block(transactions: Vec<String>, blocks: &mut Vec<Block>) {
    let prev_block = &blocks[blocks.len() -1];
    let new_block = Block::new(prev_block.clone().block_hash, transactions);
    blocks.push(new_block);
}

#[get("/")]
fn index() -> String {
    "Hello, world!".to_string()
}
