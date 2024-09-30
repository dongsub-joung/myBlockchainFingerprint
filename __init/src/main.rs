use chrono::prelude::*;
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

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


fn main() -> std::io::Result<()> {
    // Set Msg
    let data: Box<String>= Box::new("Philip Kim who Kangwon National University major Sociology met 정원덕 with JoungDongSub at 코란도 스포츠 ".to_string());
    // Set pre hashing for verify

    let previous_hash= "[220, 27, 59, 217, 173, 174, 218, 94, 24, 8, 149, 193, 131, 23, 254, 143, 12, 254, 54, 159, 247, 72, 57, 238, 202, 29, 47, 241, 21, 109, 128, 177, 107, 142, 165, 151, 18, 38, 173, 72, 15, 43, 253, 211, 219, 159, 48, 226, 217, 215, 24, 51, 106, 72, 151, 224, 208, 192, 184, 60, 101, 140, 70, 255]".to_string();

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
