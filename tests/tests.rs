/// # NFTMintingMachine Contract Test Suite
///
/// This module contains comprehensive tests for the `NFTMintingMachine` contract,
/// covering core functionality, edge cases, and security requirements.
///
/// ## Test Accounts Convention:
/// - **Alice**: Default caller/initiator (ink! default account)
/// - **Bob**: Counterparty account
/// - **Charlie**: Unauthorized third party
#[cfg(test)]
mod tests {
    use nft_minting_machine::{NFTMintingMachine, Error};
    use ink::env::{test, DefaultEnvironment};

    /// Tests the `setup_oracle` function to ensure the oracle is initialized correctly.
    /// - Verifies that the oracle can be set up successfully.
    /// - Verifies that subsequent attempts to set up the oracle fail with `Error::OracleAlreadySet`.
    #[ink::test]
    fn test_setup_oracle() {
        let mut contract = NFTMintingMachine::new();
        assert_eq!(contract.setup_oracle(), Ok(()));
        assert_eq!(contract.setup_oracle(), Err(Error::OracleAlreadySet));
    }

    /// Tests the `mint_token` function to ensure NFTs can be minted correctly.
    /// - Verifies that minting fails if the oracle is not set up.
    /// - Verifies that minting succeeds after the oracle is set up.
    /// - Verifies that the minted NFT has the correct token name and owner.
    #[ink::test]
    fn test_mint_token() {
        let mut contract = NFTMintingMachine::new();
        assert_eq!(contract.mint_token(), Err(Error::OracleNotSetup));

        contract.setup_oracle().unwrap();
        let token_index = contract.mint_token().unwrap();
        assert_eq!(token_index, 1);

        let nft = contract.get_nft(token_index).unwrap();
        assert_eq!(nft.token_name(), "NFT #1");
        assert_eq!(nft.owner(), &test::default_accounts::<DefaultEnvironment>().alice);
    }

    /// Tests the `transfer_nft` function to ensure NFTs can be transferred correctly.
    /// - Verifies that ownership transfer succeeds when initiated by the current owner.
    /// - Verifies that ownership transfer fails when initiated by a non-owner.
    #[ink::test]
    fn test_transfer_nft() {
        let mut contract = NFTMintingMachine::new();
        contract.setup_oracle().unwrap();
        let token_index = contract.mint_token().unwrap();

        let accounts = test::default_accounts::<DefaultEnvironment>();
        assert_eq!(contract.transfer_nft(token_index, accounts.bob), Ok(()));

        let nft = contract.get_nft(token_index).unwrap();
        assert_eq!(nft.owner(), &accounts.bob);

        test::set_caller::<DefaultEnvironment>(accounts.charlie);
        assert_eq!(contract.transfer_nft(token_index, accounts.alice), Err(Error::NotOwner));
    }

    /// Tests the `get_oracle_data` function to ensure the oracle's state is reported correctly.
    /// - Verifies that the oracle's `current_index` starts at `0`.
    /// - Verifies that the `current_index` increments correctly after minting NFTs.
    #[ink::test]
    fn test_get_oracle_data() {
        let mut contract = NFTMintingMachine::new();
        assert_eq!(contract.get_oracle_data().current_index, 0);

        contract.setup_oracle().unwrap();
        contract.mint_token().unwrap();
        assert_eq!(contract.get_oracle_data().current_index, 1);
    }
}
