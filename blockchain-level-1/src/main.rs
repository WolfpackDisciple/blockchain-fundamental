//level 1

// well, this exaple of a very simple blockchain
// is used for undersatand a more  complex blockchain network



struct Block {
    id: u32 ,
    data: String,
    previous_id : u32 // this ID is previous of block
}


fn main() {
    println!("liking blocks into a chain");

    let block1 = Block {
        id: 1,
        data: "the genesis block".to_string(),
        previous_id: 0 , // the genesis block doesn`t have previous block
    };

    let block2 = Block {
        id: 2,
        data: "the second block".to_string(),
        previous_id: 1 ,
    };


    let block3 = Block {
        id: 3,
        data: "the 3th block".to_string(),
        previous_id: 2 , 
    };

    //and here we checking our chain
    check_chain(&[block1, block2, block3]);

}


fn check_chain(blocks: &[Block]) {
    println!(" Checking the chain");

    for block in blocks {
        if block.previous_id == 0 {
            println!("Block {} is genesis", block.id);
        } else {
            println!("Block {}  -> Block {}", block.previous_id, block.id);
        }
    }
    println!("Done! The chain has been created!")
}