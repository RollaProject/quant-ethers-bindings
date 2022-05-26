pub use ioracleregistry_mod::*;
#[allow(clippy::too_many_arguments)]
mod ioracleregistry_mod {
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
    #[doc = "IOracleRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IORACLEREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ActivatedOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint248\",\"name\":\"oracleId\",\"type\":\"uint248\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddedOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DeactivatedOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"activateOracle\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addOracle\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deactivateOracle\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOracleId\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclesLength\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOracleActive\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOracleRegistered\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleInfo\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracles\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IOracleRegistry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IOracleRegistry<M> {
        fn clone(&self) -> Self {
            IOracleRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOracleRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IOracleRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOracleRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IOracleRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IORACLEREGISTRY_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `activateOracle` (0x7a8a0147) function"]
        pub fn activate_oracle(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([122, 138, 1, 71], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addOracle` (0xdf5dd1a5) function"]
        pub fn add_oracle(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([223, 93, 209, 165], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deactivateOracle` (0xed69c604) function"]
        pub fn deactivate_oracle(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 105, 198, 4], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOracleId` (0xbc13872a) function"]
        pub fn get_oracle_id(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([188, 19, 135, 42], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOraclesLength` (0xe308b275) function"]
        pub fn get_oracles_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([227, 8, 178, 117], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOracleActive` (0x05f29d0f) function"]
        pub fn is_oracle_active(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([5, 242, 157, 15], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOracleRegistered` (0xb9b55c9a) function"]
        pub fn is_oracle_registered(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([185, 181, 92, 154], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracleInfo` (0xbc623feb) function"]
        pub fn oracle_info(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::U256)>
        {
            self.0
                .method_hash([188, 98, 63, 235], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracles` (0x5b69a7d8) function"]
        pub fn oracles(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([91, 105, 167, 216], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ActivatedOracle` event"]
        pub fn activated_oracle_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ActivatedOracleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AddedOracle` event"]
        pub fn added_oracle_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddedOracleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DeactivatedOracle` event"]
        pub fn deactivated_oracle_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DeactivatedOracleFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IOracleRegistryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IOracleRegistry<M> {
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
    #[ethevent(name = "ActivatedOracle", abi = "ActivatedOracle(address)")]
    pub struct ActivatedOracleFilter {
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
    #[ethevent(name = "AddedOracle", abi = "AddedOracle(address,uint248)")]
    pub struct AddedOracleFilter {
        pub oracle: ethers::core::types::Address,
        pub oracle_id: ethers::core::types::U256,
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
    #[ethevent(name = "DeactivatedOracle", abi = "DeactivatedOracle(address)")]
    pub struct DeactivatedOracleFilter {
        pub oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IOracleRegistryEvents {
        ActivatedOracleFilter(ActivatedOracleFilter),
        AddedOracleFilter(AddedOracleFilter),
        DeactivatedOracleFilter(DeactivatedOracleFilter),
    }
    impl ethers::contract::EthLogDecode for IOracleRegistryEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ActivatedOracleFilter::decode_log(log) {
                return Ok(IOracleRegistryEvents::ActivatedOracleFilter(decoded));
            }
            if let Ok(decoded) = AddedOracleFilter::decode_log(log) {
                return Ok(IOracleRegistryEvents::AddedOracleFilter(decoded));
            }
            if let Ok(decoded) = DeactivatedOracleFilter::decode_log(log) {
                return Ok(IOracleRegistryEvents::DeactivatedOracleFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IOracleRegistryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOracleRegistryEvents::ActivatedOracleFilter(element) => element.fmt(f),
                IOracleRegistryEvents::AddedOracleFilter(element) => element.fmt(f),
                IOracleRegistryEvents::DeactivatedOracleFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `activateOracle`function with signature `activateOracle(address)` and selector `[122, 138, 1, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "activateOracle", abi = "activateOracle(address)")]
    pub struct ActivateOracleCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addOracle`function with signature `addOracle(address)` and selector `[223, 93, 209, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addOracle", abi = "addOracle(address)")]
    pub struct AddOracleCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `deactivateOracle`function with signature `deactivateOracle(address)` and selector `[237, 105, 198, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deactivateOracle", abi = "deactivateOracle(address)")]
    pub struct DeactivateOracleCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getOracleId`function with signature `getOracleId(address)` and selector `[188, 19, 135, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getOracleId", abi = "getOracleId(address)")]
    pub struct GetOracleIdCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getOraclesLength`function with signature `getOraclesLength()` and selector `[227, 8, 178, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getOraclesLength", abi = "getOraclesLength()")]
    pub struct GetOraclesLengthCall;
    #[doc = "Container type for all input parameters for the `isOracleActive`function with signature `isOracleActive(address)` and selector `[5, 242, 157, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isOracleActive", abi = "isOracleActive(address)")]
    pub struct IsOracleActiveCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isOracleRegistered`function with signature `isOracleRegistered(address)` and selector `[185, 181, 92, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isOracleRegistered", abi = "isOracleRegistered(address)")]
    pub struct IsOracleRegisteredCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `oracleInfo`function with signature `oracleInfo(address)` and selector `[188, 98, 63, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracleInfo", abi = "oracleInfo(address)")]
    pub struct OracleInfoCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `oracles`function with signature `oracles(uint256)` and selector `[91, 105, 167, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracles", abi = "oracles(uint256)")]
    pub struct OraclesCall(pub ethers::core::types::U256);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IOracleRegistryCalls {
        ActivateOracle(ActivateOracleCall),
        AddOracle(AddOracleCall),
        DeactivateOracle(DeactivateOracleCall),
        GetOracleId(GetOracleIdCall),
        GetOraclesLength(GetOraclesLengthCall),
        IsOracleActive(IsOracleActiveCall),
        IsOracleRegistered(IsOracleRegisteredCall),
        OracleInfo(OracleInfoCall),
        Oracles(OraclesCall),
    }
    impl ethers::core::abi::AbiDecode for IOracleRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ActivateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleRegistryCalls::ActivateOracle(decoded));
            }
            if let Ok(decoded) =
                <AddOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleRegistryCalls::AddOracle(decoded));
            }
            if let Ok(decoded) =
                <DeactivateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleRegistryCalls::DeactivateOracle(decoded));
            }
            if let Ok(decoded) =
                <GetOracleIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleRegistryCalls::GetOracleId(decoded));
            }
            if let Ok(decoded) =
                <GetOraclesLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleRegistryCalls::GetOraclesLength(decoded));
            }
            if let Ok(decoded) =
                <IsOracleActiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleRegistryCalls::IsOracleActive(decoded));
            }
            if let Ok(decoded) =
                <IsOracleRegisteredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleRegistryCalls::IsOracleRegistered(decoded));
            }
            if let Ok(decoded) =
                <OracleInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleRegistryCalls::OracleInfo(decoded));
            }
            if let Ok(decoded) =
                <OraclesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOracleRegistryCalls::Oracles(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IOracleRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IOracleRegistryCalls::ActivateOracle(element) => element.encode(),
                IOracleRegistryCalls::AddOracle(element) => element.encode(),
                IOracleRegistryCalls::DeactivateOracle(element) => element.encode(),
                IOracleRegistryCalls::GetOracleId(element) => element.encode(),
                IOracleRegistryCalls::GetOraclesLength(element) => element.encode(),
                IOracleRegistryCalls::IsOracleActive(element) => element.encode(),
                IOracleRegistryCalls::IsOracleRegistered(element) => element.encode(),
                IOracleRegistryCalls::OracleInfo(element) => element.encode(),
                IOracleRegistryCalls::Oracles(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IOracleRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOracleRegistryCalls::ActivateOracle(element) => element.fmt(f),
                IOracleRegistryCalls::AddOracle(element) => element.fmt(f),
                IOracleRegistryCalls::DeactivateOracle(element) => element.fmt(f),
                IOracleRegistryCalls::GetOracleId(element) => element.fmt(f),
                IOracleRegistryCalls::GetOraclesLength(element) => element.fmt(f),
                IOracleRegistryCalls::IsOracleActive(element) => element.fmt(f),
                IOracleRegistryCalls::IsOracleRegistered(element) => element.fmt(f),
                IOracleRegistryCalls::OracleInfo(element) => element.fmt(f),
                IOracleRegistryCalls::Oracles(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ActivateOracleCall> for IOracleRegistryCalls {
        fn from(var: ActivateOracleCall) -> Self {
            IOracleRegistryCalls::ActivateOracle(var)
        }
    }
    impl ::std::convert::From<AddOracleCall> for IOracleRegistryCalls {
        fn from(var: AddOracleCall) -> Self {
            IOracleRegistryCalls::AddOracle(var)
        }
    }
    impl ::std::convert::From<DeactivateOracleCall> for IOracleRegistryCalls {
        fn from(var: DeactivateOracleCall) -> Self {
            IOracleRegistryCalls::DeactivateOracle(var)
        }
    }
    impl ::std::convert::From<GetOracleIdCall> for IOracleRegistryCalls {
        fn from(var: GetOracleIdCall) -> Self {
            IOracleRegistryCalls::GetOracleId(var)
        }
    }
    impl ::std::convert::From<GetOraclesLengthCall> for IOracleRegistryCalls {
        fn from(var: GetOraclesLengthCall) -> Self {
            IOracleRegistryCalls::GetOraclesLength(var)
        }
    }
    impl ::std::convert::From<IsOracleActiveCall> for IOracleRegistryCalls {
        fn from(var: IsOracleActiveCall) -> Self {
            IOracleRegistryCalls::IsOracleActive(var)
        }
    }
    impl ::std::convert::From<IsOracleRegisteredCall> for IOracleRegistryCalls {
        fn from(var: IsOracleRegisteredCall) -> Self {
            IOracleRegistryCalls::IsOracleRegistered(var)
        }
    }
    impl ::std::convert::From<OracleInfoCall> for IOracleRegistryCalls {
        fn from(var: OracleInfoCall) -> Self {
            IOracleRegistryCalls::OracleInfo(var)
        }
    }
    impl ::std::convert::From<OraclesCall> for IOracleRegistryCalls {
        fn from(var: OraclesCall) -> Self {
            IOracleRegistryCalls::Oracles(var)
        }
    }
}
