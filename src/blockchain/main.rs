use blockchainlib::*;

fn main() {
    let difficulty: u128 = 0x00000fffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                }
            ],
        }],
        difficulty
    );
    println!("➕    Adicionado bloco genesis!");

    genesis_block.mine();

    println!("⛏️    Bloco genesis minerado {:?}", &genesis_block);

    let last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 360,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    }
                ],
            }
        ],
        difficulty
    );
    println!("➕    Adicionado bloco!");

    block.mine();

    println!("⛏️    Bloco minerado {:?}", &block);
}
