//level 2

// this blockchain is more advanced. Here we add hash function 
use sha2::{Sha256, Digest};

// real hash function SHA256
fn real_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result) // so here we change result to the reading string
}

struct Block {
    id: u32,
    data: String,
    previous_hash: String, // now it's for hash
    hash: String,
}

impl Block {
    fn new(id: u32, data: String, previous_hash: String) -> Self {
        // here we creating hash for all data of block
        let hash_input = format!("{}{}{}", id, data, previous_hash);
        let hash = real_hash(&hash_input);

        Block {
            id,
            data,
            previous_hash,
            hash,
        }
    }
    
    fn display(&self) {
        println!("Block {}:", self.id);
        println!("  Data: {}", self.data);
        println!("  Previous Hash: {}", self.previous_hash);
        println!("  Hash: {}", self.hash);
        println!();
    }
}

fn main() {
    println!("Now we already use SHA256");
    println!("==========================");
    
    let block1 = Block::new(1, "first block".to_string(), "0".to_string());
    let block2 = Block::new(2, "second block".to_string(), block1.hash.clone());
    let block3 = Block::new(3, "3rd block".to_string(), block2.hash.clone());
    
    // Display each block
    block1.display();
    block2.display();
    block3.display();
    
    // Check our chain
    let blocks = vec![block1, block2, block3];
    check_chain(&blocks);
}

fn check_chain(blocks: &[Block]) {
    println!("Checking the chain...");
    
    let mut is_valid = true;
    
    for i in 0..blocks.len() {
        if i == 0 {
            // Check genesis block (first block)
            if blocks[i].previous_hash != "0" {
                println!("Genesis block should have previous_hash = '0'");
                is_valid = false;
            }
        } else {
            // Check if current block's previous_hash matches the previous block's hash
            if blocks[i].previous_hash != blocks[i-1].hash {
                println!(" Block {} has incorrect previous_hash!", blocks[i].id);
                is_valid = false;
            }
        }
        
        // Verify the block's own hash is correct
        let hash_input = format!("{}{}{}", blocks[i].id, blocks[i].data, blocks[i].previous_hash);
        let computed_hash = real_hash(&hash_input);
        
        if blocks[i].hash != computed_hash {
            println!(" Block {} has incorrect hash!", blocks[i].id);
            is_valid = false;
        }
    }
    
    if is_valid {
        println!(" The chain is valid!");
    } else {
        println!(" The chain is invalid!");
    }
}