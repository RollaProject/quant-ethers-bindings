pub use ichainlinkfixedtimeoraclemanager_mod::*;
#[allow(clippy::too_many_arguments)]
mod ichainlinkfixedtimeoraclemanager_mod {
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
    #[doc = "IChainlinkFixedTimeOracleManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICHAINLINKFIXEDTIMEORACLEMANAGER_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fixedTime\",\"type\":\"uint24\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isValidTime\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FixedTimeUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint88\",\"name\":\"expiryTimestamp\",\"type\":\"uint88\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expiryRoundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"priceSubmitter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isFallback\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceRegistrySubmission\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetOracles\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"chainlinkFixedTimeUpdates\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fallbackPeriodSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_strikePrice\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidOption\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"searchRoundToSubmit\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_calldata\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistry\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_roundIdAfterExpiry\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryByRound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryFallback\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fixedTime\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isValidTime\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFixedTimeUpdate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"strikeAssetDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]}]") . expect ("invalid abi")
    });
    pub struct IChainlinkFixedTimeOracleManager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IChainlinkFixedTimeOracleManager<M> {
        fn clone(&self) -> Self {
            IChainlinkFixedTimeOracleManager(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IChainlinkFixedTimeOracleManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IChainlinkFixedTimeOracleManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IChainlinkFixedTimeOracleManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IChainlinkFixedTimeOracleManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                ICHAINLINKFIXEDTIMEORACLEMANAGER_ABI.clone(),
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
        #[doc = "Calls the contract's `chainlinkFixedTimeUpdates` (0x87e4e00c) function"]
        pub fn chainlink_fixed_time_updates(
            &self,
            p0: u32,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([135, 228, 224, 12], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fallbackPeriodSeconds` (0xab91dcb9) function"]
        pub fn fallback_period_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([171, 145, 220, 185], ())
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
        #[doc = "Calls the contract's `searchRoundToSubmit` (0x0a821fea) function"]
        pub fn search_round_to_submit(
            &self,
            asset: ethers::core::types::Address,
            expiry_timestamp: u128,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([10, 130, 31, 234], (asset, expiry_timestamp))
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
        #[doc = "Calls the contract's `setExpiryPriceInRegistryByRound` (0x4ac53653) function"]
        pub fn set_expiry_price_in_registry_by_round(
            &self,
            asset: ethers::core::types::Address,
            expiry_timestamp: u128,
            round_id_after_expiry: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [74, 197, 54, 83],
                    (asset, expiry_timestamp, round_id_after_expiry),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpiryPriceInRegistryFallback` (0x6ee3827b) function"]
        pub fn set_expiry_price_in_registry_fallback(
            &self,
            asset: ethers::core::types::Address,
            expiry_timestamp: u128,
            price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 227, 130, 123], (asset, expiry_timestamp, price))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFixedTimeUpdate` (0xc6973b96) function"]
        pub fn set_fixed_time_update(
            &self,
            fixed_time: u32,
            is_valid_time: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 151, 59, 150], (fixed_time, is_valid_time))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `strikeAssetDecimals` (0xc1325661) function"]
        pub fn strike_asset_decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([193, 50, 86, 97], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `FixedTimeUpdate` event"]
        pub fn fixed_time_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FixedTimeUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OracleAdded` event"]
        pub fn oracle_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OracleAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PriceRegistrySubmission` event"]
        pub fn price_registry_submission_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PriceRegistrySubmissionFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, IChainlinkFixedTimeOracleManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IChainlinkFixedTimeOracleManager<M>
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
    #[ethevent(name = "FixedTimeUpdate", abi = "FixedTimeUpdate(uint24,bool)")]
    pub struct FixedTimeUpdateFilter {
        pub fixed_time: u32,
        pub is_valid_time: bool,
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
        name = "PriceRegistrySubmission",
        abi = "PriceRegistrySubmission(address,uint88,uint256,uint256,address,bool)"
    )]
    pub struct PriceRegistrySubmissionFilter {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
        pub price: ethers::core::types::U256,
        pub expiry_round_id: ethers::core::types::U256,
        pub price_submitter: ethers::core::types::Address,
        pub is_fallback: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IChainlinkFixedTimeOracleManagerEvents {
        FixedTimeUpdateFilter(FixedTimeUpdateFilter),
        OracleAddedFilter(OracleAddedFilter),
        PriceRegistrySubmissionFilter(PriceRegistrySubmissionFilter),
    }
    impl ethers::contract::EthLogDecode for IChainlinkFixedTimeOracleManagerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FixedTimeUpdateFilter::decode_log(log) {
                return Ok(IChainlinkFixedTimeOracleManagerEvents::FixedTimeUpdateFilter(decoded));
            }
            if let Ok(decoded) = OracleAddedFilter::decode_log(log) {
                return Ok(IChainlinkFixedTimeOracleManagerEvents::OracleAddedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PriceRegistrySubmissionFilter::decode_log(log) {
                return Ok(
                    IChainlinkFixedTimeOracleManagerEvents::PriceRegistrySubmissionFilter(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IChainlinkFixedTimeOracleManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IChainlinkFixedTimeOracleManagerEvents::FixedTimeUpdateFilter(element) => {
                    element.fmt(f)
                }
                IChainlinkFixedTimeOracleManagerEvents::OracleAddedFilter(element) => {
                    element.fmt(f)
                }
                IChainlinkFixedTimeOracleManagerEvents::PriceRegistrySubmissionFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addAssetOracle`function with signature `addAssetOracle(address,address)` and selector `[140, 180, 186, 35]`"]
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
    #[doc = "Container type for all input parameters for the `assetOracles`function with signature `assetOracles(address)` and selector `[43, 102, 57, 134]`"]
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
    #[doc = "Container type for all input parameters for the `assets`function with signature `assets(uint256)` and selector `[207, 53, 189, 208]`"]
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
    #[doc = "Container type for all input parameters for the `chainlinkFixedTimeUpdates`function with signature `chainlinkFixedTimeUpdates(uint24)` and selector `[135, 228, 224, 12]`"]
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
        name = "chainlinkFixedTimeUpdates",
        abi = "chainlinkFixedTimeUpdates(uint24)"
    )]
    pub struct ChainlinkFixedTimeUpdatesCall(pub u32);
    #[doc = "Container type for all input parameters for the `fallbackPeriodSeconds`function with signature `fallbackPeriodSeconds()` and selector `[171, 145, 220, 185]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fallbackPeriodSeconds", abi = "fallbackPeriodSeconds()")]
    pub struct FallbackPeriodSecondsCall;
    #[doc = "Container type for all input parameters for the `getAssetOracle`function with signature `getAssetOracle(address)` and selector `[13, 176, 21, 244]`"]
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
    #[doc = "Container type for all input parameters for the `getAssetsLength`function with signature `getAssetsLength()` and selector `[168, 157, 73, 12]`"]
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
    #[doc = "Container type for all input parameters for the `getCurrentPrice`function with signature `getCurrentPrice(address)` and selector `[132, 204, 49, 91]`"]
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
    #[doc = "Container type for all input parameters for the `isValidOption`function with signature `isValidOption(address,uint88,uint256)` and selector `[226, 65, 18, 97]`"]
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
    #[doc = "Container type for all input parameters for the `searchRoundToSubmit`function with signature `searchRoundToSubmit(address,uint88)` and selector `[10, 130, 31, 234]`"]
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
        name = "searchRoundToSubmit",
        abi = "searchRoundToSubmit(address,uint88)"
    )]
    pub struct SearchRoundToSubmitCall {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
    }
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistry`function with signature `setExpiryPriceInRegistry(address,uint88,bytes)` and selector `[185, 189, 135, 150]`"]
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
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistryByRound`function with signature `setExpiryPriceInRegistryByRound(address,uint88,uint256)` and selector `[74, 197, 54, 83]`"]
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
        name = "setExpiryPriceInRegistryByRound",
        abi = "setExpiryPriceInRegistryByRound(address,uint88,uint256)"
    )]
    pub struct SetExpiryPriceInRegistryByRoundCall {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
        pub round_id_after_expiry: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistryFallback`function with signature `setExpiryPriceInRegistryFallback(address,uint88,uint256)` and selector `[110, 227, 130, 123]`"]
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
        name = "setExpiryPriceInRegistryFallback",
        abi = "setExpiryPriceInRegistryFallback(address,uint88,uint256)"
    )]
    pub struct SetExpiryPriceInRegistryFallbackCall {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
        pub price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setFixedTimeUpdate`function with signature `setFixedTimeUpdate(uint24,bool)` and selector `[198, 151, 59, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFixedTimeUpdate", abi = "setFixedTimeUpdate(uint24,bool)")]
    pub struct SetFixedTimeUpdateCall {
        pub fixed_time: u32,
        pub is_valid_time: bool,
    }
    #[doc = "Container type for all input parameters for the `strikeAssetDecimals`function with signature `strikeAssetDecimals()` and selector `[193, 50, 86, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "strikeAssetDecimals", abi = "strikeAssetDecimals()")]
    pub struct StrikeAssetDecimalsCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IChainlinkFixedTimeOracleManagerCalls {
        AddAssetOracle(AddAssetOracleCall),
        AssetOracles(AssetOraclesCall),
        Assets(AssetsCall),
        ChainlinkFixedTimeUpdates(ChainlinkFixedTimeUpdatesCall),
        FallbackPeriodSeconds(FallbackPeriodSecondsCall),
        GetAssetOracle(GetAssetOracleCall),
        GetAssetsLength(GetAssetsLengthCall),
        GetCurrentPrice(GetCurrentPriceCall),
        IsValidOption(IsValidOptionCall),
        SearchRoundToSubmit(SearchRoundToSubmitCall),
        SetExpiryPriceInRegistry(SetExpiryPriceInRegistryCall),
        SetExpiryPriceInRegistryByRound(SetExpiryPriceInRegistryByRoundCall),
        SetExpiryPriceInRegistryFallback(SetExpiryPriceInRegistryFallbackCall),
        SetFixedTimeUpdate(SetFixedTimeUpdateCall),
        StrikeAssetDecimals(StrikeAssetDecimalsCall),
    }
    impl ethers::core::abi::AbiDecode for IChainlinkFixedTimeOracleManagerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddAssetOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::AddAssetOracle(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AssetOraclesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::AssetOracles(decoded));
            }
            if let Ok(decoded) = <AssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::Assets(decoded));
            }
            if let Ok(decoded) =
                <ChainlinkFixedTimeUpdatesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IChainlinkFixedTimeOracleManagerCalls::ChainlinkFixedTimeUpdates(decoded),
                );
            }
            if let Ok(decoded) =
                <FallbackPeriodSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::FallbackPeriodSeconds(decoded));
            }
            if let Ok(decoded) =
                <GetAssetOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::GetAssetOracle(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetAssetsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::GetAssetsLength(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetCurrentPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::GetCurrentPrice(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IsValidOptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::IsValidOption(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SearchRoundToSubmitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::SearchRoundToSubmit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistry(decoded));
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryByRoundCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryByRound(decoded),
                );
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryFallbackCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryFallback(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <SetFixedTimeUpdateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::SetFixedTimeUpdate(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <StrikeAssetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IChainlinkFixedTimeOracleManagerCalls::StrikeAssetDecimals(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IChainlinkFixedTimeOracleManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IChainlinkFixedTimeOracleManagerCalls::AddAssetOracle(element) => element.encode(),
                IChainlinkFixedTimeOracleManagerCalls::AssetOracles(element) => element.encode(),
                IChainlinkFixedTimeOracleManagerCalls::Assets(element) => element.encode(),
                IChainlinkFixedTimeOracleManagerCalls::ChainlinkFixedTimeUpdates(element) => {
                    element.encode()
                }
                IChainlinkFixedTimeOracleManagerCalls::FallbackPeriodSeconds(element) => {
                    element.encode()
                }
                IChainlinkFixedTimeOracleManagerCalls::GetAssetOracle(element) => element.encode(),
                IChainlinkFixedTimeOracleManagerCalls::GetAssetsLength(element) => element.encode(),
                IChainlinkFixedTimeOracleManagerCalls::GetCurrentPrice(element) => element.encode(),
                IChainlinkFixedTimeOracleManagerCalls::IsValidOption(element) => element.encode(),
                IChainlinkFixedTimeOracleManagerCalls::SearchRoundToSubmit(element) => {
                    element.encode()
                }
                IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistry(element) => {
                    element.encode()
                }
                IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryByRound(element) => {
                    element.encode()
                }
                IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryFallback(
                    element,
                ) => element.encode(),
                IChainlinkFixedTimeOracleManagerCalls::SetFixedTimeUpdate(element) => {
                    element.encode()
                }
                IChainlinkFixedTimeOracleManagerCalls::StrikeAssetDecimals(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for IChainlinkFixedTimeOracleManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IChainlinkFixedTimeOracleManagerCalls::AddAssetOracle(element) => element.fmt(f),
                IChainlinkFixedTimeOracleManagerCalls::AssetOracles(element) => element.fmt(f),
                IChainlinkFixedTimeOracleManagerCalls::Assets(element) => element.fmt(f),
                IChainlinkFixedTimeOracleManagerCalls::ChainlinkFixedTimeUpdates(element) => {
                    element.fmt(f)
                }
                IChainlinkFixedTimeOracleManagerCalls::FallbackPeriodSeconds(element) => {
                    element.fmt(f)
                }
                IChainlinkFixedTimeOracleManagerCalls::GetAssetOracle(element) => element.fmt(f),
                IChainlinkFixedTimeOracleManagerCalls::GetAssetsLength(element) => element.fmt(f),
                IChainlinkFixedTimeOracleManagerCalls::GetCurrentPrice(element) => element.fmt(f),
                IChainlinkFixedTimeOracleManagerCalls::IsValidOption(element) => element.fmt(f),
                IChainlinkFixedTimeOracleManagerCalls::SearchRoundToSubmit(element) => {
                    element.fmt(f)
                }
                IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistry(element) => {
                    element.fmt(f)
                }
                IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryByRound(element) => {
                    element.fmt(f)
                }
                IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryFallback(
                    element,
                ) => element.fmt(f),
                IChainlinkFixedTimeOracleManagerCalls::SetFixedTimeUpdate(element) => {
                    element.fmt(f)
                }
                IChainlinkFixedTimeOracleManagerCalls::StrikeAssetDecimals(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AddAssetOracleCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: AddAssetOracleCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::AddAssetOracle(var)
        }
    }
    impl ::std::convert::From<AssetOraclesCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: AssetOraclesCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::AssetOracles(var)
        }
    }
    impl ::std::convert::From<AssetsCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: AssetsCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::Assets(var)
        }
    }
    impl ::std::convert::From<ChainlinkFixedTimeUpdatesCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: ChainlinkFixedTimeUpdatesCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::ChainlinkFixedTimeUpdates(var)
        }
    }
    impl ::std::convert::From<FallbackPeriodSecondsCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: FallbackPeriodSecondsCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::FallbackPeriodSeconds(var)
        }
    }
    impl ::std::convert::From<GetAssetOracleCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: GetAssetOracleCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::GetAssetOracle(var)
        }
    }
    impl ::std::convert::From<GetAssetsLengthCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: GetAssetsLengthCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::GetAssetsLength(var)
        }
    }
    impl ::std::convert::From<GetCurrentPriceCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: GetCurrentPriceCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::GetCurrentPrice(var)
        }
    }
    impl ::std::convert::From<IsValidOptionCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: IsValidOptionCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::IsValidOption(var)
        }
    }
    impl ::std::convert::From<SearchRoundToSubmitCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: SearchRoundToSubmitCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::SearchRoundToSubmit(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: SetExpiryPriceInRegistryCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistry(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryByRoundCall>
        for IChainlinkFixedTimeOracleManagerCalls
    {
        fn from(var: SetExpiryPriceInRegistryByRoundCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryByRound(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryFallbackCall>
        for IChainlinkFixedTimeOracleManagerCalls
    {
        fn from(var: SetExpiryPriceInRegistryFallbackCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryFallback(var)
        }
    }
    impl ::std::convert::From<SetFixedTimeUpdateCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: SetFixedTimeUpdateCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::SetFixedTimeUpdate(var)
        }
    }
    impl ::std::convert::From<StrikeAssetDecimalsCall> for IChainlinkFixedTimeOracleManagerCalls {
        fn from(var: StrikeAssetDecimalsCall) -> Self {
            IChainlinkFixedTimeOracleManagerCalls::StrikeAssetDecimals(var)
        }
    }
}
