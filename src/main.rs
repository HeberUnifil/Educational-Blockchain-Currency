use actix_web::*;
// use std::{fmt::format, sync::Mutex};
use std::sync::Mutex;

// mod routes;
// use routes::ping::*;
// use routes::info::*;
// use routes::catalogo::*;

use blockchainlib::*;


struct AppState {
    blockchain: Mutex<Blockchain>,
}

async fn add_block(data: web::Data<AppState>) -> HttpResponse {
    let mut blockchain = data.blockchain.lock().unwrap();
    let difficulty: u128 = 0x00000fffffffffffffffffffffffffff;
    let last_index = blockchain.blocks.len();
    let last_index_u32: u32 = if last_index <= u32::MAX as usize {
        last_index as u32 // Safe conversion
    } else {
        panic!("Vector length exceeds u32 maximum value.");
    };
    

    if last_index_u32 == 0 {

        let mut genesis_block = Block::new(
            0,
            now(),
            vec![0; 32],
            vec![Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Owner".to_owned(),
                        value: 50,
                    },
                ],
            }],
            difficulty
        );
        println!("➕    Adicionado bloco genesis!");
    
        genesis_block.mine();
        println!("⛏️    Bloco genesis minerado {:?}", &genesis_block);
        let response = genesis_block.clone();
    
        blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");
    
        return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("➕    Adicionado bloco genesis! <br> ⛏️    Bloco genesis minerado {:?}", response))

    }

    let last_hash = blockchain.blocks.last().unwrap().hash.clone();
    let mut new_block = Block::new(
        last_index_u32,
        now(),
        last_hash,
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Owner".to_owned(),
                    value: 150,
                },
            ],
        }],
        difficulty
    );
    println!("➕    Adicionado bloco!");

    new_block.mine();
    println!("⛏️    Bloco minerado {:?}", &new_block);
    let response = new_block.clone();

    blockchain.update_with_block(new_block).expect("Failed to add block");

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(format!("➕    Adicionado novo bloco! <br> ⛏️    Novo bloco minerado {:?}", response))
}

async fn info(data: web::Data<AppState>) -> HttpResponse {
    let blockchain = data.blockchain.lock().unwrap();
    let mut data: String = "🔗Blocos on-chain:".to_owned();

    // println!("{:?}",&blockchain.blocks);
    for block in &blockchain.blocks {
        println!("Bloco coletado{:?}", &block);
        data = format!("{} <br> {:?}", data, &block)
    }

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(format!("{}", data))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = web::Data::new(AppState {
        blockchain: Mutex::new(Blockchain::new())
    });

    let api = HttpServer::new( move || {
        App::new()
        .app_data(blockchain.clone())
        .route("/", web::get().to(info))
        .route("/add", web::get().to(add_block))

    });

    let porta = 9091;
    let api = api.bind(format!("127.0.0.1:{}", porta)).expect("⚠️ Erro de conexão...");

    println!("Conexão estabelecida! \n http://localhost:{}", porta);

    api.run().await
}
