pub use simple_erc20::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod simple_erc20 {
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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static SIMPLEERC20_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052346102f5576103ac80380380610019816102fa565b9283398101906040818303126102f55780516001600160401b03908181116102f5578361004791840161031f565b91602093848201518381116102f557610060920161031f565b8251908282116102df5760008054926001958685811c951680156102d5575b888610146102c1578190601f95868111610273575b508890868311600114610214578492610209575b5050600019600383901b1c191690861b1781555b81519384116101f55784548581811c911680156101eb575b878210146101d757838111610194575b5085928411600114610134578394955092610129575b5050600019600383901b1c191690821b1790555b601260ff1960025416176002556040516011908161039b8239f35b0151905038806100fa565b9190601f1984169585845280842093905b87821061017d57505083859610610164575b505050811b01905561010e565b015160001960f88460031b161c19169055388080610157565b808785968294968601518155019501930190610145565b8582528682208480870160051c8201928988106101ce575b0160051c019086905b8281106101c35750506100e4565b8381550186906101b5565b925081926101ac565b634e487b7160e01b82526022600452602482fd5b90607f16906100d4565b634e487b7160e01b81526041600452602490fd5b0151905038806100a8565b8480528985208994509190601f198416865b8c82821061025d5750508411610244575b505050811b0181556100bc565b015160001960f88460031b161c19169055388080610237565b8385015186558c97909501949384019301610226565b9091508380528884208680850160051c8201928b86106102b8575b918a91869594930160051c01915b8281106102aa575050610094565b8681558594508a910161029c565b9250819261028e565b634e487b7160e01b83526022600452602483fd5b94607f169461007f565b634e487b7160e01b600052604160045260246000fd5b600080fd5b6040519190601f01601f191682016001600160401b038111838210176102df57604052565b81601f820112156102f5578051906001600160401b0382116102df57602090610350601f8401601f191683016102fa565b938385528284830101116102f55782906000905b838383106103825750501161037857505090565b6000918301015290565b8193508281939201015182828801015201839161036456fe600080fdfea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
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
