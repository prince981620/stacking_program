# NFT Staking Project

## Description
This project implements an staking mechanism using the Anchor framework on Solana. Users can stake their Native Sol,NFTs and SPL Token to earn rewards, and the system manages the staking process, including initialization, staking, unstaking, and claiming rewards.

## NFT Freezing
The project includes functionality to freeze NFTs during the staking process. When an NFT is staked, it is temporarily frozen to prevent any transfers or modifications until the user unstakes it. This ensures that the NFT remains secure while it is being staked.

### Freezing and Unfreezing NFTs
- **Freeze NFT**: The `freeze` function is called when an NFT is staked, locking it in place.
- **Thaw NFT**: The `thaw` function can be called to unlock the NFT when it is unstaked.

## SOL Transfer and Withdraw
The project includes functionality to Transfer SOL during the staking process. When an SOL is staked, it is temporarily Transferred to Stake account PDA and is locked to prevent any transfers or modifications until the user unstakes it.

### Transferring and Withdrawing SOl
- **Stake SOl**: The `transfer` function is called when SOL is staked, Transferring it in Staking Account PDA.
- **Unstake SOL**: The `close` function can be called to withdraw the SOL when it is unstaked.

## SPL Token Transfer and Withdraw
The project includes functionality to Transfer SPL Token during the staking process. When an SPL Token is staked, it is temporarily Transferred to ATA of Stake account PDA and is locked to prevent any transfers or modifications until the user unstakes it.

### Transferring and Withdrawing SPL Token
- **Stake SPL Token**: The `transfer_checked` function is called when SPL Token is staked, Transferring it in Staking Account PDA's ATA Account.
- **Unstake SPL Token**: The `transfer_checked` function can be called to withdraw the SOL when it is unstaked.

## Installation

**To install the necessary dependencies, run the following command:**

  ```bash
  yarn install
  ```
  or
  ```bash
  npm install
  ```

## Usage

**To build the project, use:**
  ```bash
  anchor build
  ```

**To run tests, use:**
  ```bash
  anchor test
  ```

**To deploy the project, use the following command:**
  ```bash
  anchor deploy
  ```


## Calculations

**To earn extra yield Asset must be locked for certain peroode otherwise flexible staking where you can unstake at any periode of time earn constant benefits only**

**Staking Periode must be greater that min_stake_periode of 60 sec**
