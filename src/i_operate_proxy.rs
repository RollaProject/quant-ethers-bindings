pub use ioperateproxy_mod::*;
#[allow(clippy::too_many_arguments)]
mod ioperateproxy_mod {
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
    #[doc = "IOperateProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IOPERATEPROXY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"originalSender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FunctionCallExecuted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"callee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"callFunction\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IOperateProxy<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IOperateProxy<M> {
        fn clone(&self) -> Self {
            IOperateProxy(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOperateProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IOperateProxy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOperateProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IOperateProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IOPERATEPROXY_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `callFunction` (0x9c23da50) function"]
        pub fn call_function(
            &self,
            callee: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 35, 218, 80], (callee, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `FunctionCallExecuted` event"]
        pub fn function_call_executed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FunctionCallExecutedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, FunctionCallExecutedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IOperateProxy<M> {
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
        name = "FunctionCallExecuted",
        abi = "FunctionCallExecuted(address,bytes)"
    )]
    pub struct FunctionCallExecutedFilter {
        #[ethevent(indexed)]
        pub original_sender: ethers::core::types::Address,
        pub return_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `callFunction`function with signature `callFunction(address,bytes)` and selector `[156, 35, 218, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "callFunction", abi = "callFunction(address,bytes)")]
    pub struct CallFunctionCall {
        pub callee: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
}
