use actix_web::*;
// use std::{fmt::format, sync::Mutex};
use std::sync::Mutex;
use serde::Deserialize;

use blockchainlib::*;


struct AppState {
    blockchain: Mutex<Blockchain>,
}
// CARREGA AS INFORMAÇÕES ENVIADAS VIA REQUEST JSON
#[derive(Deserialize)]
struct Info {    
    sender: String,
    receiver: String,
    input_value: u64,
    output_value: u64
    // IMPLEMENTAR UMA PESQUISA APRIMORADA DE SALDO DO SENDER ANTES DA CRIAÇÃO DO BLOCO
        
}
//IMPLEMENTA MÉTODO PARA ADICIONAR E MINERAR UM BLOCO BASEADO NUMA REQUEST JSON
// NESTE MÉTODO, CASO A REQUEST APRESENTE UM REMETENTE, OCORRE UMA TRANSAÇÃO CONVENCIONAL, CASO CONTRARIO, OCORRE UMA TRANSAÇÃO DE BASE MONETÁRIA 
async fn add_defined_block(data: web::Data<AppState>, info: web::Json<Info>) -> HttpResponse {
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
            last_index_u32.clone(),
            now(),
            vec![0; 32],
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
        println!("➕    Adicionado bloco genesis!");
    
        genesis_block.mine();
        println!("⛏️    Bloco genesis minerado {:?}", &genesis_block);
        let response = genesis_block.clone();
    
        blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");
    
        return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("➕    Adicionado bloco genesis! <br> ⛏️    Bloco genesis minerado {:?}", response))

    }

    if info.sender == ""{
        
        let last_hash = blockchain.blocks.last().unwrap().hash.clone();
        let mut new_block = Block::new(
        last_index_u32,
        now(),
        last_hash,
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: info.receiver.clone().to_owned(),
                    value: info.output_value.clone(),
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

        return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("➕    Adicionado novo bloco! <br> ⛏️    Novo bloco minerado {:?}", response))
    }

    let last_hash = blockchain.blocks.last().unwrap().hash.clone();
    let mut new_block = Block::new(
        last_index_u32,
        now(),
        last_hash,
        vec![Transaction {
            inputs: vec![
                transaction::Output {
                    to_addr: info.sender.clone().to_owned(),
                    value: info.input_value.clone(),
                },
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: info.receiver.clone().to_owned(),
                    value: info.output_value.clone(),
                },
                transaction::Output {
                    to_addr: info.sender.clone().to_owned(),
                    value: info.input_value.clone() - info.output_value.clone(),
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
//IMPLEMENTAÇÃO DO MÉTODO PARA APRESENTAR TODOS OS DADOS DA BLOCKCHAIN
async fn info(data: web::Data<AppState>) -> HttpResponse {
    let blockchain = data.blockchain.lock().unwrap();
    let mut data: String = "🔗Blocos on-chain:".to_owned();

    // println!("{:?}",&blockchain.blocks);
    for block in &blockchain.blocks {
        // println!("Bloco coletado{:?}", &block);
        let mut trasactions = "✔️Transações: <br>".to_string();
        
        for t in block.transactions.clone(){
            trasactions = format!("{}   Entradas: {:?} <br> Saidas {:?} <br>", trasactions, t.inputs, t.outputs)
        }

        data = format!("{} <br> BLOCO [{}]: {:?} <br>   🕝Timestamp: {} <br>    ↩️Hash do Bloco Anterior: {} <br>    ⛏️Tentativas: {} <br>    {}____________________________________", data, &block.index, hex::encode(&block.hash) ,&block.timestamp, hex::encode(&block.prev_block_hash),  &block.nonce, trasactions)
    }
    blockchain.get_blocks_json();
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
        .route("/add", web::post().to(add_defined_block))
        

    });

    let porta = 9091;
    let api = api.bind(format!("127.0.0.1:{}", porta)).expect("⚠️ Erro de conexão...");

    println!("Conexão estabelecida! \n http://localhost:{}", porta);

    api.run().await
}
