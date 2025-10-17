use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

// Time function
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// SHA256 hash function
fn real_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result) 
}

#[derive(Debug, Clone)]
struct Block {
    id: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(id: u32, data: String, previous_hash: String) -> Self {
        let timestamp = now();
        
        let hash_input = format!("{}{}{}{}", id, timestamp, data, previous_hash);
        let hash = real_hash(&hash_input);
        
        Block {
            id,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    
    fn new_with_timestamp(id: u32, data: String, previous_hash: String, timestamp: u64) -> Self {
        let hash_input = format!("{}{}{}{}", id, timestamp, data, previous_hash);
        let hash = real_hash(&hash_input);
        
        Block {
            id,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
    
    fn is_valid(&self) -> bool {
        let hash_input = format!("{}{}{}{}", self.id, self.timestamp, self.data, self.previous_hash);
        let calculated_hash = real_hash(&hash_input);
        calculated_hash == self.hash
    }
    
    fn display(&self) {
        println!("--- Block {} ---", self.id);
        println!("  Time: {}", self.timestamp);
        println!("  Data: {}", self.data);
        
        if self.previous_hash.len() >= 16 {
            println!("  Previous Hash: {}...", &self.previous_hash[0..16]);
        } else {
            println!("  Previous Hash: {}", self.previous_hash);
        }
        
        if self.hash.len() >= 16 {
            println!("  Hash: {}...", &self.hash[0..16]);
        } else {
            println!("  Hash: {}", self.hash);
        }
        
        println!("  Is Valid?: {}", self.is_valid());
        println!();
    }
}

fn check_chain(blocks: &[Block]) -> bool {
    println!("Checking blockchain integrity...");
    
    let mut is_valid = true;
    
    for i in 0..blocks.len() {
        if i == 0 {
            if blocks[i].previous_hash != "0" {
                println!("ERROR: Genesis block should have previous_hash = '0'");
                is_valid = false;
            }
        } else {
            if blocks[i].previous_hash != blocks[i-1].hash {
                println!("ERROR: Block {} has incorrect previous_hash!", blocks[i].id);
                is_valid = false;
            }
            
            if blocks[i].timestamp <= blocks[i-1].timestamp {
                println!("ERROR: Block {} has invalid timestamp! Should be after previous block", blocks[i].id);
                is_valid = false;
            }
        }
        
        let hash_input = format!("{}{}{}{}", blocks[i].id, blocks[i].timestamp, blocks[i].data, blocks[i].previous_hash);
        let computed_hash = real_hash(&hash_input);
        
        if blocks[i].hash != computed_hash {
            println!("ERROR: Block {} has incorrect hash!", blocks[i].id);
            is_valid = false;
        }
        
        if is_valid {
            println!("Block {} validation: PASSED", blocks[i].id);
        }
    }
    
    if is_valid {
        println!("BLOCKCHAIN STATUS: VALID");
    } else {
        println!("BLOCKCHAIN STATUS: INVALID");
    }
    
    is_valid
}

fn main() {
    println!("BLOCKCHAIN SYSTEM - LEVEL 3+");
    println!("============================\n");
    
    let block1 = Block::new(1, "Genesis: Alice send to Bob 50 coins".to_string(), "0".to_string());
    let block2 = Block::new(2, "Bob send Charlie 30 coins".to_string(), block1.hash.clone());
    let block3 = Block::new(3, "Charlie send Alice 10 coins".to_string(), block2.hash.clone());
    
    let blockchain = vec![block1, block2, block3];
    
    println!("BLOCKCHAIN CONTENTS:");
    for block in &blockchain {
        block.display();
    }
    
    check_chain(&blockchain);
    
    println!("FEATURES IMPLEMENTED:");
    println!("- Chain of blocks");
    println!("- SHA256 hashing"); 
    println!("- Cryptographic links between blocks");
    println!("- Block validation");
    println!("- Timestamp verification");
    println!("- Integrity checking");
    println!("- Tamper detection");
    
    // run unit-tests
    println!("\nRUNNING UNIT TESTS...");
    run_unit_tests();
}

// special unit-tests
fn run_unit_tests() {
    test_valid_block_creation();
    test_invalid_hash_detection();
    test_chain_validation();
    test_tamper_detection();
    test_timestamp_validation();
    println!("\nALL UNIT TESTS PASSED! ✅");
}

fn test_valid_block_creation() {
    println!("TEST: Valid block creation");
    let block = Block::new(1, "Test data".to_string(), "0".to_string());
    assert!(block.is_valid(), "Newly created block should be valid");
    assert_eq!(block.id, 1, "Block ID should be correct");
    assert_eq!(block.previous_hash, "0", "Genesis block should have previous_hash '0'");
    println!("  ✅ Valid block creation test passed");
}

fn test_invalid_hash_detection() {
    println!("TEST: Invalid hash detection");
    let mut block = Block::new(1, "Original data".to_string(), "0".to_string());
    
    // create block with wrong hash
    block.hash = "invalid_hash".to_string();
    
    assert!(!block.is_valid(), "Block with incorrect hash should be invalid");
    println!("  ✅ Invalid hash detection test passed");
}

fn test_chain_validation() {
    println!("TEST: Chain validation");
    
    
    let block1 = Block::new_with_timestamp(1, "Genesis".to_string(), "0".to_string(), 1000);
    let block2 = Block::new_with_timestamp(2, "Block 2".to_string(), block1.hash.clone(), 1001);
    let block3 = Block::new_with_timestamp(3, "Block 3".to_string(), block2.hash.clone(), 1002);
    
    let valid_chain = vec![block1, block2, block3];
    assert!(check_chain(&valid_chain), "Valid chain should pass validation");
    println!("  ✅ Chain validation test passed");
}

fn test_tamper_detection() {
    println!("TEST: Tamper detection");
    
    let mut block1 = Block::new_with_timestamp(1, "Original".to_string(), "0".to_string(), 1000);
    let block2 = Block::new_with_timestamp(2, "Second".to_string(), block1.hash.clone(), 1001);
    
    // change data without hash calculate
    block1.data = "Tampered!".to_string();
    
    let tampered_chain = vec![block1, block2];
    assert!(!check_chain(&tampered_chain), "Tampered chain should fail validation");
    println!("  ✅ Tamper detection test passed");
}

fn test_timestamp_validation() {
    println!("TEST: Timestamp validation");
    
    let block1 = Block::new_with_timestamp(1, "First".to_string(), "0".to_string(), 1000);
    
    // create block with wrong timespamp
    let block2 = Block::new_with_timestamp(2, "Second".to_string(), block1.hash.clone(), 999); // time before previos block
    
    let invalid_time_chain = vec![block1, block2];
    assert!(!check_chain(&invalid_time_chain), "Chain with invalid timestamps should fail");
    println!("  ✅ Timestamp validation test passed");
}

// Unit-tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation_and_validation() {
        let block = Block::new(1, "Test".to_string(), "0".to_string());
        assert!(block.is_valid());
        assert_eq!(block.previous_hash, "0");
    }

    #[test]
    fn test_chain_integrity() {
       
        let block1 = Block::new_with_timestamp(1, "Genesis".to_string(), "0".to_string(), 1000);
        let block2 = Block::new_with_timestamp(2, "Second".to_string(), block1.hash.clone(), 1001);
        
        let chain = vec![block1, block2];
        assert!(check_chain(&chain));
    }

    #[test]
    fn test_tampered_block_detection() {
        let mut block = Block::new(1, "Original".to_string(), "0".to_string());
        block.data = "Tampered".to_string(); 
        
        assert!(!block.is_valid());
    }

    #[test]
    fn test_wrong_previous_hash() {
        let block1 = Block::new_with_timestamp(1, "First".to_string(), "0".to_string(), 1000);
        let block2 = Block::new_with_timestamp(2, "Second".to_string(), "wrong_hash".to_string(), 1001); // wrong previous_hash
        
        let chain = vec![block1, block2];
        assert!(!check_chain(&chain));
    }

    #[test]
    fn test_timestamp_order() {
        let block1 = Block::new_with_timestamp(1, "First".to_string(), "0".to_string(), 1000);
        let block2 = Block::new_with_timestamp(2, "Second".to_string(), block1.hash.clone(), 999); // time previos block
        
        let chain = vec![block1, block2];
        assert!(!check_chain(&chain));
    }

    #[test]
    fn test_new_with_timestamp() {
        
        let block = Block::new_with_timestamp(1, "Test".to_string(), "0".to_string(), 12345);
        assert!(block.is_valid());
        assert_eq!(block.timestamp, 12345);
    }
}