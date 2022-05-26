pub use customerrors_mod::*;
#[allow(clippy::too_many_arguments)]
mod customerrors_mod {
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
    #[doc = "CustomErrors was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CUSTOMERRORS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| serde_json::from_str("[]").expect("invalid abi"));
    pub struct CustomErrors<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CustomErrors<M> {
        fn clone(&self) -> Self {
            CustomErrors(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CustomErrors<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CustomErrors<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CustomErrors))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CustomErrors<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CUSTOMERRORS_ABI.clone(), client).into()
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CustomErrors<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
}
