pub use operate_proxy::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod operate_proxy {
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
    #[doc = "OperateProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static OPERATEPROXY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"originalSender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FunctionCallExecuted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"callee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"callFunction\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static OPERATEPROXY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001657610370908161001c8239f35b600080fdfe604060808152600436101561001357600080fd5b600090813560e01c639c23da501461002a57600080fd5b346102b257807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102b25760043573ffffffffffffffffffffffffffffffffffffffff811681036102aa576024359067ffffffffffffffff82116102ae57366023830112156102ae5781600401356100ac6100a782610329565b6102b6565b818152856020948583019336602482840101116102aa57806024889301863783010152823b156102275751859283929083905af13d1561021f573d906100f46100a783610329565b9182523d858484013e5b1561019c5790825181815282518093818484015286935b8285106101845750507fe4ba8ce34a410742c9618f9eab0d74ba704f3ed170bff84ffda0aef0969977799211610178575b83817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f329601168101030190a251f35b84848483010152610146565b84820181015185850188015293840193859250610115565b5060849151907f08c379a00000000000000000000000000000000000000000000000000000000082526004820152602360248201527f4f70657261746550726f78793a206c6f772d6c6576656c2063616c6c2066616960448201527f6c656400000000000000000000000000000000000000000000000000000000006064820152fd5b6060906100fe565b6084848651907f08c379a00000000000000000000000000000000000000000000000000000000082526004820152602660248201527f4f70657261746550726f78793a2063616c6c6565206973206e6f74206120636f60448201527f6e747261637400000000000000000000000000000000000000000000000000006064820152fd5b8280fd5b8380fd5b5080fd5b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f604051930116820182811067ffffffffffffffff8211176102fa57604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b67ffffffffffffffff81116102fa57601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0166020019056fea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
        });
    pub struct OperateProxy<M>(ethers::contract::Contract<M>);
    impl<M> Clone for OperateProxy<M> {
        fn clone(&self) -> Self {
            OperateProxy(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for OperateProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for OperateProxy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(OperateProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> OperateProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), OPERATEPROXY_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                OPERATEPROXY_ABI.clone(),
                OPERATEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for OperateProxy<M> {
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
    #[doc = "Container type for all input parameters for the `callFunction` function with signature `callFunction(address,bytes)` and selector `[156, 35, 218, 80]`"]
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
