#[macro_use]
extern crate rocket;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Block {
    block_data: String,
    previous_hash: String,
    transaction_list: Vec<String>,
    block_hash: String,
}

impl Block {
    fn new(previous_hash: String, transaction_list: Vec<String>) -> Block {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}+{:?}", previous_hash, transaction_list));
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

    let block1 = Block::new(
        genesus.block_hash.clone(),
        ["trans 3".to_string(), "trans 4".to_string()].to_vec(),
    );

    let block2 = Block::new(block1.block_hash.clone(), ["trans 5".to_string()].to_vec());

    println!("{:?} \n {:?} \n {:?}", genesus, block1, block2);

    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> String {
    "Hello, world!".to_string()
}
