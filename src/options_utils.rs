pub use optionsutils_mod::*;
#[allow(clippy::too_many_arguments)]
mod optionsutils_mod {
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
    #[doc = "OptionsUtils was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static OPTIONSUTILS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"SALT\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"STRIKE_PRICE_DECIMALS\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static OPTIONSUTILS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608080604052346100195760f6908161001f823930815050f35b600080fdfe6080806040526004361015601257600080fd5b600090813560e01c908163b9cc8f16146089575063ba9a91a514603457600080fd5b807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011260865760206040517f524f4c4c412e46494e414e4345000000000000000000000000000000000000008152f35b80fd5b9050817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011260bc5780601260209252f35b5080fdfea2646970667358221220594822aa59b383b4afa591bc413fd61209655069450e0259faf91345ced4b7c164736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct OptionsUtils<M>(ethers::contract::Contract<M>);
    impl<M> Clone for OptionsUtils<M> {
        fn clone(&self) -> Self {
            OptionsUtils(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for OptionsUtils<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for OptionsUtils<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(OptionsUtils))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> OptionsUtils<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), OPTIONSUTILS_ABI.clone(), client).into()
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
                OPTIONSUTILS_ABI.clone(),
                OPTIONSUTILS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `SALT` (0xba9a91a5) function"]
        pub fn salt(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([186, 154, 145, 165], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `STRIKE_PRICE_DECIMALS` (0xb9cc8f16) function"]
        pub fn strike_price_decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([185, 204, 143, 22], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for OptionsUtils<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `SALT`function with signature `SALT()` and selector `[186, 154, 145, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "SALT", abi = "SALT()")]
    pub struct SaltCall;
    #[doc = "Container type for all input parameters for the `STRIKE_PRICE_DECIMALS`function with signature `STRIKE_PRICE_DECIMALS()` and selector `[185, 204, 143, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "STRIKE_PRICE_DECIMALS", abi = "STRIKE_PRICE_DECIMALS()")]
    pub struct StrikePriceDecimalsCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum OptionsUtilsCalls {
        Salt(SaltCall),
        StrikePriceDecimals(StrikePriceDecimalsCall),
    }
    impl ethers::core::abi::AbiDecode for OptionsUtilsCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <SaltCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(OptionsUtilsCalls::Salt(decoded));
            }
            if let Ok(decoded) =
                <StrikePriceDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OptionsUtilsCalls::StrikePriceDecimals(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for OptionsUtilsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                OptionsUtilsCalls::Salt(element) => element.encode(),
                OptionsUtilsCalls::StrikePriceDecimals(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for OptionsUtilsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OptionsUtilsCalls::Salt(element) => element.fmt(f),
                OptionsUtilsCalls::StrikePriceDecimals(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<SaltCall> for OptionsUtilsCalls {
        fn from(var: SaltCall) -> Self {
            OptionsUtilsCalls::Salt(var)
        }
    }
    impl ::std::convert::From<StrikePriceDecimalsCall> for OptionsUtilsCalls {
        fn from(var: StrikePriceDecimalsCall) -> Self {
            OptionsUtilsCalls::StrikePriceDecimals(var)
        }
    }
}
