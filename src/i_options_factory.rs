pub use i_options_factory::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_options_factory {
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
    #[doc = "IOptionsFactory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IOPTIONSFACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"qTokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"creator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint88\",\"name\":\"expiry\",\"type\":\"uint88\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isCall\",\"type\":\"bool\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"strikePrice\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"collateralTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OptionCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetsRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_isCall\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_strikePrice\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createOption\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenAsCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_isCall\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_strikePrice\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCollateralToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_isCall\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_strikePrice\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getQToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"contract QToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isQToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"optionsDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"strikeAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IOptionsFactory<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IOptionsFactory<M> {
        fn clone(&self) -> Self {
            IOptionsFactory(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOptionsFactory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IOptionsFactory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOptionsFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IOptionsFactory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IOPTIONSFACTORY_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `assetsRegistry` (0x911303b4) function"]
        pub fn assets_registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([145, 19, 3, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collateralToken` (0xb2016bd4) function"]
        pub fn collateral_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([178, 1, 107, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `controller` (0xf77c4791) function"]
        pub fn controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([247, 124, 71, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createOption` (0x1fa6e922) function"]
        pub fn create_option(
            &self,
            underlying_asset: ethers::core::types::Address,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            is_call: bool,
            strike_price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [31, 166, 233, 34],
                    (underlying_asset, oracle, expiry_time, is_call, strike_price),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCollateralToken` (0x7b0815f5) function"]
        pub fn get_collateral_token(
            &self,
            underlying_asset: ethers::core::types::Address,
            q_token_as_collateral: ethers::core::types::Address,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            is_call: bool,
            strike_price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, bool)>
        {
            self.0
                .method_hash(
                    [123, 8, 21, 245],
                    (
                        underlying_asset,
                        q_token_as_collateral,
                        oracle,
                        expiry_time,
                        is_call,
                        strike_price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getQToken` (0xbd128fcb) function"]
        pub fn get_q_token(
            &self,
            underlying_asset: ethers::core::types::Address,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            is_call: bool,
            strike_price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::Address, bool)>
        {
            self.0
                .method_hash(
                    [189, 18, 143, 203],
                    (underlying_asset, oracle, expiry_time, is_call, strike_price),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `implementation` (0x5c60da1b) function"]
        pub fn implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isQToken` (0x0c74804e) function"]
        pub fn is_q_token(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([12, 116, 128, 78], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `optionsDecimals` (0x8493cec1) function"]
        pub fn options_decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([132, 147, 206, 193], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracleRegistry` (0x4bb93ab1) function"]
        pub fn oracle_registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([75, 185, 58, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `strikeAsset` (0x17d69bc8) function"]
        pub fn strike_asset(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([23, 214, 155, 200], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OptionCreated` event"]
        pub fn option_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OptionCreatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, OptionCreatedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IOptionsFactory<M> {
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
    #[ethevent(
        name = "OptionCreated",
        abi = "OptionCreated(address,address,address,address,uint88,bool,uint256,uint256)"
    )]
    pub struct OptionCreatedFilter {
        pub q_token_address: ethers::core::types::Address,
        pub creator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub underlying: ethers::core::types::Address,
        pub oracle: ethers::core::types::Address,
        pub expiry: u128,
        pub is_call: bool,
        pub strike_price: ethers::core::types::U256,
        pub collateral_token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `assetsRegistry` function with signature `assetsRegistry()` and selector `[145, 19, 3, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assetsRegistry", abi = "assetsRegistry()")]
    pub struct AssetsRegistryCall;
    #[doc = "Container type for all input parameters for the `collateralToken` function with signature `collateralToken()` and selector `[178, 1, 107, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "collateralToken", abi = "collateralToken()")]
    pub struct CollateralTokenCall;
    #[doc = "Container type for all input parameters for the `controller` function with signature `controller()` and selector `[247, 124, 71, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "controller", abi = "controller()")]
    pub struct ControllerCall;
    #[doc = "Container type for all input parameters for the `createOption` function with signature `createOption(address,address,uint88,bool,uint256)` and selector `[31, 166, 233, 34]`"]
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
        name = "createOption",
        abi = "createOption(address,address,uint88,bool,uint256)"
    )]
    pub struct CreateOptionCall {
        pub underlying_asset: ethers::core::types::Address,
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub is_call: bool,
        pub strike_price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getCollateralToken` function with signature `getCollateralToken(address,address,address,uint88,bool,uint256)` and selector `[123, 8, 21, 245]`"]
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
        name = "getCollateralToken",
        abi = "getCollateralToken(address,address,address,uint88,bool,uint256)"
    )]
    pub struct GetCollateralTokenCall {
        pub underlying_asset: ethers::core::types::Address,
        pub q_token_as_collateral: ethers::core::types::Address,
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub is_call: bool,
        pub strike_price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getQToken` function with signature `getQToken(address,address,uint88,bool,uint256)` and selector `[189, 18, 143, 203]`"]
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
        name = "getQToken",
        abi = "getQToken(address,address,uint88,bool,uint256)"
    )]
    pub struct GetQTokenCall {
        pub underlying_asset: ethers::core::types::Address,
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub is_call: bool,
        pub strike_price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    #[doc = "Container type for all input parameters for the `isQToken` function with signature `isQToken(address)` and selector `[12, 116, 128, 78]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isQToken", abi = "isQToken(address)")]
    pub struct IsQTokenCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `optionsDecimals` function with signature `optionsDecimals()` and selector `[132, 147, 206, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "optionsDecimals", abi = "optionsDecimals()")]
    pub struct OptionsDecimalsCall;
    #[doc = "Container type for all input parameters for the `oracleRegistry` function with signature `oracleRegistry()` and selector `[75, 185, 58, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracleRegistry", abi = "oracleRegistry()")]
    pub struct OracleRegistryCall;
    #[doc = "Container type for all input parameters for the `strikeAsset` function with signature `strikeAsset()` and selector `[23, 214, 155, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "strikeAsset", abi = "strikeAsset()")]
    pub struct StrikeAssetCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IOptionsFactoryCalls {
        AssetsRegistry(AssetsRegistryCall),
        CollateralToken(CollateralTokenCall),
        Controller(ControllerCall),
        CreateOption(CreateOptionCall),
        GetCollateralToken(GetCollateralTokenCall),
        GetQToken(GetQTokenCall),
        Implementation(ImplementationCall),
        IsQToken(IsQTokenCall),
        OptionsDecimals(OptionsDecimalsCall),
        OracleRegistry(OracleRegistryCall),
        StrikeAsset(StrikeAssetCall),
    }
    impl ethers::core::abi::AbiDecode for IOptionsFactoryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssetsRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::AssetsRegistry(decoded));
            }
            if let Ok(decoded) =
                <CollateralTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::CollateralToken(decoded));
            }
            if let Ok(decoded) =
                <ControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::Controller(decoded));
            }
            if let Ok(decoded) =
                <CreateOptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::CreateOption(decoded));
            }
            if let Ok(decoded) =
                <GetCollateralTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::GetCollateralToken(decoded));
            }
            if let Ok(decoded) =
                <GetQTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::GetQToken(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <IsQTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::IsQToken(decoded));
            }
            if let Ok(decoded) =
                <OptionsDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::OptionsDecimals(decoded));
            }
            if let Ok(decoded) =
                <OracleRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::OracleRegistry(decoded));
            }
            if let Ok(decoded) =
                <StrikeAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOptionsFactoryCalls::StrikeAsset(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IOptionsFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IOptionsFactoryCalls::AssetsRegistry(element) => element.encode(),
                IOptionsFactoryCalls::CollateralToken(element) => element.encode(),
                IOptionsFactoryCalls::Controller(element) => element.encode(),
                IOptionsFactoryCalls::CreateOption(element) => element.encode(),
                IOptionsFactoryCalls::GetCollateralToken(element) => element.encode(),
                IOptionsFactoryCalls::GetQToken(element) => element.encode(),
                IOptionsFactoryCalls::Implementation(element) => element.encode(),
                IOptionsFactoryCalls::IsQToken(element) => element.encode(),
                IOptionsFactoryCalls::OptionsDecimals(element) => element.encode(),
                IOptionsFactoryCalls::OracleRegistry(element) => element.encode(),
                IOptionsFactoryCalls::StrikeAsset(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IOptionsFactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOptionsFactoryCalls::AssetsRegistry(element) => element.fmt(f),
                IOptionsFactoryCalls::CollateralToken(element) => element.fmt(f),
                IOptionsFactoryCalls::Controller(element) => element.fmt(f),
                IOptionsFactoryCalls::CreateOption(element) => element.fmt(f),
                IOptionsFactoryCalls::GetCollateralToken(element) => element.fmt(f),
                IOptionsFactoryCalls::GetQToken(element) => element.fmt(f),
                IOptionsFactoryCalls::Implementation(element) => element.fmt(f),
                IOptionsFactoryCalls::IsQToken(element) => element.fmt(f),
                IOptionsFactoryCalls::OptionsDecimals(element) => element.fmt(f),
                IOptionsFactoryCalls::OracleRegistry(element) => element.fmt(f),
                IOptionsFactoryCalls::StrikeAsset(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssetsRegistryCall> for IOptionsFactoryCalls {
        fn from(var: AssetsRegistryCall) -> Self {
            IOptionsFactoryCalls::AssetsRegistry(var)
        }
    }
    impl ::std::convert::From<CollateralTokenCall> for IOptionsFactoryCalls {
        fn from(var: CollateralTokenCall) -> Self {
            IOptionsFactoryCalls::CollateralToken(var)
        }
    }
    impl ::std::convert::From<ControllerCall> for IOptionsFactoryCalls {
        fn from(var: ControllerCall) -> Self {
            IOptionsFactoryCalls::Controller(var)
        }
    }
    impl ::std::convert::From<CreateOptionCall> for IOptionsFactoryCalls {
        fn from(var: CreateOptionCall) -> Self {
            IOptionsFactoryCalls::CreateOption(var)
        }
    }
    impl ::std::convert::From<GetCollateralTokenCall> for IOptionsFactoryCalls {
        fn from(var: GetCollateralTokenCall) -> Self {
            IOptionsFactoryCalls::GetCollateralToken(var)
        }
    }
    impl ::std::convert::From<GetQTokenCall> for IOptionsFactoryCalls {
        fn from(var: GetQTokenCall) -> Self {
            IOptionsFactoryCalls::GetQToken(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for IOptionsFactoryCalls {
        fn from(var: ImplementationCall) -> Self {
            IOptionsFactoryCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<IsQTokenCall> for IOptionsFactoryCalls {
        fn from(var: IsQTokenCall) -> Self {
            IOptionsFactoryCalls::IsQToken(var)
        }
    }
    impl ::std::convert::From<OptionsDecimalsCall> for IOptionsFactoryCalls {
        fn from(var: OptionsDecimalsCall) -> Self {
            IOptionsFactoryCalls::OptionsDecimals(var)
        }
    }
    impl ::std::convert::From<OracleRegistryCall> for IOptionsFactoryCalls {
        fn from(var: OracleRegistryCall) -> Self {
            IOptionsFactoryCalls::OracleRegistry(var)
        }
    }
    impl ::std::convert::From<StrikeAssetCall> for IOptionsFactoryCalls {
        fn from(var: StrikeAssetCall) -> Self {
            IOptionsFactoryCalls::StrikeAsset(var)
        }
    }
    #[doc = "Container type for all return fields from the `assetsRegistry` function with signature `assetsRegistry()` and selector `[145, 19, 3, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssetsRegistryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `collateralToken` function with signature `collateralToken()` and selector `[178, 1, 107, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CollateralTokenReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `controller` function with signature `controller()` and selector `[247, 124, 71, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ControllerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `createOption` function with signature `createOption(address,address,uint88,bool,uint256)` and selector `[31, 166, 233, 34]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreateOptionReturn(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `getCollateralToken` function with signature `getCollateralToken(address,address,address,uint88,bool,uint256)` and selector `[123, 8, 21, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCollateralTokenReturn(pub ethers::core::types::U256, pub bool);
    #[doc = "Container type for all return fields from the `getQToken` function with signature `getQToken(address,address,uint88,bool,uint256)` and selector `[189, 18, 143, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetQTokenReturn(pub ethers::core::types::Address, pub bool);
    #[doc = "Container type for all return fields from the `implementation` function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ImplementationReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isQToken` function with signature `isQToken(address)` and selector `[12, 116, 128, 78]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsQTokenReturn(pub bool);
    #[doc = "Container type for all return fields from the `optionsDecimals` function with signature `optionsDecimals()` and selector `[132, 147, 206, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OptionsDecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `oracleRegistry` function with signature `oracleRegistry()` and selector `[75, 185, 58, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OracleRegistryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `strikeAsset` function with signature `strikeAsset()` and selector `[23, 214, 155, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct StrikeAssetReturn(pub ethers::core::types::Address);
}
