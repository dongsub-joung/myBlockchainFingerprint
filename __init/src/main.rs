use chrono::prelude::*;
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

#[derive(Debug)]
struct Block {
    timestamp: DateTime<Utc>,
    data: Box<String>, // JoungDongSub fixed
    previous_hash: String,
    hash: String,
    nonce: u64,
}
impl Block {
    fn new(data: Box<String>, previous_hash: String) -> Block {
        let timestamp :DateTime<Utc> = Utc::now(); 
        let nonce = 0; // Initial nonce
        
        let mut hasher= Sha512::new();

        let digest= format!("{}{}{}{}", previous_hash.clone(), timestamp, data.clone(), nonce.clone());
        hasher.update(digest);
        let hash = hasher.finalize();
        let hash= format!("{:?}", hash);     

        Block {timestamp, data, previous_hash, hash, nonce }
    }
}

fn converte_to_Json(block: Block)){

}

fn write_txt_in_dis(json: Json){

}
 

fn main(){
    let data: Box<String>= Box::new("testing".to_string());
    let previous_hash= "cb2964f8cc21bb61bfcfa0a98aa8dada".to_string();
    let block= Block::new(data, previous_hash);

    let time= format!("./{}-post.txt", block.timestamp);
    
}

// # ref
// # https://users.rust-lang.org/t/saving-a-complex-struct-to-disk/97664/3
