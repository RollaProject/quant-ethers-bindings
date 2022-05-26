pub use icontroller_mod::*;
#[allow(clippy::too_many_arguments)]
mod icontroller_mod {
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
    #[doc = "IController was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICONTROLLER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"collateralTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountClaimed\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"collateralReturned\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountNeutralized\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"collateralReclaimed\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"longTokenReturned\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NeutralizePosition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountExercised\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"payout\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"payoutAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OptionsExercised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mintedTo\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"optionsAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"collateralAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OptionsPositionMinted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qTokenToMint\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qTokenForCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"optionsAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"collateralAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SpreadMinted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct ActionArgs[]\",\"name\":\"_actions\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"operate\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"optionsFactory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"quantCalculator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IController<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IController<M> {
        fn clone(&self) -> Self {
            IController(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IController<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IController<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IController))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IController<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICONTROLLER_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `operate` (0x7b7bed54) function"]
        pub fn operate(
            &self,
            actions: ::std::vec::Vec<ActionArgs>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([123, 123, 237, 84], actions)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `optionsFactory` (0xe66ef2c4) function"]
        pub fn options_factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([230, 110, 242, 196], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quantCalculator` (0x4a34962e) function"]
        pub fn quant_calculator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([74, 52, 150, 46], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CollateralClaimed` event"]
        pub fn collateral_claimed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollateralClaimedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NeutralizePosition` event"]
        pub fn neutralize_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NeutralizePositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OptionsExercised` event"]
        pub fn options_exercised_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OptionsExercisedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OptionsPositionMinted` event"]
        pub fn options_position_minted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OptionsPositionMintedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SpreadMinted` event"]
        pub fn spread_minted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SpreadMintedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IControllerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IController<M> {
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
        name = "CollateralClaimed",
        abi = "CollateralClaimed(address,uint256,uint256,uint256,address)"
    )]
    pub struct CollateralClaimedFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub collateral_token_id: ethers::core::types::U256,
        pub amount_claimed: ethers::core::types::U256,
        pub collateral_returned: ethers::core::types::U256,
        pub collateral_asset: ethers::core::types::Address,
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
        name = "NeutralizePosition",
        abi = "NeutralizePosition(address,address,uint256,uint256,address,address)"
    )]
    pub struct NeutralizePositionFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub q_token: ethers::core::types::Address,
        pub amount_neutralized: ethers::core::types::U256,
        pub collateral_reclaimed: ethers::core::types::U256,
        pub collateral_asset: ethers::core::types::Address,
        pub long_token_returned: ethers::core::types::Address,
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
        name = "OptionsExercised",
        abi = "OptionsExercised(address,address,uint256,uint256,address)"
    )]
    pub struct OptionsExercisedFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub q_token: ethers::core::types::Address,
        pub amount_exercised: ethers::core::types::U256,
        pub payout: ethers::core::types::U256,
        pub payout_asset: ethers::core::types::Address,
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
        name = "OptionsPositionMinted",
        abi = "OptionsPositionMinted(address,address,address,uint256,address,uint256)"
    )]
    pub struct OptionsPositionMintedFilter {
        #[ethevent(indexed)]
        pub minted_to: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub minter: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub q_token: ethers::core::types::Address,
        pub options_amount: ethers::core::types::U256,
        pub collateral_asset: ethers::core::types::Address,
        pub collateral_amount: ethers::core::types::U256,
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
        name = "SpreadMinted",
        abi = "SpreadMinted(address,address,address,uint256,address,uint256)"
    )]
    pub struct SpreadMintedFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub q_token_to_mint: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub q_token_for_collateral: ethers::core::types::Address,
        pub options_amount: ethers::core::types::U256,
        pub collateral_asset: ethers::core::types::Address,
        pub collateral_amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IControllerEvents {
        CollateralClaimedFilter(CollateralClaimedFilter),
        NeutralizePositionFilter(NeutralizePositionFilter),
        OptionsExercisedFilter(OptionsExercisedFilter),
        OptionsPositionMintedFilter(OptionsPositionMintedFilter),
        SpreadMintedFilter(SpreadMintedFilter),
    }
    impl ethers::contract::EthLogDecode for IControllerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CollateralClaimedFilter::decode_log(log) {
                return Ok(IControllerEvents::CollateralClaimedFilter(decoded));
            }
            if let Ok(decoded) = NeutralizePositionFilter::decode_log(log) {
                return Ok(IControllerEvents::NeutralizePositionFilter(decoded));
            }
            if let Ok(decoded) = OptionsExercisedFilter::decode_log(log) {
                return Ok(IControllerEvents::OptionsExercisedFilter(decoded));
            }
            if let Ok(decoded) = OptionsPositionMintedFilter::decode_log(log) {
                return Ok(IControllerEvents::OptionsPositionMintedFilter(decoded));
            }
            if let Ok(decoded) = SpreadMintedFilter::decode_log(log) {
                return Ok(IControllerEvents::SpreadMintedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IControllerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IControllerEvents::CollateralClaimedFilter(element) => element.fmt(f),
                IControllerEvents::NeutralizePositionFilter(element) => element.fmt(f),
                IControllerEvents::OptionsExercisedFilter(element) => element.fmt(f),
                IControllerEvents::OptionsPositionMintedFilter(element) => element.fmt(f),
                IControllerEvents::SpreadMintedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `operate`function with signature `operate((uint8,address,address,address,uint256,uint256,bytes)[])` and selector `[123, 123, 237, 84]`"]
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
        name = "operate",
        abi = "operate((uint8,address,address,address,uint256,uint256,bytes)[])"
    )]
    pub struct OperateCall {
        pub actions: ::std::vec::Vec<ActionArgs>,
    }
    #[doc = "Container type for all input parameters for the `optionsFactory`function with signature `optionsFactory()` and selector `[230, 110, 242, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "optionsFactory", abi = "optionsFactory()")]
    pub struct OptionsFactoryCall;
    #[doc = "Container type for all input parameters for the `quantCalculator`function with signature `quantCalculator()` and selector `[74, 52, 150, 46]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "quantCalculator", abi = "quantCalculator()")]
    pub struct QuantCalculatorCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IControllerCalls {
        Operate(OperateCall),
        OptionsFactory(OptionsFactoryCall),
        QuantCalculator(QuantCalculatorCall),
    }
    impl ethers::core::abi::AbiDecode for IControllerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <OperateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IControllerCalls::Operate(decoded));
            }
            if let Ok(decoded) =
                <OptionsFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IControllerCalls::OptionsFactory(decoded));
            }
            if let Ok(decoded) =
                <QuantCalculatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IControllerCalls::QuantCalculator(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IControllerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IControllerCalls::Operate(element) => element.encode(),
                IControllerCalls::OptionsFactory(element) => element.encode(),
                IControllerCalls::QuantCalculator(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IControllerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IControllerCalls::Operate(element) => element.fmt(f),
                IControllerCalls::OptionsFactory(element) => element.fmt(f),
                IControllerCalls::QuantCalculator(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<OperateCall> for IControllerCalls {
        fn from(var: OperateCall) -> Self {
            IControllerCalls::Operate(var)
        }
    }
    impl ::std::convert::From<OptionsFactoryCall> for IControllerCalls {
        fn from(var: OptionsFactoryCall) -> Self {
            IControllerCalls::OptionsFactory(var)
        }
    }
    impl ::std::convert::From<QuantCalculatorCall> for IControllerCalls {
        fn from(var: QuantCalculatorCall) -> Self {
            IControllerCalls::QuantCalculator(var)
        }
    }
}
