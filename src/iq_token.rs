pub use iqtoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod iqtoken_mod {
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
    #[doc = "IQToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IQTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"expiryTime\",\"outputs\":[{\"internalType\":\"uint88\",\"name\":\"\",\"type\":\"uint88\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isCall\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strikeAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strikePrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"underlyingAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IQToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IQToken<M> {
        fn clone(&self) -> Self {
            IQToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IQToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IQToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IQToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IQToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IQTOKEN_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `burn` (0x9dc29fac) function"]
        pub fn burn(
            &self,
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (account, amount))
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
        #[doc = "Calls the contract's `expiryTime` (0x99bc0aea) function"]
        pub fn expiry_time(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([153, 188, 10, 234], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isCall` (0x6d636478) function"]
        pub fn is_call(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 99, 100, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (account, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracle` (0x7dc0d1d0) function"]
        pub fn oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([125, 192, 209, 208], ())
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
        #[doc = "Calls the contract's `strikePrice` (0xc52987cf) function"]
        pub fn strike_price(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([197, 41, 135, 207], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlyingAsset` (0x7158da7c) function"]
        pub fn underlying_asset(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([113, 88, 218, 124], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IQToken<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(address,uint256)` and selector `[157, 194, 159, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `controller`function with signature `controller()` and selector `[247, 124, 71, 145]`"]
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
    #[doc = "Container type for all input parameters for the `expiryTime`function with signature `expiryTime()` and selector `[153, 188, 10, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expiryTime", abi = "expiryTime()")]
    pub struct ExpiryTimeCall;
    #[doc = "Container type for all input parameters for the `isCall`function with signature `isCall()` and selector `[109, 99, 100, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isCall", abi = "isCall()")]
    pub struct IsCallCall;
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `oracle`function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    #[doc = "Container type for all input parameters for the `strikeAsset`function with signature `strikeAsset()` and selector `[23, 214, 155, 200]`"]
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
    #[doc = "Container type for all input parameters for the `strikePrice`function with signature `strikePrice()` and selector `[197, 41, 135, 207]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "strikePrice", abi = "strikePrice()")]
    pub struct StrikePriceCall;
    #[doc = "Container type for all input parameters for the `underlyingAsset`function with signature `underlyingAsset()` and selector `[113, 88, 218, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "underlyingAsset", abi = "underlyingAsset()")]
    pub struct UnderlyingAssetCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IQTokenCalls {
        Burn(BurnCall),
        Controller(ControllerCall),
        ExpiryTime(ExpiryTimeCall),
        IsCall(IsCallCall),
        Mint(MintCall),
        Oracle(OracleCall),
        StrikeAsset(StrikeAssetCall),
        StrikePrice(StrikePriceCall),
        UnderlyingAsset(UnderlyingAssetCall),
    }
    impl ethers::core::abi::AbiDecode for IQTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IQTokenCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <ControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQTokenCalls::Controller(decoded));
            }
            if let Ok(decoded) =
                <ExpiryTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQTokenCalls::ExpiryTime(decoded));
            }
            if let Ok(decoded) = <IsCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQTokenCalls::IsCall(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IQTokenCalls::Mint(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQTokenCalls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <StrikeAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQTokenCalls::StrikeAsset(decoded));
            }
            if let Ok(decoded) =
                <StrikePriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQTokenCalls::StrikePrice(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQTokenCalls::UnderlyingAsset(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IQTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IQTokenCalls::Burn(element) => element.encode(),
                IQTokenCalls::Controller(element) => element.encode(),
                IQTokenCalls::ExpiryTime(element) => element.encode(),
                IQTokenCalls::IsCall(element) => element.encode(),
                IQTokenCalls::Mint(element) => element.encode(),
                IQTokenCalls::Oracle(element) => element.encode(),
                IQTokenCalls::StrikeAsset(element) => element.encode(),
                IQTokenCalls::StrikePrice(element) => element.encode(),
                IQTokenCalls::UnderlyingAsset(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IQTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IQTokenCalls::Burn(element) => element.fmt(f),
                IQTokenCalls::Controller(element) => element.fmt(f),
                IQTokenCalls::ExpiryTime(element) => element.fmt(f),
                IQTokenCalls::IsCall(element) => element.fmt(f),
                IQTokenCalls::Mint(element) => element.fmt(f),
                IQTokenCalls::Oracle(element) => element.fmt(f),
                IQTokenCalls::StrikeAsset(element) => element.fmt(f),
                IQTokenCalls::StrikePrice(element) => element.fmt(f),
                IQTokenCalls::UnderlyingAsset(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BurnCall> for IQTokenCalls {
        fn from(var: BurnCall) -> Self {
            IQTokenCalls::Burn(var)
        }
    }
    impl ::std::convert::From<ControllerCall> for IQTokenCalls {
        fn from(var: ControllerCall) -> Self {
            IQTokenCalls::Controller(var)
        }
    }
    impl ::std::convert::From<ExpiryTimeCall> for IQTokenCalls {
        fn from(var: ExpiryTimeCall) -> Self {
            IQTokenCalls::ExpiryTime(var)
        }
    }
    impl ::std::convert::From<IsCallCall> for IQTokenCalls {
        fn from(var: IsCallCall) -> Self {
            IQTokenCalls::IsCall(var)
        }
    }
    impl ::std::convert::From<MintCall> for IQTokenCalls {
        fn from(var: MintCall) -> Self {
            IQTokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<OracleCall> for IQTokenCalls {
        fn from(var: OracleCall) -> Self {
            IQTokenCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<StrikeAssetCall> for IQTokenCalls {
        fn from(var: StrikeAssetCall) -> Self {
            IQTokenCalls::StrikeAsset(var)
        }
    }
    impl ::std::convert::From<StrikePriceCall> for IQTokenCalls {
        fn from(var: StrikePriceCall) -> Self {
            IQTokenCalls::StrikePrice(var)
        }
    }
    impl ::std::convert::From<UnderlyingAssetCall> for IQTokenCalls {
        fn from(var: UnderlyingAssetCall) -> Self {
            IQTokenCalls::UnderlyingAsset(var)
        }
    }
}
