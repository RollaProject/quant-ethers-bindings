pub use signedconvertertester_mod::*;
#[allow(clippy::too_many_arguments)]
mod signedconvertertester_mod {
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
    #[doc = "SignedConverterTester was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SIGNEDCONVERTERTESTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"a\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"fromIntTest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"fromUintTest\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static SIGNEDCONVERTERTESTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60808060405234610016576101da908161001c8239f35b600080fdfe608080604052600436101561001357600080fd5b60003560e01c90816385ba2da11461003e575063c27b828b14610036575b600080fd5b610031610100565b346100315760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261003157600435907f80000000000000000000000000000000000000000000000000000000000000008210156100a457604051828152602090f35b807f08c379a0000000000000000000000000000000000000000000000000000000006064925260206004820152601b60248201527f5175616e744d6174683a206f7574206f6620696e742072616e676500000000006044820152fd5b50346100315760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610031576004356000811261014657602090604051908152f35b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5175616e744d6174683a206e6567617469766520696e740000000000000000006044820152fdfea2646970667358221220e5f6b681aea407643d8699864455f1705b2b408a310789b5fffc2dfe8f9b0cf864736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct SignedConverterTester<M>(ethers::contract::Contract<M>);
    impl<M> Clone for SignedConverterTester<M> {
        fn clone(&self) -> Self {
            SignedConverterTester(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for SignedConverterTester<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for SignedConverterTester<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SignedConverterTester))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> SignedConverterTester<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                SIGNEDCONVERTERTESTER_ABI.clone(),
                client,
            )
            .into()
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
                SIGNEDCONVERTERTESTER_ABI.clone(),
                SIGNEDCONVERTERTESTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `fromIntTest` (0xc27b828b) function"]
        pub fn from_int_test(
            &self,
            a: I256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([194, 123, 130, 139], a)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fromUintTest` (0x85ba2da1) function"]
        pub fn from_uint_test(
            &self,
            a: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([133, 186, 45, 161], a)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for SignedConverterTester<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `fromIntTest`function with signature `fromIntTest(int256)` and selector `[194, 123, 130, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fromIntTest", abi = "fromIntTest(int256)")]
    pub struct FromIntTestCall {
        pub a: I256,
    }
    #[doc = "Container type for all input parameters for the `fromUintTest`function with signature `fromUintTest(uint256)` and selector `[133, 186, 45, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fromUintTest", abi = "fromUintTest(uint256)")]
    pub struct FromUintTestCall {
        pub a: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SignedConverterTesterCalls {
        FromIntTest(FromIntTestCall),
        FromUintTest(FromUintTestCall),
    }
    impl ethers::core::abi::AbiDecode for SignedConverterTesterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FromIntTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SignedConverterTesterCalls::FromIntTest(decoded));
            }
            if let Ok(decoded) =
                <FromUintTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SignedConverterTesterCalls::FromUintTest(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for SignedConverterTesterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                SignedConverterTesterCalls::FromIntTest(element) => element.encode(),
                SignedConverterTesterCalls::FromUintTest(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for SignedConverterTesterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SignedConverterTesterCalls::FromIntTest(element) => element.fmt(f),
                SignedConverterTesterCalls::FromUintTest(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FromIntTestCall> for SignedConverterTesterCalls {
        fn from(var: FromIntTestCall) -> Self {
            SignedConverterTesterCalls::FromIntTest(var)
        }
    }
    impl ::std::convert::From<FromUintTestCall> for SignedConverterTesterCalls {
        fn from(var: FromUintTestCall) -> Self {
            SignedConverterTesterCalls::FromUintTest(var)
        }
    }
}
