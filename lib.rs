#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod nft_minting_machine {
    use ink::storage::Mapping;
    use ink::prelude::string::{String, ToString};

    /// Represents an NFT with a name and owner.
    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Nft {
        token_name: String,
        owner: AccountId,
    }

    /// Holds the current NFT counter.
    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct OracleData {
        pub current_index: u64,
    }

    /// Custom error types for the contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotAdmin = 0,
        OracleAlreadySet = 1,
        OracleNotSetup = 2,
        CounterOverflow = 3,
        NFTNotFound = 4,
        NotOwner = 5,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    /// Manages NFT minting and tracks state using an oracle.
    #[ink(storage)]
    pub struct NFTMintingMachine {
        admin: AccountId,
        oracle_setup: bool,
        oracle_index: u64,
        nfts: Mapping<u64, Nft>,
    }

    impl Default for NFTMintingMachine {
        fn default() -> Self {
            Self {
                admin: AccountId::from([0u8; 32]),
                oracle_setup: false,
                oracle_index: 0,
                nfts: Mapping::default(),
            }
        }
    }

    impl NFTMintingMachine {
        /// Initializes the contract with the deployer as admin.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
                ..Default::default()
            }
        }

        /// Transfers ownership of an NFT.
        #[ink(message)]
        pub fn transfer_nft(&mut self, nft_index: u64, new_owner: AccountId) -> Result<()> {
            let mut nft = self.nfts.get(nft_index).ok_or(Error::NFTNotFound)?;
            if self.env().caller() != nft.owner {
                return Err(Error::NotOwner);
            }
            nft.owner = new_owner;
            self.nfts.insert(nft_index, &nft);
            Ok(())
        }

        /// Sets up the oracle (admin-only, one-time).
        #[ink(message)]
        pub fn setup_oracle(&mut self) -> Result<()> {
            if self.env().caller() != self.admin {
                return Err(Error::NotAdmin);
            }
            if self.oracle_setup {
                return Err(Error::OracleAlreadySet);
            }
            self.oracle_setup = true;
            self.oracle_index = 0;
            Ok(())
        }

        /// Mints a new NFT and assigns it to the caller.
        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<u64> {
            if !self.oracle_setup {
                return Err(Error::OracleNotSetup);
            }
            let next_index = self.oracle_index.checked_add(1).ok_or(Error::CounterOverflow)?;
            self.oracle_index = next_index;

            let mut token_name = String::from("NFT #");
            token_name.push_str(&next_index.to_string());

            let nft = Nft {
                token_name,
                owner: self.env().caller(),
            };
            self.nfts.insert(next_index, &nft);
            Ok(next_index)
        }

        /// Returns the current oracle data.
        #[ink(message)]
        pub fn get_oracle_data(&self) -> OracleData {
            OracleData {
                current_index: self.oracle_index,
            }
        }

        /// Retrieves an NFT by its index.
        #[ink(message)]
        pub fn get_nft(&self, index: u64) -> Option<Nft> {
            self.nfts.get(index)
        }
    }
}
