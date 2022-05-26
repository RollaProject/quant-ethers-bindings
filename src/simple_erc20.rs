pub use simpleerc20_mod::*;
#[allow(clippy::too_many_arguments)]
mod simpleerc20_mod {
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
    #[doc = "SimpleERC20 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SIMPLEERC20_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static SIMPLEERC20_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052346101745761045d8038038061001981610190565b92833981016040828203126101745781516001600160401b03919082811161017457816100479185016101c2565b92602091828201518481116101745761006092016101c2565b918351908111610167575b6000916100818261007c855461024b565b610285565b80601f83116001146100df575081906100b49584926100d4575b50508160011b916000199060031b1c191617905561032f565b6100c6601260ff196002541617600255565b604051603a90816104238239f35b01519050388061009b565b600080529194601f1986167f290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e5639385905b82821061014f5750509160019391876100b4989410610136575b505050811b01905561032f565b015160001960f88460031b161c19169055388080610129565b8060018697829497870151815501960194019061010f565b61016f610179565b61006b565b600080fd5b50634e487b7160e01b600052604160045260246000fd5b6040519190601f01601f191682016001600160401b038111838210176101b557604052565b6101bd610179565b604052565b81601f82011215610174578051906001600160401b03821161023e575b6020906101f4601f8401601f19168301610190565b938385528284830101116101745782906000905b838383106102265750501161021c57505090565b6000918301015290565b81935082819392010151828288010152018391610208565b610246610179565b6101df565b90600182811c9216801561027b575b602083101461026557565b634e487b7160e01b600052602260045260246000fd5b91607f169161025a565b601f8111610291575050565b60009081805260208220906020601f850160051c830194106102ce575b601f0160051c01915b8281106102c357505050565b8181556001016102b7565b90925082906102ae565b90601f82116102e5575050565b60019160009083825260208220906020601f850160051c83019410610325575b601f0160051c01915b82811061031b5750505050565b818155830161030e565b9092508290610305565b80519091906001600160401b038111610415575b60019061035981610354845461024b565b6102d8565b602080601f8311600114610394575081929394600092610389575b5050600019600383901b1c191690821b179055565b015190503880610374565b6001600052601f198316959091907fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf6926000905b8882106103fe57505083859697106103e5575b505050811b019055565b015160001960f88460031b161c191690553880806103db565b8087859682949686015181550195019301906103c8565b61041d610179565b61034356fe600080fdfea264697066735822122043794df9990db67cd71594afdb9a92311b7f2024b51998aa39e6ee037aea14d364736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct SimpleERC20<M>(ethers::contract::Contract<M>);
    impl<M> Clone for SimpleERC20<M> {
        fn clone(&self) -> Self {
            SimpleERC20(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for SimpleERC20<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for SimpleERC20<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SimpleERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> SimpleERC20<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), SIMPLEERC20_ABI.clone(), client).into()
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
                SIMPLEERC20_ABI.clone(),
                SIMPLEERC20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for SimpleERC20<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
}
