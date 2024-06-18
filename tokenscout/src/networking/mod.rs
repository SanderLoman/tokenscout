pub mod rpc_list;

use async_trait::async_trait;
use ethers::{
    providers::{Http, Middleware, Provider, ProviderError, Ws},
    types::{Block, H256, U64},
};
use rpc_list::RpcList;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;

// Chain trait for connecting to a blockchain
#[async_trait]
pub trait Chain {
    async fn connect(
        &self,
    ) -> Result<Arc<dyn ChainReader + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>;
}

// ChainReader trait for reading blockchain data
#[async_trait]
pub trait ChainReader {
    async fn get_block(
        &self,
        block_number: u64,
    ) -> Result<Block<H256>, Box<dyn std::error::Error + Send + Sync>>;
}

// Enum to handle different provider types
pub enum ProviderType {
    Http(Provider<Http>),
    Ws(Provider<Ws>),
}

impl ProviderType {
    pub async fn get_block_number(&self) -> Result<U64, ProviderError> {
        match self {
            ProviderType::Http(provider) => provider.get_block_number().await,
            ProviderType::Ws(provider) => provider.get_block_number().await,
        }
    }

    pub async fn get_block(&self, block_number: u64) -> Result<Option<Block<H256>>, ProviderError> {
        match self {
            ProviderType::Http(provider) => provider.get_block(block_number).await,
            ProviderType::Ws(provider) => provider.get_block(block_number).await,
        }
    }
}

// Structure representing an Ethereum node with multiple endpoints
#[derive(Clone)]
pub struct EthNode {
    pub urls: Vec<String>,
    pub active_url: Arc<Mutex<String>>,
}

impl EthNode {
    pub fn new(urls: Vec<String>) -> Self {
        EthNode {
            active_url: Arc::new(Mutex::new(urls[0].clone())),
            urls,
        }
    }

    pub async fn connect_provider(
        url: &str,
    ) -> Result<ProviderType, Box<dyn std::error::Error + Send + Sync>> {
        if url.starts_with("wss://") {
            Ok(ProviderType::Ws(Provider::<Ws>::connect(url).await?))
        } else if url.starts_with("http://") || url.starts_with("https://") {
            Ok(ProviderType::Http(Provider::<Http>::try_from(
                url.to_string(),
            )?))
        } else {
            Err("Unsupported URL scheme".into())
        }
    }
}

#[async_trait]
impl Chain for EthNode {
    async fn connect(
        &self,
    ) -> Result<Arc<dyn ChainReader + Send + Sync>, Box<dyn std::error::Error + Send + Sync>> {
        for url in &self.urls {
            match EthNode::connect_provider(url).await {
                Ok(provider) => {
                    let reader = Arc::new(EthReader {
                        provider: RwLock::new(provider),
                        urls: self.urls.clone(),
                        active_url: self.active_url.clone(),
                    });
                    // Start monitoring the connection in a background task
                    tokio::spawn(monitor_connection(reader.clone()));
                    return Ok(reader);
                }
                Err(e) => println!("Failed to connect to {}: {}", url, e),
            }
        }
        Err("All endpoints failed".into())
    }
}

// Structure for reading data from Ethereum
pub struct EthReader {
    pub provider: RwLock<ProviderType>,
    pub urls: Vec<String>,
    pub active_url: Arc<Mutex<String>>,
}

#[async_trait]
impl ChainReader for EthReader {
    async fn get_block(
        &self,
        block_number: u64,
    ) -> Result<Block<H256>, Box<dyn std::error::Error + Send + Sync>> {
        let provider = self.provider.read().await;
        let block = provider
            .get_block(block_number)
            .await?
            .ok_or_else(|| "Block not found")?;
        Ok(block)
    }
}

// Function to monitor connection and switch URLs if the current one fails
async fn monitor_connection(reader: Arc<EthReader>) {
    loop {
        sleep(Duration::from_secs(30)).await; // Check every 30 seconds
        if let Err(e) = reader.provider.read().await.get_block_number().await {
            println!("Connection check failed: {}", e);
            // Try to reconnect using other URLs
            for url in &reader.urls {
                if *reader.active_url.lock().await != *url {
                    match EthNode::connect_provider(url).await {
                        Ok(provider) => {
                            *reader.active_url.lock().await = url.clone();
                            let mut writer = reader.provider.write().await;
                            *writer = provider;
                            println!("Switched to backup URL: {}", url);
                            break;
                        }
                        Err(e) => println!("Failed to connect to {}: {}", url, e),
                    }
                }
            }
        }
    }
}

// NetworkManager for managing multiple blockchain connections
pub struct NetworkManager {
    pub chains: RwLock<HashMap<String, Arc<dyn Chain + Send + Sync>>>,
}

impl NetworkManager {
    pub fn new() -> Self {
        NetworkManager {
            chains: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_chain(&self, name: String, chain: Arc<dyn Chain + Send + Sync>) {
        self.chains.write().await.insert(name, chain);
    }

    pub async fn get_chain(&self, name: &str) -> Option<Arc<dyn Chain + Send + Sync>> {
        self.chains.read().await.get(name).cloned()
    }

    pub async fn connect_all(
        &self,
    ) -> Result<Vec<Arc<dyn ChainReader + Send + Sync>>, Box<dyn std::error::Error + Send + Sync>>
    {
        let chains = self.chains.read().await;
        let mut connected_readers = Vec::new();
        for (_name, chain) in chains.iter() {
            let connected_reader = chain.connect().await?;
            connected_readers.push(connected_reader);
        }
        Ok(connected_readers)
    }
}

// Function to initialize NetworkManager with default chains
pub async fn initialize_network_manager(
) -> Result<NetworkManager, Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();
    let rpc_list = RpcList::new();
    let manager = NetworkManager::new();

    for (name, urls) in rpc_list.endpoints {
        manager.add_chain(name, Arc::new(EthNode::new(urls))).await;
    }

    Ok(manager)
}
