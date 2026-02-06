# Anchor Vault Program

A Solana program built with Anchor framework for secure vault management.

## Setup & Deployment

### Prerequisites

* Rust and Anchor CLI installed
* Solana CLI configured

### Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/siddharth-09/sol-anchor-vault.git
   cd sol-anchor-vault
   ```
2. **Sync program keys**

   ```bash
   anchor keys sync
   ```
3. **Build the program**

   ```bash
   anchor build
   ```
4. **Configure cluster**
   Edit `Anchor.toml` and set your desired cluster:

   * `localnet` - Local testing
   * `devnet` - Devnet deployment
   * `mainnet` - Production deployment
5. **Deploy**

   ```bash
   anchor deploy
   ```

   After deployment, you'll receive a transaction signature. Verify it on [Solscan](https://solscan.io/).
6. **Run tests**

   ```bash
   anchor test
   ```

## Program Instructions

### Initialize Vault

Creates a new vault account for storing assets securely.

### Deposit

Allows users to deposit tokens into their vault.

### Withdraw

Enables vault owners to withdraw their deposited tokens.

### Close Vault

Closes the vault account and returns rent to the owner.

## Test ScreenShot


## Testing

All program instructions are covered by integration tests. Run `anchor test` to verify:

* Vault initialization
* Deposit functionality
* Withdrawal mechanics
* Vault closure

## License

[Add your license here]
