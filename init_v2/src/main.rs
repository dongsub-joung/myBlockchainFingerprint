use chrono::prelude::*;
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

use std::env;
use std::vec::Splice;

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    timestamp: String,
    data: Box<String>, // JoungDongSub fixed
    previous_hash: String,
    hash: String,
    nonce: u64,
}
impl Block {
    fn new(data: Box<String>, previous_hash: String) -> Block {
        let timestamp= Self::get_current_timestamp();

        let nonce = 0; // Initial nonce
        
        let mut hasher= Sha512::new();

        let digest= format!("{}{}{}{}", previous_hash.clone(), timestamp, data.clone(), nonce.clone());
        hasher.update(digest);
        let hash = hasher.finalize();
        let hash= format!("{:?}", hash);     

        Block {timestamp, data, previous_hash, hash, nonce }
    }

    fn get_current_timestamp() -> String{
        let time:DateTime<Utc>= Utc::now();

        return format!("{}", time);
    }
}

fn converte_to_Json(block: Block) -> String{
    let serialized_user = serde_json::to_string(&block).unwrap();
    
    return serialized_user;
}

fn get_previous_hash() -> String{
    // Set your file name
    let FILE_NAME= "something".to_string();
    
    let PATH= "/home/tatuya/git/myBlockchainFingerprint/init_v2/".to_string();

    let parts = "some string 123 content".split("previous_hash");

    let mut i= 0;
    let mut moto_hash_string: &str= "";
    for part in parts{
        if i==1 {
            moto_hash_string= part;
        }
        i+=1;
    }

    moto_hash_string.to_string()
}

fn main() -> std::io::Result<()> {
    // Set Msg
    let text_body= "Philip Kim who Kangwon National University major Sociology met 정원덕 with JoungDongSub at 코란도 스포츠".to_string();
    let data: Box<String>= Box::new(text_body);
    
    // Set pre hashing for verify
    let previous_hash= get_previous_hash();
    let block= Block::new(data, previous_hash);

    let time= format!("./{}-post.txt", block.timestamp);

    let result= converte_to_Json(block);
    
    let mut file = File::create(time)?;
    file.write_all(result.as_bytes())?;

    Ok(())
}

// # ref
// https://users.rust-lang.org/t/saving-a-complex-struct-to-disk/97664/3
// https://turreta.com/blog/2019/09/22/rust-convert-struct-instances-to-and-from-json/
