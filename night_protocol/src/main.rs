#[macro_use]
extern crate rocket;
use rocket::response::content::Json;
use rocket::State;
use serde_json::json;

mod blockchain;
use blockchain::blockchain::*;
use blockchain::master_chain::*;

#[launch]
fn rocket() -> _ {
    let mut blocks = Blockchain::new();
    let mut master_blocks = Master::new();

    let mut i = 0.0f64;

    // Me and da bois test the chain

    loop {
        i = i + 1.0;
        if blocks.blocks.len() == 20 {
            master_blocks.add_master_block(blocks.blocks);
            blocks.blocks = vec![];
            blocks.genesus();
        }
        blocks.add_block(Transaction {
            sender: "net".to_string(),
            reciever: "user".to_string(),
            amount: i * 5.0,
        });
        if i == 100.0 { 
            break;
        }
    }

    // Example Actions

    // print all the blocks

    for block in &blocks.blocks {
        print!(
            "id:{}\n hash: {},\n previous_hash: {},\n Transaction: {:?}\n\n",
            block.id, block.block_hash, block.previous_hash, block.transaction
        );
    }

    // print the master blockchain

    for master_block in &master_blocks.master_blocks {
        print!(
            "id: {}\n hash: {},\n previous_hash: {}, \n Blocks: {:?}\n\n",
            master_block.id,
            master_block.block_hash,
            master_block.previous_hash,
            master_block.block_data
        )
    }

    let balance: f64 = blocks.calculate_balance() + master_blocks.calculate_balance();
    println!("{}", balance);

    //  server
    rocket::build()
        .manage(blocks)
        .manage(master_blocks)
        .mount("/transactions", routes![index, get_transaction])
        .mount("/balance", routes![get_balance])
}

// Get all blocks pagination mode
#[get("/?<page>")]
fn index(page: i64, blocks: &State<Blockchain>, master_blocks: &State<Master>) -> Json<String> {
    if page == 0 {
        let serialized = serde_json::to_string(&blocks.blocks).unwrap();
        return Json(serialized);
    }
    let serialized = serde_json::to_string(&master_blocks.find_blocks_by_master_id(page)).unwrap();
    Json(serialized)
}

// Get Balance
#[get("/")]
fn get_balance(blocks: &State<Blockchain>, master_blocks: &State<Master>) -> Json<String> {
    let balance: f64 = blocks.calculate_balance() + master_blocks.calculate_balance();
    let data = json!({
        "balance": balance.to_string()
    });

    Json(data.to_string())
}

// Find Transaction
#[get("/<hash>")]
fn get_transaction(
    hash: String,
    master_blocks: &State<Master>,
    blocks: &State<Blockchain>,
) -> Json<String> {
    let block: Block;
    let possible_master_block = master_blocks.find_block_by_hash(hash.clone());
    let possible_block = blocks.find_block_by_hash(hash);

    if possible_block.id != 0 {
        block = possible_block;
        let serialized = serde_json::to_string(&block).unwrap();
        return Json(serialized);
    } else if possible_master_block.id != 0 {
        block = possible_master_block;
        let serialized = serde_json::to_string(&block).unwrap();
        return Json(serialized);
    } else {
        block = possible_block;
    }

    let serialized = serde_json::to_string(&block).unwrap();
    Json(serialized)
}
