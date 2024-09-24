pub struct App{
    pub blocks: Vec,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: Box<String>, // JoungDongSub fixed
    pub nonce: u64,
}







// impl Block {
//     fn new(index: u32, data: String, previous_hash: String) -> Block {
//         let timestamp = get_current_timestamp();
//         let nonce = -1; // Initial nonce(JoungDongSub fixed)
//         let hash = calculate_hash(index, &previous_hash, timestamp, &data, nonce);
//         Block { index, timestamp, data, previous_hash, hash, nonce }
//     }
// }