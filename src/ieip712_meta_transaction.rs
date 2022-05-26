pub use ieip712metatransaction_mod::*;
#[allow(clippy::too_many_arguments)]
mod ieip712metatransaction_mod {
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
    #[doc = "IEIP712MetaTransaction was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IEIP712METATRANSACTION_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeMetaTransaction\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IEIP712MetaTransaction<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IEIP712MetaTransaction<M> {
        fn clone(&self) -> Self {
            IEIP712MetaTransaction(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IEIP712MetaTransaction<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IEIP712MetaTransaction<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IEIP712MetaTransaction))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IEIP712MetaTransaction<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IEIP712METATRANSACTION_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `executeMetaTransaction` (0xde6fe4f7) function"]
        pub fn execute_meta_transaction(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Bytes,
            p2: ethers::core::types::U256,
            p3: [u8; 32],
            p4: [u8; 32],
            p5: u8,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([222, 111, 228, 247], (p0, p1, p2, p3, p4, p5))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNonce` (0x2d0335ab) function"]
        pub fn get_nonce(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([45, 3, 53, 171], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IEIP712MetaTransaction<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `executeMetaTransaction`function with signature `executeMetaTransaction(address,bytes,uint256,bytes32,bytes32,uint8)` and selector `[222, 111, 228, 247]`"]
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
        name = "executeMetaTransaction",
        abi = "executeMetaTransaction(address,bytes,uint256,bytes32,bytes32,uint8)"
    )]
    pub struct ExecuteMetaTransactionCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Bytes,
        pub ethers::core::types::U256,
        pub [u8; 32],
        pub [u8; 32],
        pub u8,
    );
    #[doc = "Container type for all input parameters for the `getNonce`function with signature `getNonce(address)` and selector `[45, 3, 53, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IEIP712MetaTransactionCalls {
        ExecuteMetaTransaction(ExecuteMetaTransactionCall),
        GetNonce(GetNonceCall),
    }
    impl ethers::core::abi::AbiDecode for IEIP712MetaTransactionCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ExecuteMetaTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEIP712MetaTransactionCalls::ExecuteMetaTransaction(decoded));
            }
            if let Ok(decoded) =
                <GetNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEIP712MetaTransactionCalls::GetNonce(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IEIP712MetaTransactionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IEIP712MetaTransactionCalls::ExecuteMetaTransaction(element) => element.encode(),
                IEIP712MetaTransactionCalls::GetNonce(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IEIP712MetaTransactionCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IEIP712MetaTransactionCalls::ExecuteMetaTransaction(element) => element.fmt(f),
                IEIP712MetaTransactionCalls::GetNonce(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ExecuteMetaTransactionCall> for IEIP712MetaTransactionCalls {
        fn from(var: ExecuteMetaTransactionCall) -> Self {
            IEIP712MetaTransactionCalls::ExecuteMetaTransaction(var)
        }
    }
    impl ::std::convert::From<GetNonceCall> for IEIP712MetaTransactionCalls {
        fn from(var: GetNonceCall) -> Self {
            IEIP712MetaTransactionCalls::GetNonce(var)
        }
    }
}
