pub use ipriceregistry_mod::*;
#[allow(clippy::too_many_arguments)]
mod ipriceregistry_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IPriceRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPRICEREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"_settlementPriceDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_settlementPrice\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceStored\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOptionPriceStatus\",\"outputs\":[{\"internalType\":\"enum PriceStatus\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSettlementPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSettlementPriceWithDecimals\",\"outputs\":[{\"internalType\":\"struct PriceWithDecimals\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasSettlementPrice\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_settlementPriceDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_settlementPrice\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSettlementPrice\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IPriceRegistry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPriceRegistry<M> {
        fn clone(&self) -> Self {
            IPriceRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPriceRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPriceRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPriceRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPriceRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPRICEREGISTRY_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getOptionPriceStatus` (0x8df6475d) function"]
        pub fn get_option_price_status(
            &self,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([141, 246, 71, 93], (oracle, expiry_time, asset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSettlementPrice` (0x26fea572) function"]
        pub fn get_settlement_price(
            &self,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([38, 254, 165, 114], (oracle, expiry_time, asset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSettlementPriceWithDecimals` (0x81818226) function"]
        pub fn get_settlement_price_with_decimals(
            &self,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, PriceWithDecimals> {
            self.0
                .method_hash([129, 129, 130, 38], (oracle, expiry_time, asset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasSettlementPrice` (0xf8ef0be6) function"]
        pub fn has_settlement_price(
            &self,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 239, 11, 230], (oracle, expiry_time, asset))
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
        #[doc = "Calls the contract's `setSettlementPrice` (0xeae65f08) function"]
        pub fn set_settlement_price(
            &self,
            asset: ethers::core::types::Address,
            expiry_time: u128,
            settlement_price_decimals: u8,
            settlement_price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [234, 230, 95, 8],
                    (
                        asset,
                        expiry_time,
                        settlement_price_decimals,
                        settlement_price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PriceStored` event"]
        pub fn price_stored_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PriceStoredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PriceStoredFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IPriceRegistry<M> {
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
        name = "PriceStored",
        abi = "PriceStored(address,address,uint88,uint8,uint256)"
    )]
    pub struct PriceStoredFilter {
        #[ethevent(indexed)]
        pub oracle: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expiry_time: u128,
        pub settlement_price_decimals: u8,
        pub settlement_price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getOptionPriceStatus`function with signature `getOptionPriceStatus(address,uint88,address)` and selector `[141, 246, 71, 93]`"]
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
        name = "getOptionPriceStatus",
        abi = "getOptionPriceStatus(address,uint88,address)"
    )]
    pub struct GetOptionPriceStatusCall {
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getSettlementPrice`function with signature `getSettlementPrice(address,uint88,address)` and selector `[38, 254, 165, 114]`"]
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
        name = "getSettlementPrice",
        abi = "getSettlementPrice(address,uint88,address)"
    )]
    pub struct GetSettlementPriceCall {
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getSettlementPriceWithDecimals`function with signature `getSettlementPriceWithDecimals(address,uint88,address)` and selector `[129, 129, 130, 38]`"]
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
        name = "getSettlementPriceWithDecimals",
        abi = "getSettlementPriceWithDecimals(address,uint88,address)"
    )]
    pub struct GetSettlementPriceWithDecimalsCall {
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hasSettlementPrice`function with signature `hasSettlementPrice(address,uint88,address)` and selector `[248, 239, 11, 230]`"]
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
        name = "hasSettlementPrice",
        abi = "hasSettlementPrice(address,uint88,address)"
    )]
    pub struct HasSettlementPriceCall {
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `oracleRegistry`function with signature `oracleRegistry()` and selector `[75, 185, 58, 177]`"]
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
    #[doc = "Container type for all input parameters for the `setSettlementPrice`function with signature `setSettlementPrice(address,uint88,uint8,uint256)` and selector `[234, 230, 95, 8]`"]
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
        name = "setSettlementPrice",
        abi = "setSettlementPrice(address,uint88,uint8,uint256)"
    )]
    pub struct SetSettlementPriceCall {
        pub asset: ethers::core::types::Address,
        pub expiry_time: u128,
        pub settlement_price_decimals: u8,
        pub settlement_price: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPriceRegistryCalls {
        GetOptionPriceStatus(GetOptionPriceStatusCall),
        GetSettlementPrice(GetSettlementPriceCall),
        GetSettlementPriceWithDecimals(GetSettlementPriceWithDecimalsCall),
        HasSettlementPrice(HasSettlementPriceCall),
        OracleRegistry(OracleRegistryCall),
        SetSettlementPrice(SetSettlementPriceCall),
    }
    impl ethers::core::abi::AbiDecode for IPriceRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetOptionPriceStatusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceRegistryCalls::GetOptionPriceStatus(decoded));
            }
            if let Ok(decoded) =
                <GetSettlementPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceRegistryCalls::GetSettlementPrice(decoded));
            }
            if let Ok(decoded) =
                <GetSettlementPriceWithDecimalsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPriceRegistryCalls::GetSettlementPriceWithDecimals(decoded));
            }
            if let Ok(decoded) =
                <HasSettlementPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceRegistryCalls::HasSettlementPrice(decoded));
            }
            if let Ok(decoded) =
                <OracleRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceRegistryCalls::OracleRegistry(decoded));
            }
            if let Ok(decoded) =
                <SetSettlementPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceRegistryCalls::SetSettlementPrice(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPriceRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPriceRegistryCalls::GetOptionPriceStatus(element) => element.encode(),
                IPriceRegistryCalls::GetSettlementPrice(element) => element.encode(),
                IPriceRegistryCalls::GetSettlementPriceWithDecimals(element) => element.encode(),
                IPriceRegistryCalls::HasSettlementPrice(element) => element.encode(),
                IPriceRegistryCalls::OracleRegistry(element) => element.encode(),
                IPriceRegistryCalls::SetSettlementPrice(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPriceRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPriceRegistryCalls::GetOptionPriceStatus(element) => element.fmt(f),
                IPriceRegistryCalls::GetSettlementPrice(element) => element.fmt(f),
                IPriceRegistryCalls::GetSettlementPriceWithDecimals(element) => element.fmt(f),
                IPriceRegistryCalls::HasSettlementPrice(element) => element.fmt(f),
                IPriceRegistryCalls::OracleRegistry(element) => element.fmt(f),
                IPriceRegistryCalls::SetSettlementPrice(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetOptionPriceStatusCall> for IPriceRegistryCalls {
        fn from(var: GetOptionPriceStatusCall) -> Self {
            IPriceRegistryCalls::GetOptionPriceStatus(var)
        }
    }
    impl ::std::convert::From<GetSettlementPriceCall> for IPriceRegistryCalls {
        fn from(var: GetSettlementPriceCall) -> Self {
            IPriceRegistryCalls::GetSettlementPrice(var)
        }
    }
    impl ::std::convert::From<GetSettlementPriceWithDecimalsCall> for IPriceRegistryCalls {
        fn from(var: GetSettlementPriceWithDecimalsCall) -> Self {
            IPriceRegistryCalls::GetSettlementPriceWithDecimals(var)
        }
    }
    impl ::std::convert::From<HasSettlementPriceCall> for IPriceRegistryCalls {
        fn from(var: HasSettlementPriceCall) -> Self {
            IPriceRegistryCalls::HasSettlementPrice(var)
        }
    }
    impl ::std::convert::From<OracleRegistryCall> for IPriceRegistryCalls {
        fn from(var: OracleRegistryCall) -> Self {
            IPriceRegistryCalls::OracleRegistry(var)
        }
    }
    impl ::std::convert::From<SetSettlementPriceCall> for IPriceRegistryCalls {
        fn from(var: SetSettlementPriceCall) -> Self {
            IPriceRegistryCalls::SetSettlementPrice(var)
        }
    }
}
