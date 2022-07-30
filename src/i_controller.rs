pub use i_controller::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_controller {
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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"collateralTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountClaimed\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"collateralReturned\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amountNeutralized\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"collateralReclaimed\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"longTokenReturned\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NeutralizePosition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountExercised\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"payout\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"payoutAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OptionsExercised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mintedTo\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"optionsAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"collateralAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OptionsPositionMinted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qTokenToMint\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qTokenForCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"optionsAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"collateralAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SpreadMinted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_collateralTokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exercise\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintOptionsPosition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qTokenToMint\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenForCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintSpread\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_collateralTokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"neutralizePosition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ActionArgs[]\",\"name\":\"_actions\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"operate\",\"outputs\":[]}]") . expect ("invalid abi")
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
        #[doc = "Calls the contract's `claimCollateral` (0xbb8cebb5) function"]
        pub fn claim_collateral(
            &self,
            collateral_token_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 140, 235, 181], (collateral_token_id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exercise` (0xdd068ec1) function"]
        pub fn exercise(
            &self,
            q_token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 6, 142, 193], (q_token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintOptionsPosition` (0x43406026) function"]
        pub fn mint_options_position(
            &self,
            to: ethers::core::types::Address,
            q_token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 64, 96, 38], (to, q_token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintSpread` (0xa645a740) function"]
        pub fn mint_spread(
            &self,
            q_token_to_mint: ethers::core::types::Address,
            q_token_for_collateral: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [166, 69, 167, 64],
                    (q_token_to_mint, q_token_for_collateral, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `neutralizePosition` (0x34ae540b) function"]
        pub fn neutralize_position(
            &self,
            collateral_token_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 174, 84, 11], (collateral_token_id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `operate` (0x7b7bed54) function"]
        pub fn operate(
            &self,
            actions: ::std::vec::Vec<ActionArgs>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 123, 237, 84], actions)
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
    #[doc = "Container type for all input parameters for the `claimCollateral` function with signature `claimCollateral(uint256,uint256)` and selector `[187, 140, 235, 181]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimCollateral", abi = "claimCollateral(uint256,uint256)")]
    pub struct ClaimCollateralCall {
        pub collateral_token_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `exercise` function with signature `exercise(address,uint256)` and selector `[221, 6, 142, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exercise", abi = "exercise(address,uint256)")]
    pub struct ExerciseCall {
        pub q_token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mintOptionsPosition` function with signature `mintOptionsPosition(address,address,uint256)` and selector `[67, 64, 96, 38]`"]
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
        name = "mintOptionsPosition",
        abi = "mintOptionsPosition(address,address,uint256)"
    )]
    pub struct MintOptionsPositionCall {
        pub to: ethers::core::types::Address,
        pub q_token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mintSpread` function with signature `mintSpread(address,address,uint256)` and selector `[166, 69, 167, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mintSpread", abi = "mintSpread(address,address,uint256)")]
    pub struct MintSpreadCall {
        pub q_token_to_mint: ethers::core::types::Address,
        pub q_token_for_collateral: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `neutralizePosition` function with signature `neutralizePosition(uint256,uint256)` and selector `[52, 174, 84, 11]`"]
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
        name = "neutralizePosition",
        abi = "neutralizePosition(uint256,uint256)"
    )]
    pub struct NeutralizePositionCall {
        pub collateral_token_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `operate` function with signature `operate((uint8,address,address,address,uint256,uint256,bytes)[])` and selector `[123, 123, 237, 84]`"]
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IControllerCalls {
        ClaimCollateral(ClaimCollateralCall),
        Exercise(ExerciseCall),
        MintOptionsPosition(MintOptionsPositionCall),
        MintSpread(MintSpreadCall),
        NeutralizePosition(NeutralizePositionCall),
        Operate(OperateCall),
    }
    impl ethers::core::abi::AbiDecode for IControllerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ClaimCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IControllerCalls::ClaimCollateral(decoded));
            }
            if let Ok(decoded) =
                <ExerciseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IControllerCalls::Exercise(decoded));
            }
            if let Ok(decoded) =
                <MintOptionsPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IControllerCalls::MintOptionsPosition(decoded));
            }
            if let Ok(decoded) =
                <MintSpreadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IControllerCalls::MintSpread(decoded));
            }
            if let Ok(decoded) =
                <NeutralizePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IControllerCalls::NeutralizePosition(decoded));
            }
            if let Ok(decoded) =
                <OperateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IControllerCalls::Operate(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IControllerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IControllerCalls::ClaimCollateral(element) => element.encode(),
                IControllerCalls::Exercise(element) => element.encode(),
                IControllerCalls::MintOptionsPosition(element) => element.encode(),
                IControllerCalls::MintSpread(element) => element.encode(),
                IControllerCalls::NeutralizePosition(element) => element.encode(),
                IControllerCalls::Operate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IControllerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IControllerCalls::ClaimCollateral(element) => element.fmt(f),
                IControllerCalls::Exercise(element) => element.fmt(f),
                IControllerCalls::MintOptionsPosition(element) => element.fmt(f),
                IControllerCalls::MintSpread(element) => element.fmt(f),
                IControllerCalls::NeutralizePosition(element) => element.fmt(f),
                IControllerCalls::Operate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ClaimCollateralCall> for IControllerCalls {
        fn from(var: ClaimCollateralCall) -> Self {
            IControllerCalls::ClaimCollateral(var)
        }
    }
    impl ::std::convert::From<ExerciseCall> for IControllerCalls {
        fn from(var: ExerciseCall) -> Self {
            IControllerCalls::Exercise(var)
        }
    }
    impl ::std::convert::From<MintOptionsPositionCall> for IControllerCalls {
        fn from(var: MintOptionsPositionCall) -> Self {
            IControllerCalls::MintOptionsPosition(var)
        }
    }
    impl ::std::convert::From<MintSpreadCall> for IControllerCalls {
        fn from(var: MintSpreadCall) -> Self {
            IControllerCalls::MintSpread(var)
        }
    }
    impl ::std::convert::From<NeutralizePositionCall> for IControllerCalls {
        fn from(var: NeutralizePositionCall) -> Self {
            IControllerCalls::NeutralizePosition(var)
        }
    }
    impl ::std::convert::From<OperateCall> for IControllerCalls {
        fn from(var: OperateCall) -> Self {
            IControllerCalls::Operate(var)
        }
    }
}
