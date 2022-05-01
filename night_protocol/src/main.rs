#[macro_use]
extern crate rocket;
use rand::Rng;
use rocket::response::content::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

mod blockchain;
use blockchain::blockchain::*;
use blockchain::master_chain::*;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Serialize, Deserialize)]
struct SharedBlockchain {
    blocks: Mutex<Blockchain>,
}

#[derive(Serialize, Deserialize)]
struct SharedMaster {
    master_blocks: Mutex<Master>,
}

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
            amount: 500.0,
        });
        if i == 2.0 {
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
        .attach(CORS)
        .manage(SharedBlockchain {
            blocks: Mutex::new(blocks),
        })
        .manage(SharedMaster {
            master_blocks: Mutex::new(master_blocks),
        })
        .mount("/transactions", routes![index, get_transaction])
        .mount("/balance", routes![get_balance])
        .mount("/gamble", routes![gamble])
        .mount("/countmaster", routes![count_master])
}

// Count master blocks
#[get("/")]
fn count_master(shared_master_blocks: &State<SharedMaster>) -> Json<String> {
    let master_blocks: &SharedMaster = shared_master_blocks.inner();
    let data = json!({
        "master_count" : master_blocks.master_blocks.lock().unwrap().master_blocks.len()
    });
    Json(data.to_string())
}

// Get all blocks pagination mode
#[get("/?<page>")]
fn index(
    page: i64,
    shared_blocks: &State<SharedBlockchain>,
    shared_master_blocks: &State<SharedMaster>,
) -> Json<String> {
    let blocks: &SharedBlockchain = shared_blocks.inner();
    let master_blocks: &SharedMaster = shared_master_blocks.inner();

    if page == 1 {
        let serialized = serde_json::to_string(&blocks.blocks.lock().unwrap().blocks).unwrap();
        return Json(serialized);
    }
    let serialized = serde_json::to_string(
        &master_blocks
            .master_blocks
            .lock()
            .unwrap()
            .find_blocks_by_master_id(page - 1),
    )
    .unwrap();
    Json(serialized)
}

// Get Balance
#[get("/")]
fn get_balance(
    shared_blocks: &State<SharedBlockchain>,
    shared_master_blocks: &State<SharedMaster>,
) -> Json<String> {
    let blocks: &SharedBlockchain = shared_blocks.inner();
    let master_blocks: &SharedMaster = shared_master_blocks.inner();

    let balance: f64 = blocks.blocks.lock().unwrap().calculate_balance()
        + master_blocks
            .master_blocks
            .lock()
            .unwrap()
            .calculate_balance();
    let data = json!({
        "balance": balance.to_string()
    });

    Json(data.to_string())
}

// Find Transaction
#[get("/<hash>")]
fn get_transaction(
    hash: String,
    shared_master_blocks: &State<SharedMaster>,
    shared_blocks: &State<SharedBlockchain>,
) -> Json<String> {
    let blocks: &SharedBlockchain = shared_blocks.inner();
    let master_blocks: &SharedMaster = shared_master_blocks.inner();

    let block: Block;
    let possible_master_block = master_blocks
        .master_blocks
        .lock()
        .unwrap()
        .find_block_by_hash(hash.clone());
    let possible_block = blocks.blocks.lock().unwrap().find_block_by_hash(hash);

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

// Gamble func

#[post("/?<amount>")]
fn gamble(
    amount: f64,
    shared_blocks: &State<SharedBlockchain>,
    shared_master_blocks: &State<SharedMaster>,
) -> Json<String> {
    let blocks: &SharedBlockchain = shared_blocks.inner();
    let master_blocks: &SharedMaster = shared_master_blocks.inner();

    let balance: f64 = blocks.blocks.lock().unwrap().calculate_balance()
        + master_blocks
            .master_blocks
            .lock()
            .unwrap()
            .calculate_balance();
    if amount > balance {
        let data = json!({
            "error": "Balance Error"
        });
        Json(data.to_string())
    } else if amount < 0.0001 {
        let data = json!({
            "error": "Invalid Amount"
        });
        Json(data.to_string())
    } else {
        let win: bool;
        let mut rng = rand::thread_rng();
        let num: i32 = rng.gen_range(0..2);
        println!("{}", num);
        if num == 1 {
            win = true;
        } else {
            win = false;
        }

        if blocks.blocks.lock().unwrap().blocks.len() == 20 {
            master_blocks
                .master_blocks
                .lock()
                .unwrap()
                .add_master_block(blocks.blocks.lock().unwrap().blocks.clone());
            blocks.blocks.lock().unwrap().blocks = vec![];
            blocks.blocks.lock().unwrap().genesus();
        }

        if win == true {
            blocks.blocks.lock().unwrap().add_block(Transaction {
                reciever: "user".to_string(),
                sender: "net".to_string(),
                amount: amount * 1.5,
            });
            let data = json!({
                "win": "true",
                "amount": amount * 1.5,
                "newBalance": amount * 1.5 + balance
            });
            Json(data.to_string())
        } else {
            blocks.blocks.lock().unwrap().add_block(Transaction {
                reciever: "net".to_string(),
                sender: "user".to_string(),
                amount: amount,
            });
            let data = json!({
                "win": "false",
                "amount": amount,
                "newBalance": balance - amount
            });
            Json(data.to_string())
        }
    }
}
