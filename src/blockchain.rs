extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use self::sha2::{Sha256, Digest};
use std::fmt::Write;
use std::hash::Hash;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
    difficulty: u32,
    miner_address: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            current_transaction: Vec::new(),
            difficulty,
            miner_address,
            reward: 100.0,
        };
        chain.generate_new_block();
        chain
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };
        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };
        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.current_transaction);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:?}", &block);
        self.chain.push(block);
        true
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.current_transaction.push(Transaction {
            sender,
            receiver,
            amount,
        });
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    fn get_merkle(curr_transaction: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();
        for t in &curr_transaction {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }
        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }
        while merkle.len() > 1 {
            let mut result = Vec::new();
            for i in 0..merkle.len() / 2 {
                let concat = format!("{:?}{:?}", merkle[i * 2], merkle[i * 2 + 1]);
                let hash = Chain::hash_str(&concat);
                result.push(hash);
            }
            merkle = result;
        }
        merkle[0].clone()
    }
}