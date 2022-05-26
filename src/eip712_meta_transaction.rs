pub use eip712metatransaction_mod::*;
#[allow(clippy::too_many_arguments)]
mod eip712metatransaction_mod {
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
    #[doc = "EIP712MetaTransaction was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static EIP712METATRANSACTION_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"userAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address payable\",\"name\":\"relayerAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MetaTransactionExecuted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct EIP712MetaTransaction.MetaAction\",\"name\":\"metaAction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct ActionArgs[]\",\"name\":\"actions\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeMetaTransaction\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct EIP712MetaTransaction<M>(ethers::contract::Contract<M>);
    impl<M> Clone for EIP712MetaTransaction<M> {
        fn clone(&self) -> Self {
            EIP712MetaTransaction(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for EIP712MetaTransaction<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for EIP712MetaTransaction<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(EIP712MetaTransaction))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> EIP712MetaTransaction<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                EIP712METATRANSACTION_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `executeMetaTransaction` (0x29c89c14) function"]
        pub fn execute_meta_transaction(
            &self,
            meta_action: MetaAction,
            gas_limit: ethers::core::types::U256,
            r: [u8; 32],
            s: [u8; 32],
            v: u8,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::Bytes)>
        {
            self.0
                .method_hash([41, 200, 156, 20], (meta_action, gas_limit, r, s, v))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNonce` (0x2d0335ab) function"]
        pub fn get_nonce(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([45, 3, 53, 171], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `MetaTransactionExecuted` event"]
        pub fn meta_transaction_executed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MetaTransactionExecutedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, MetaTransactionExecutedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for EIP712MetaTransaction<M>
    {
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
        name = "MetaTransactionExecuted",
        abi = "MetaTransactionExecuted(address,address,bool,uint256,bytes)"
    )]
    pub struct MetaTransactionExecutedFilter {
        #[ethevent(indexed)]
        pub user_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub relayer_address: ethers::core::types::Address,
        pub success: bool,
        pub nonce: ethers::core::types::U256,
        pub return_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `executeMetaTransaction`function with signature `executeMetaTransaction((uint256,uint256,address,(uint8,address,address,address,uint256,uint256,bytes)[]),uint256,bytes32,bytes32,uint8)` and selector `[41, 200, 156, 20]`"]
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
        abi = "executeMetaTransaction((uint256,uint256,address,(uint8,address,address,address,uint256,uint256,bytes)[]),uint256,bytes32,bytes32,uint8)"
    )]
    pub struct ExecuteMetaTransactionCall {
        pub meta_action: MetaAction,
        pub gas_limit: ethers::core::types::U256,
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub v: u8,
    }
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
    pub struct GetNonceCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `version`function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum EIP712MetaTransactionCalls {
        ExecuteMetaTransaction(ExecuteMetaTransactionCall),
        GetNonce(GetNonceCall),
        Name(NameCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for EIP712MetaTransactionCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ExecuteMetaTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EIP712MetaTransactionCalls::ExecuteMetaTransaction(decoded));
            }
            if let Ok(decoded) =
                <GetNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EIP712MetaTransactionCalls::GetNonce(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(EIP712MetaTransactionCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EIP712MetaTransactionCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for EIP712MetaTransactionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                EIP712MetaTransactionCalls::ExecuteMetaTransaction(element) => element.encode(),
                EIP712MetaTransactionCalls::GetNonce(element) => element.encode(),
                EIP712MetaTransactionCalls::Name(element) => element.encode(),
                EIP712MetaTransactionCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for EIP712MetaTransactionCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                EIP712MetaTransactionCalls::ExecuteMetaTransaction(element) => element.fmt(f),
                EIP712MetaTransactionCalls::GetNonce(element) => element.fmt(f),
                EIP712MetaTransactionCalls::Name(element) => element.fmt(f),
                EIP712MetaTransactionCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ExecuteMetaTransactionCall> for EIP712MetaTransactionCalls {
        fn from(var: ExecuteMetaTransactionCall) -> Self {
            EIP712MetaTransactionCalls::ExecuteMetaTransaction(var)
        }
    }
    impl ::std::convert::From<GetNonceCall> for EIP712MetaTransactionCalls {
        fn from(var: GetNonceCall) -> Self {
            EIP712MetaTransactionCalls::GetNonce(var)
        }
    }
    impl ::std::convert::From<NameCall> for EIP712MetaTransactionCalls {
        fn from(var: NameCall) -> Self {
            EIP712MetaTransactionCalls::Name(var)
        }
    }
    impl ::std::convert::From<VersionCall> for EIP712MetaTransactionCalls {
        fn from(var: VersionCall) -> Self {
            EIP712MetaTransactionCalls::Version(var)
        }
    }
}
