# TokenScout

[![build](https://github.com/SanderLoman/tokenscout/actions/workflows/build.yml/badge.svg)](https://github.com/SanderLoman/tokenscout/actions/workflows/build.yml)
[![tests](https://github.com/SanderLoman/tokenscout/actions/workflows/tests.yml/badge.svg)](https://github.com/SanderLoman/tokenscout/actions/workflows/tests.yml)
[![Codecov](https://img.shields.io/codecov/c/github/SanderLoman/tokenscout?token=JT1850HR9J)](https://app.codecov.io/gh/SanderLoman/tokenscout)

TokenScout is a tool that helps you to find the best tokens to invest in.

TokenScout is a sophisticated bot built in idiomatic Rust, designed to track all deployed tokens on the Ethereum blockchain. It identifies tokens linked to whale wallets and assesses their potential profitability. Additionally, it ensures the safety of investments by simulating token transactions to detect malicious behavior.

## Features

1. **Token Tracking**: Monitors all newly deployed tokens on the Ethereum blockchain.
2. **Whale Wallet Identification**: Walks through all branches of connected wallets to identify whale wallets associated with the token deployer.
3. **Profitability Analysis**: Assesses the potential profitability of tokens backed by whale wallets based on historical data and transaction patterns.
4. **Malicious Token Detection**: Simulates buying and selling the maximum amount of tokens to detect Salmonella tokens.
5. **Real-time Transaction Analysis**: Analyzes transactions in real-time to identify patterns associated with whale activity.
6. **Performance Metrics Tracking**: Implements robust data storage solutions to track performance metrics and outcomes.
7. **Simulation Framework**: Provides a reliable simulation framework to test for malicious behavior in new tokens.

## Installation

To install TokenScout, ensure you have Rust installed on your machine. Then, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/tokenscout.git
cd tokenscout
make
```

## License

TokenScout is released under the Apache License 2.0, See the [LICENSE](./LICENSE) file for more details.
