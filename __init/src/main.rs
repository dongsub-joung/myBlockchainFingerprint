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
    let data: Box<String>= Box::new("환율조작국-한국 원화가치 + 국정원 수사력 축소 = 탈조선".to_string());
    // Set pre hashing for verify
    let previous_hash= "[245, 82, 57, 249, 246, 8, 119, 120, 87, 154, 95, 111, 164, 191, 55, 103, 204, 45, 93, 161, 87, 166, 104, 181, 188, 19, 64, 7, 142, 215, 59, 116, 42, 1, 242, 180, 137, 98, 255, 55, 43, 73, 135, 250, 61, 47, 223, 8, 200, 111, 62, 105, 90, 13, 18, 157, 174, 104, 127, 63, 182, 123, 198, 170]".to_string();


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
