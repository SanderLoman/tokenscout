use std::{collections::HashMap, sync::Arc};

pub trait Chain {
    fn connect(&self) -> Result<Arc<dyn ChainReader>, Box<dyn std::error::Error>>;
}

pub trait ChainReader {
    fn get_block(&self, block_number: u64) -> Result<Block, Box<dyn std::error::Error>>;
}

#[derive(Clone)]
pub struct EthNode {
    pub url: String,
}

impl Chain for EthNode {
    fn connect(&self) -> Result<Arc<dyn ChainReader>, Box<dyn std::error::Error>> {
        // Replace with your Ethereum node connection logic
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Block {
    pub number: u64,
    // Add other relevant block data here
}

impl ChainReader for Arc<EthNode> {
    fn get_block(&self, block_number: u64) -> Result<Block, Box<dyn std::error::Error>> {
        // Replace with your logic to get block using self.url and block_number
        unimplemented!()
    }
}

pub struct NetworkManager {
    pub chains: HashMap<String, Arc<dyn Chain>>,
}

impl NetworkManager {
    pub fn new() -> Self {
        NetworkManager {
            chains: HashMap::new(),
        }
    }

    pub fn add_chain(&mut self, name: String, chain: Arc<dyn Chain>) {
        self.chains.insert(name, chain);
    }

    pub fn get_chain(&self, name: &str) -> Option<Arc<dyn Chain>> {
        self.chains.get(name).cloned()
    }

    pub fn connect_all(&self) -> Result<Vec<Arc<dyn ChainReader>>, Box<dyn std::error::Error>> {
        let mut connected_readers = Vec::new();
        for (_name, chain) in self.chains.iter() {
            let connected_reader = chain.connect()?;
            connected_readers.push(connected_reader);
        }
        Ok(connected_readers)
    }
}
