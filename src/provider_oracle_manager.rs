pub use provider_oracle_manager::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod provider_oracle_manager {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ProviderOracleManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PROVIDERORACLEMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetOracles\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_strikePrice\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidOption\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_calldata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistry\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct ProviderOracleManager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ProviderOracleManager<M> {
        fn clone(&self) -> Self {
            ProviderOracleManager(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ProviderOracleManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ProviderOracleManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ProviderOracleManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ProviderOracleManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                PROVIDERORACLEMANAGER_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `addAssetOracle` (0x8cb4ba23) function"]
        pub fn add_asset_oracle(
            &self,
            asset: ethers::core::types::Address,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 180, 186, 35], (asset, oracle))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetOracles` (0x2b663986) function"]
        pub fn asset_oracles(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([43, 102, 57, 134], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assets` (0xcf35bdd0) function"]
        pub fn assets(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([207, 53, 189, 208], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetOracle` (0x0db015f4) function"]
        pub fn get_asset_oracle(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 176, 21, 244], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetsLength` (0xa89d490c) function"]
        pub fn get_assets_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([168, 157, 73, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentPrice` (0x84cc315b) function"]
        pub fn get_current_price(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([132, 204, 49, 91], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isValidOption` (0xe2411261) function"]
        pub fn is_valid_option(
            &self,
            underlying_asset: ethers::core::types::Address,
            expiry_time: u128,
            strike_price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [226, 65, 18, 97],
                    (underlying_asset, expiry_time, strike_price),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `priceRegistry` (0x535131d7) function"]
        pub fn price_registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([83, 81, 49, 215], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpiryPriceInRegistry` (0xb9bd8796) function"]
        pub fn set_expiry_price_in_registry(
            &self,
            asset: ethers::core::types::Address,
            expiry_timestamp: u128,
            calldata: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 189, 135, 150], (asset, expiry_timestamp, calldata))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OracleAdded` event"]
        pub fn oracle_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OracleAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ProviderOracleManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ProviderOracleManager<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "OracleAdded", abi = "OracleAdded(address,address)")]
    pub struct OracleAddedFilter {
        pub asset: ethers::core::types::Address,
        pub oracle: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ProviderOracleManagerEvents {
        OracleAddedFilter(OracleAddedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for ProviderOracleManagerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = OracleAddedFilter::decode_log(log) {
                return Ok(ProviderOracleManagerEvents::OracleAddedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ProviderOracleManagerEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ProviderOracleManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ProviderOracleManagerEvents::OracleAddedFilter(element) => element.fmt(f),
                ProviderOracleManagerEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addAssetOracle` function with signature `addAssetOracle(address,address)` and selector `[140, 180, 186, 35]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addAssetOracle", abi = "addAssetOracle(address,address)")]
    pub struct AddAssetOracleCall {
        pub asset: ethers::core::types::Address,
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `assetOracles` function with signature `assetOracles(address)` and selector `[43, 102, 57, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assetOracles", abi = "assetOracles(address)")]
    pub struct AssetOraclesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `assets` function with signature `assets(uint256)` and selector `[207, 53, 189, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assets", abi = "assets(uint256)")]
    pub struct AssetsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `getAssetOracle` function with signature `getAssetOracle(address)` and selector `[13, 176, 21, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetOracle", abi = "getAssetOracle(address)")]
    pub struct GetAssetOracleCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAssetsLength` function with signature `getAssetsLength()` and selector `[168, 157, 73, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetsLength", abi = "getAssetsLength()")]
    pub struct GetAssetsLengthCall;
    #[doc = "Container type for all input parameters for the `getCurrentPrice` function with signature `getCurrentPrice(address)` and selector `[132, 204, 49, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCurrentPrice", abi = "getCurrentPrice(address)")]
    pub struct GetCurrentPriceCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isValidOption` function with signature `isValidOption(address,uint88,uint256)` and selector `[226, 65, 18, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isValidOption", abi = "isValidOption(address,uint88,uint256)")]
    pub struct IsValidOptionCall {
        pub underlying_asset: ethers::core::types::Address,
        pub expiry_time: u128,
        pub strike_price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `priceRegistry` function with signature `priceRegistry()` and selector `[83, 81, 49, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceRegistry", abi = "priceRegistry()")]
    pub struct PriceRegistryCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistry` function with signature `setExpiryPriceInRegistry(address,uint88,bytes)` and selector `[185, 189, 135, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "setExpiryPriceInRegistry",
        abi = "setExpiryPriceInRegistry(address,uint88,bytes)"
    )]
    pub struct SetExpiryPriceInRegistryCall {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
        pub calldata: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ProviderOracleManagerCalls {
        AddAssetOracle(AddAssetOracleCall),
        AssetOracles(AssetOraclesCall),
        Assets(AssetsCall),
        GetAssetOracle(GetAssetOracleCall),
        GetAssetsLength(GetAssetsLengthCall),
        GetCurrentPrice(GetCurrentPriceCall),
        IsValidOption(IsValidOptionCall),
        Owner(OwnerCall),
        PriceRegistry(PriceRegistryCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetExpiryPriceInRegistry(SetExpiryPriceInRegistryCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for ProviderOracleManagerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddAssetOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::AddAssetOracle(decoded));
            }
            if let Ok(decoded) =
                <AssetOraclesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::AssetOracles(decoded));
            }
            if let Ok(decoded) = <AssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::Assets(decoded));
            }
            if let Ok(decoded) =
                <GetAssetOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::GetAssetOracle(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::GetAssetsLength(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::GetCurrentPrice(decoded));
            }
            if let Ok(decoded) =
                <IsValidOptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::IsValidOption(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PriceRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::PriceRegistry(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ProviderOracleManagerCalls::SetExpiryPriceInRegistry(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProviderOracleManagerCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ProviderOracleManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ProviderOracleManagerCalls::AddAssetOracle(element) => element.encode(),
                ProviderOracleManagerCalls::AssetOracles(element) => element.encode(),
                ProviderOracleManagerCalls::Assets(element) => element.encode(),
                ProviderOracleManagerCalls::GetAssetOracle(element) => element.encode(),
                ProviderOracleManagerCalls::GetAssetsLength(element) => element.encode(),
                ProviderOracleManagerCalls::GetCurrentPrice(element) => element.encode(),
                ProviderOracleManagerCalls::IsValidOption(element) => element.encode(),
                ProviderOracleManagerCalls::Owner(element) => element.encode(),
                ProviderOracleManagerCalls::PriceRegistry(element) => element.encode(),
                ProviderOracleManagerCalls::RenounceOwnership(element) => element.encode(),
                ProviderOracleManagerCalls::SetExpiryPriceInRegistry(element) => element.encode(),
                ProviderOracleManagerCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ProviderOracleManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ProviderOracleManagerCalls::AddAssetOracle(element) => element.fmt(f),
                ProviderOracleManagerCalls::AssetOracles(element) => element.fmt(f),
                ProviderOracleManagerCalls::Assets(element) => element.fmt(f),
                ProviderOracleManagerCalls::GetAssetOracle(element) => element.fmt(f),
                ProviderOracleManagerCalls::GetAssetsLength(element) => element.fmt(f),
                ProviderOracleManagerCalls::GetCurrentPrice(element) => element.fmt(f),
                ProviderOracleManagerCalls::IsValidOption(element) => element.fmt(f),
                ProviderOracleManagerCalls::Owner(element) => element.fmt(f),
                ProviderOracleManagerCalls::PriceRegistry(element) => element.fmt(f),
                ProviderOracleManagerCalls::RenounceOwnership(element) => element.fmt(f),
                ProviderOracleManagerCalls::SetExpiryPriceInRegistry(element) => element.fmt(f),
                ProviderOracleManagerCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddAssetOracleCall> for ProviderOracleManagerCalls {
        fn from(var: AddAssetOracleCall) -> Self {
            ProviderOracleManagerCalls::AddAssetOracle(var)
        }
    }
    impl ::std::convert::From<AssetOraclesCall> for ProviderOracleManagerCalls {
        fn from(var: AssetOraclesCall) -> Self {
            ProviderOracleManagerCalls::AssetOracles(var)
        }
    }
    impl ::std::convert::From<AssetsCall> for ProviderOracleManagerCalls {
        fn from(var: AssetsCall) -> Self {
            ProviderOracleManagerCalls::Assets(var)
        }
    }
    impl ::std::convert::From<GetAssetOracleCall> for ProviderOracleManagerCalls {
        fn from(var: GetAssetOracleCall) -> Self {
            ProviderOracleManagerCalls::GetAssetOracle(var)
        }
    }
    impl ::std::convert::From<GetAssetsLengthCall> for ProviderOracleManagerCalls {
        fn from(var: GetAssetsLengthCall) -> Self {
            ProviderOracleManagerCalls::GetAssetsLength(var)
        }
    }
    impl ::std::convert::From<GetCurrentPriceCall> for ProviderOracleManagerCalls {
        fn from(var: GetCurrentPriceCall) -> Self {
            ProviderOracleManagerCalls::GetCurrentPrice(var)
        }
    }
    impl ::std::convert::From<IsValidOptionCall> for ProviderOracleManagerCalls {
        fn from(var: IsValidOptionCall) -> Self {
            ProviderOracleManagerCalls::IsValidOption(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ProviderOracleManagerCalls {
        fn from(var: OwnerCall) -> Self {
            ProviderOracleManagerCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PriceRegistryCall> for ProviderOracleManagerCalls {
        fn from(var: PriceRegistryCall) -> Self {
            ProviderOracleManagerCalls::PriceRegistry(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ProviderOracleManagerCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ProviderOracleManagerCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryCall> for ProviderOracleManagerCalls {
        fn from(var: SetExpiryPriceInRegistryCall) -> Self {
            ProviderOracleManagerCalls::SetExpiryPriceInRegistry(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ProviderOracleManagerCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ProviderOracleManagerCalls::TransferOwnership(var)
        }
    }
    #[doc = "Container type for all return fields from the `assetOracles` function with signature `assetOracles(address)` and selector `[43, 102, 57, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssetOraclesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `assets` function with signature `assets(uint256)` and selector `[207, 53, 189, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssetsReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAssetOracle` function with signature `getAssetOracle(address)` and selector `[13, 176, 21, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAssetOracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAssetsLength` function with signature `getAssetsLength()` and selector `[168, 157, 73, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAssetsLengthReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getCurrentPrice` function with signature `getCurrentPrice(address)` and selector `[132, 204, 49, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCurrentPriceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isValidOption` function with signature `isValidOption(address,uint88,uint256)` and selector `[226, 65, 18, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsValidOptionReturn(pub bool);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `priceRegistry` function with signature `priceRegistry()` and selector `[83, 81, 49, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PriceRegistryReturn(pub ethers::core::types::Address);
}
