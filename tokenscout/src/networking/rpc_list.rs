use std::collections::HashMap;
pub struct RpcList {
    pub endpoints: HashMap<String, Vec<String>>,
}

impl RpcList {
    pub fn new() -> Self {
        let mut endpoints = HashMap::new();
        endpoints.insert(
            "ethereum".to_string(),
            vec![
                "https://eth.llamarpc.com".to_string(),
                "https://rpc.builder0x69.io".to_string(),
                "https://eth-pokt.nodies.app".to_string(),
                "https://rpc.public.curie.radiumblock.co/http/ethereum".to_string(),
                "https://eth-mainnet.public.blastapi.io".to_string(),
                "https://eth-mainnet.rpcfast.com?api_key=xbhWBI1Wkguk8SNMu1bvvLurPGLXmgwYeC4S6g2H7WdwFigZSmPWVZRxrskEQwIf".to_string(),
                "https://rpc.ankr.com/eth".to_string(),
                "https://ethereum.blockpi.network/v1/rpc/public".to_string(),
                "https://eth.drpc.org".to_string(),
                "https://core.gashawk.io/rpc".to_string(),
                "https://rpc.mevblocker.io/noreverts".to_string(),
                "https://rpc.mevblocker.io".to_string(),
                "https://ethereum-rpc.publicnode.com".to_string(),
                "https://virginia.rpc.blxrbdn.com".to_string(),
                "https://ethereum.rpc.subquery.network/public".to_string(),
                "https://mainnet.gateway.tenderly.co".to_string(),
                "https://gateway.tenderly.co/public/mainnet".to_string(),
                "wss://ethereum-rpc.publicnode.com".to_string(),
                "wss://eth.drpc.org".to_string(),
                "https://eth.rpc.blxrbdn.com".to_string(),
                "https://uk.rpc.blxrbdn.com".to_string(),
                "https://rpc.payload.de".to_string(),
                "https://rpc.mevblocker.io/fullprivacy".to_string(),
                "https://endpoints.omniatech.io/v1/eth/mainnet/public".to_string(),
                "https://rpc.mevblocker.io/fast".to_string(),
                "https://eth.api.onfinality.io/public".to_string(),
                "https://public.stackup.sh/api/v1/node/ethereum-mainnet".to_string(),
                "https://1rpc.io/eth".to_string(),
                "https://rpc.graffiti.farm".to_string(),
                "https://rpc.eth.gateway.fm".to_string(),
                "https://eth.meowrpc.com".to_string(),
                "https://singapore.rpc.blxrbdn.com".to_string(),
                "wss://mainnet.gateway.tenderly.co".to_string(),
                "https://api.tatum.io/v3/blockchain/node/ethereum-mainnet".to_string(),
                "https://api.zan.top/node/v1/eth/mainnet/public".to_string(),
                "https://api.securerpc.com/v1".to_string(),
                "https://rpc.lokibuilder.xyz/wallet".to_string(),
                "wss://eth-mainnet.4everland.org/ws/v1/37fa9972c1b1cd5fab542c7bdd4cde2f".to_string(),
                "https://eth.nodeconnect.org".to_string(),
                "https://rpc.flashbots.net/fast".to_string(),
                "https://api.stateless.solutions/ethereum/v1/demo".to_string(),
                "https://eth.merkle.io".to_string(),
                "https://rpc.flashbots.net".to_string(),
            ],
        );
        // Add more chains here similarly if needed

        RpcList { endpoints }
    }

    pub fn add_custom_endpoints(&mut self, chain: &str, urls: Vec<String>) {
        self.endpoints.insert(chain.to_string(), urls);
    }

    pub fn get_endpoints(&self, chain: &str) -> Option<&Vec<String>> {
        self.endpoints.get(chain)
    }
}
