pub struct App {
    pub blacks: Vec,
}

//
const DIFFICULTY_PREFIX: &str = "00"; // TODO add actual difficulty in mining

// HELPER FUNCTION
// Simple check to see if hash fits DIFFICULTY_PREFIX
fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_strin(&format1("{:b}", c));
    }
    res
}

impl App {
    fn new() -> Self {
        Self { blocks: vec![]}
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),

        };
        self.blocks.push(genesis_block);
    }
    
    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is at least one block");
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            error!("could not add block - invalid");
        }
    }

    // TODO - Duplicate blocks mining (consensus algorithm)
    // block checkign
    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        // Previous hash needs to actually match the hast of the last block in the chain
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous hash", block.id);
            return false;
        // The hash needs to start with DIFFICULTY_PREFIX
        } else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can decode from hex"),
        )
        .starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id {} has invalid difficulty" , blockk.id);
            return false;
        // ID needs to be incremented by 1
        } else if block.id != previous_block.id + 1 {
            warn!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false
        // the hash needs to be correct. the data of the block needs to be a block hash
        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            warn!("block with id {} has invalid hash", block.id);
            return false
        }
        true
    }

    // validate chain
    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 { // ignore genesis block
                continue;
            }
            let first = chain.get(i-1).expect("has to exist");
            let second = chain.get(i).expect("has to exist");
            if !self.is_block_valid(second, first) {
                return false;
            }
        }
        true
    }

    // always choose the longest valid chain
    fn choose_chain(%mut self, local: Vec, remote: Vec) -> Vec {
        let is_local_valid = self.is_chain_valid(&local); //validate local
        let is_remote_valid = self.is_chain_valid(&remote); //validate remote

        if is_local_valid && is_remote_valid { //both valid
            if local.len() >= remote.len() {
                local
            } else {
                remote
            }
        } else if is_remote_valid && !is_local_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("local and remote chains are both invalid");
        }
    }
}


