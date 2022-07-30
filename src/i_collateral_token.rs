pub use i_collateral_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_collateral_token {
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
    #[doc = "ICollateralToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICOLLATERALTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"qTokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qTokenAsCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralTokenCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralTokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnCollateralToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qTokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createOptionCollateralToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenAsCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createSpreadCollateralToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenAsCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getCollateralTokenId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"idToInfo\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"metaSetApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralTokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintCollateralToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"optionsFactory_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOptionsFactory\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct ICollateralToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ICollateralToken<M> {
        fn clone(&self) -> Self {
            ICollateralToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ICollateralToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICollateralToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICollateralToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ICollateralToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICOLLATERALTOKEN_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `burnCollateralToken` (0x3e029179) function"]
        pub fn burn_collateral_token(
            &self,
            owner: ethers::core::types::Address,
            collateral_token_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 2, 145, 121], (owner, collateral_token_id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createOptionCollateralToken` (0x1c82a559) function"]
        pub fn create_option_collateral_token(
            &self,
            q_token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 130, 165, 89], q_token_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createSpreadCollateralToken` (0xe1ad9221) function"]
        pub fn create_spread_collateral_token(
            &self,
            q_token_address: ethers::core::types::Address,
            q_token_as_collateral: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [225, 173, 146, 33],
                    (q_token_address, q_token_as_collateral),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCollateralTokenId` (0x6ed1ff6d) function"]
        pub fn get_collateral_token_id(
            &self,
            q_token: ethers::core::types::Address,
            q_token_as_collateral: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([110, 209, 255, 109], (q_token, q_token_as_collateral))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `idToInfo` (0xd94a7e53) function"]
        pub fn id_to_info(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, ethers::core::types::Address),
        > {
            self.0
                .method_hash([217, 74, 126, 83], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `metaSetApprovalForAll` (0xb4e60a32) function"]
        pub fn meta_set_approval_for_all(
            &self,
            owner: ethers::core::types::Address,
            operator: ethers::core::types::Address,
            approved: bool,
            nonce: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [180, 230, 10, 50],
                    (owner, operator, approved, nonce, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintCollateralToken` (0xe37f5ac7) function"]
        pub fn mint_collateral_token(
            &self,
            recipient: ethers::core::types::Address,
            collateral_token_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 127, 90, 199],
                    (recipient, collateral_token_id, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOptionsFactory` (0x47a95d48) function"]
        pub fn set_options_factory(
            &self,
            options_factory: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 169, 93, 72], options_factory)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CollateralTokenCreated` event"]
        pub fn collateral_token_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollateralTokenCreatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CollateralTokenCreatedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICollateralToken<M> {
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
        name = "CollateralTokenCreated",
        abi = "CollateralTokenCreated(address,address,uint256)"
    )]
    pub struct CollateralTokenCreatedFilter {
        #[ethevent(indexed)]
        pub q_token_address: ethers::core::types::Address,
        pub q_token_as_collateral: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `burnCollateralToken` function with signature `burnCollateralToken(address,uint256,uint256)` and selector `[62, 2, 145, 121]`"]
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
        name = "burnCollateralToken",
        abi = "burnCollateralToken(address,uint256,uint256)"
    )]
    pub struct BurnCollateralTokenCall {
        pub owner: ethers::core::types::Address,
        pub collateral_token_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `createOptionCollateralToken` function with signature `createOptionCollateralToken(address)` and selector `[28, 130, 165, 89]`"]
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
        name = "createOptionCollateralToken",
        abi = "createOptionCollateralToken(address)"
    )]
    pub struct CreateOptionCollateralTokenCall {
        pub q_token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `createSpreadCollateralToken` function with signature `createSpreadCollateralToken(address,address)` and selector `[225, 173, 146, 33]`"]
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
        name = "createSpreadCollateralToken",
        abi = "createSpreadCollateralToken(address,address)"
    )]
    pub struct CreateSpreadCollateralTokenCall {
        pub q_token_address: ethers::core::types::Address,
        pub q_token_as_collateral: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCollateralTokenId` function with signature `getCollateralTokenId(address,address)` and selector `[110, 209, 255, 109]`"]
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
        name = "getCollateralTokenId",
        abi = "getCollateralTokenId(address,address)"
    )]
    pub struct GetCollateralTokenIdCall {
        pub q_token: ethers::core::types::Address,
        pub q_token_as_collateral: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `idToInfo` function with signature `idToInfo(uint256)` and selector `[217, 74, 126, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "idToInfo", abi = "idToInfo(uint256)")]
    pub struct IdToInfoCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `metaSetApprovalForAll` function with signature `metaSetApprovalForAll(address,address,bool,uint256,uint256,uint8,bytes32,bytes32)` and selector `[180, 230, 10, 50]`"]
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
        name = "metaSetApprovalForAll",
        abi = "metaSetApprovalForAll(address,address,bool,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct MetaSetApprovalForAllCall {
        pub owner: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
        pub approved: bool,
        pub nonce: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `mintCollateralToken` function with signature `mintCollateralToken(address,uint256,uint256)` and selector `[227, 127, 90, 199]`"]
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
        name = "mintCollateralToken",
        abi = "mintCollateralToken(address,uint256,uint256)"
    )]
    pub struct MintCollateralTokenCall {
        pub recipient: ethers::core::types::Address,
        pub collateral_token_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setOptionsFactory` function with signature `setOptionsFactory(address)` and selector `[71, 169, 93, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setOptionsFactory", abi = "setOptionsFactory(address)")]
    pub struct SetOptionsFactoryCall {
        pub options_factory: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICollateralTokenCalls {
        BurnCollateralToken(BurnCollateralTokenCall),
        CreateOptionCollateralToken(CreateOptionCollateralTokenCall),
        CreateSpreadCollateralToken(CreateSpreadCollateralTokenCall),
        GetCollateralTokenId(GetCollateralTokenIdCall),
        IdToInfo(IdToInfoCall),
        MetaSetApprovalForAll(MetaSetApprovalForAllCall),
        MintCollateralToken(MintCollateralTokenCall),
        SetOptionsFactory(SetOptionsFactoryCall),
    }
    impl ethers::core::abi::AbiDecode for ICollateralTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BurnCollateralTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICollateralTokenCalls::BurnCollateralToken(decoded));
            }
            if let Ok(decoded) =
                <CreateOptionCollateralTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICollateralTokenCalls::CreateOptionCollateralToken(decoded));
            }
            if let Ok(decoded) =
                <CreateSpreadCollateralTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICollateralTokenCalls::CreateSpreadCollateralToken(decoded));
            }
            if let Ok(decoded) =
                <GetCollateralTokenIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICollateralTokenCalls::GetCollateralTokenId(decoded));
            }
            if let Ok(decoded) =
                <IdToInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICollateralTokenCalls::IdToInfo(decoded));
            }
            if let Ok(decoded) =
                <MetaSetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICollateralTokenCalls::MetaSetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <MintCollateralTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICollateralTokenCalls::MintCollateralToken(decoded));
            }
            if let Ok(decoded) =
                <SetOptionsFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICollateralTokenCalls::SetOptionsFactory(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICollateralTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICollateralTokenCalls::BurnCollateralToken(element) => element.encode(),
                ICollateralTokenCalls::CreateOptionCollateralToken(element) => element.encode(),
                ICollateralTokenCalls::CreateSpreadCollateralToken(element) => element.encode(),
                ICollateralTokenCalls::GetCollateralTokenId(element) => element.encode(),
                ICollateralTokenCalls::IdToInfo(element) => element.encode(),
                ICollateralTokenCalls::MetaSetApprovalForAll(element) => element.encode(),
                ICollateralTokenCalls::MintCollateralToken(element) => element.encode(),
                ICollateralTokenCalls::SetOptionsFactory(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICollateralTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICollateralTokenCalls::BurnCollateralToken(element) => element.fmt(f),
                ICollateralTokenCalls::CreateOptionCollateralToken(element) => element.fmt(f),
                ICollateralTokenCalls::CreateSpreadCollateralToken(element) => element.fmt(f),
                ICollateralTokenCalls::GetCollateralTokenId(element) => element.fmt(f),
                ICollateralTokenCalls::IdToInfo(element) => element.fmt(f),
                ICollateralTokenCalls::MetaSetApprovalForAll(element) => element.fmt(f),
                ICollateralTokenCalls::MintCollateralToken(element) => element.fmt(f),
                ICollateralTokenCalls::SetOptionsFactory(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BurnCollateralTokenCall> for ICollateralTokenCalls {
        fn from(var: BurnCollateralTokenCall) -> Self {
            ICollateralTokenCalls::BurnCollateralToken(var)
        }
    }
    impl ::std::convert::From<CreateOptionCollateralTokenCall> for ICollateralTokenCalls {
        fn from(var: CreateOptionCollateralTokenCall) -> Self {
            ICollateralTokenCalls::CreateOptionCollateralToken(var)
        }
    }
    impl ::std::convert::From<CreateSpreadCollateralTokenCall> for ICollateralTokenCalls {
        fn from(var: CreateSpreadCollateralTokenCall) -> Self {
            ICollateralTokenCalls::CreateSpreadCollateralToken(var)
        }
    }
    impl ::std::convert::From<GetCollateralTokenIdCall> for ICollateralTokenCalls {
        fn from(var: GetCollateralTokenIdCall) -> Self {
            ICollateralTokenCalls::GetCollateralTokenId(var)
        }
    }
    impl ::std::convert::From<IdToInfoCall> for ICollateralTokenCalls {
        fn from(var: IdToInfoCall) -> Self {
            ICollateralTokenCalls::IdToInfo(var)
        }
    }
    impl ::std::convert::From<MetaSetApprovalForAllCall> for ICollateralTokenCalls {
        fn from(var: MetaSetApprovalForAllCall) -> Self {
            ICollateralTokenCalls::MetaSetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<MintCollateralTokenCall> for ICollateralTokenCalls {
        fn from(var: MintCollateralTokenCall) -> Self {
            ICollateralTokenCalls::MintCollateralToken(var)
        }
    }
    impl ::std::convert::From<SetOptionsFactoryCall> for ICollateralTokenCalls {
        fn from(var: SetOptionsFactoryCall) -> Self {
            ICollateralTokenCalls::SetOptionsFactory(var)
        }
    }
    #[doc = "Container type for all return fields from the `createOptionCollateralToken` function with signature `createOptionCollateralToken(address)` and selector `[28, 130, 165, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreateOptionCollateralTokenReturn {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `createSpreadCollateralToken` function with signature `createSpreadCollateralToken(address,address)` and selector `[225, 173, 146, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreateSpreadCollateralTokenReturn {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getCollateralTokenId` function with signature `getCollateralTokenId(address,address)` and selector `[110, 209, 255, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCollateralTokenIdReturn {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `idToInfo` function with signature `idToInfo(uint256)` and selector `[217, 74, 126, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IdToInfoReturn(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
}
