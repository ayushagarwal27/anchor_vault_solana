# Vault Smart Contract (Anchor)

A Solana program enabling users to create, deposit into, withdraw from, and close a user-specific account.

- A smart contract developed with Anchor Rust framework
- User can open a vault, which will be system account unique to user
- User can deposit amount into vault
- User can withdraw amount from vault
- User can close vault

## Functions

- Initialise Vault
- Deposit amount into vault
- Withdraw amount from vault
- Close vault account

## Enhancements

- VaultState PDA stores vault and vault_state bump for faster access, resulting in less compute

