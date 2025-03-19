# Solana Liquidity Lending Program

This program provides functionality to interact with the Kamino lending protocol on Solana, allowing users to deposit, borrow, and repay assets.

## Features

- Deposit tokens into Kamino reserves
- Borrow assets from Kamino reserves
- Repay borrowed assets to Kamino reserves

## Prerequisites

- Rust 1.70.0 or later
- Solana CLI tools
- Anchor Framework
- Node.js and npm (for running tests)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd solana-lending
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build
```

## Testing

To run the tests:

```bash
anchor test
```

The tests will run on your local Solana validator.

## Deployment

1. Configure your Solana cluster in `Anchor.toml`:
```toml
[provider]
cluster = "mainnet-beta" # or "devnet" for testing
wallet = "~/.config/solana/id.json"
```

2. Deploy the program:
```bash
anchor deploy
```

## Usage

The program provides three main instructions:

1. `kamino_deposit_reserve_liquidity`: Deposit tokens into a Kamino reserve
2. `kamino_borrow_obligation_liquidity`: Borrow assets from a Kamino reserve
3. `kamino_repay_obligation_liquidity`: Repay borrowed assets to a Kamino reserve

Each instruction requires specific account contexts and parameters. See the program documentation for detailed usage instructions.

## Security Considerations

- Always verify the program ID before interacting with the program
- Ensure proper account validation and permissions
- Use appropriate error handling and validation
- Follow Solana best practices for account management

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.