pub use ieacaggregatorproxy_mod::*;
#[allow(clippy::too_many_arguments)]
mod ieacaggregatorproxy_mod {
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
    #[doc = "IEACAggregatorProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IEACAGGREGATORPROXY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"current\",\"type\":\"int256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AnswerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"startedBy\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRound\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accessController\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"aggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_aggregator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"confirmAggregator\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRound\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"phaseAggregators\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"phaseId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_aggregator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposeAggregator\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposedAggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposedGetRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposedLatestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_accessController\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setController\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IEACAggregatorProxy<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IEACAggregatorProxy<M> {
        fn clone(&self) -> Self {
            IEACAggregatorProxy(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IEACAggregatorProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IEACAggregatorProxy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IEACAggregatorProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IEACAggregatorProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IEACAGGREGATORPROXY_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `acceptOwnership` (0x79ba5097) function"]
        pub fn accept_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accessController` (0xbc43cbaf) function"]
        pub fn access_controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([188, 67, 203, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `aggregator` (0x245a7bfc) function"]
        pub fn aggregator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([36, 90, 123, 252], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `confirmAggregator` (0xa928c096) function"]
        pub fn confirm_aggregator(
            &self,
            aggregator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 40, 192, 150], aggregator)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `description` (0x7284e416) function"]
        pub fn description(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAnswer` (0xb5ab58dc) function"]
        pub fn get_answer(
            &self,
            round_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([181, 171, 88, 220], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoundData` (0x9a6fc8f5) function"]
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTimestamp` (0xb633620c) function"]
        pub fn get_timestamp(
            &self,
            round_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 51, 98, 12], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestAnswer` (0x50d25bcd) function"]
        pub fn latest_answer(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([80, 210, 91, 205], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRound` (0x668a0f02) function"]
        pub fn latest_round(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([102, 138, 15, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRoundData` (0xfeaf968c) function"]
        pub fn latest_round_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestTimestamp` (0x8205bf6a) function"]
        pub fn latest_timestamp(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([130, 5, 191, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `phaseAggregators` (0xc1597304) function"]
        pub fn phase_aggregators(
            &self,
            p0: u16,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([193, 89, 115, 4], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `phaseId` (0x58303b10) function"]
        pub fn phase_id(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([88, 48, 59, 16], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposeAggregator` (0xf8a2abd3) function"]
        pub fn propose_aggregator(
            &self,
            aggregator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 162, 171, 211], aggregator)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposedAggregator` (0xe8c4be30) function"]
        pub fn proposed_aggregator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([232, 196, 190, 48], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposedGetRoundData` (0x6001ac53) function"]
        pub fn proposed_get_round_data(
            &self,
            round_id: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([96, 1, 172, 83], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposedLatestRoundData` (0x8f6b4d91) function"]
        pub fn proposed_latest_round_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([143, 107, 77, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setController` (0x92eefe9b) function"]
        pub fn set_controller(
            &self,
            access_controller: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 238, 254, 155], access_controller)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AnswerUpdated` event"]
        pub fn answer_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AnswerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewRound` event"]
        pub fn new_round_filter(&self) -> ethers::contract::builders::Event<M, NewRoundFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferRequested` event"]
        pub fn ownership_transfer_requested_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferRequestedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IEACAggregatorProxyEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IEACAggregatorProxy<M>
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
    #[ethevent(name = "AnswerUpdated", abi = "AnswerUpdated(int256,uint256,uint256)")]
    pub struct AnswerUpdatedFilter {
        #[ethevent(indexed)]
        pub current: I256,
        #[ethevent(indexed)]
        pub round_id: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
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
    #[ethevent(name = "NewRound", abi = "NewRound(uint256,address,uint256)")]
    pub struct NewRoundFilter {
        #[ethevent(indexed)]
        pub round_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub started_by: ethers::core::types::Address,
        pub started_at: ethers::core::types::U256,
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
        name = "OwnershipTransferRequested",
        abi = "OwnershipTransferRequested(address,address)"
    )]
    pub struct OwnershipTransferRequestedFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IEACAggregatorProxyEvents {
        AnswerUpdatedFilter(AnswerUpdatedFilter),
        NewRoundFilter(NewRoundFilter),
        OwnershipTransferRequestedFilter(OwnershipTransferRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for IEACAggregatorProxyEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AnswerUpdatedFilter::decode_log(log) {
                return Ok(IEACAggregatorProxyEvents::AnswerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = NewRoundFilter::decode_log(log) {
                return Ok(IEACAggregatorProxyEvents::NewRoundFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferRequestedFilter::decode_log(log) {
                return Ok(IEACAggregatorProxyEvents::OwnershipTransferRequestedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(IEACAggregatorProxyEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IEACAggregatorProxyEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IEACAggregatorProxyEvents::AnswerUpdatedFilter(element) => element.fmt(f),
                IEACAggregatorProxyEvents::NewRoundFilter(element) => element.fmt(f),
                IEACAggregatorProxyEvents::OwnershipTransferRequestedFilter(element) => {
                    element.fmt(f)
                }
                IEACAggregatorProxyEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `acceptOwnership`function with signature `acceptOwnership()` and selector `[121, 186, 80, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    #[doc = "Container type for all input parameters for the `accessController`function with signature `accessController()` and selector `[188, 67, 203, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "accessController", abi = "accessController()")]
    pub struct AccessControllerCall;
    #[doc = "Container type for all input parameters for the `aggregator`function with signature `aggregator()` and selector `[36, 90, 123, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "aggregator", abi = "aggregator()")]
    pub struct AggregatorCall;
    #[doc = "Container type for all input parameters for the `confirmAggregator`function with signature `confirmAggregator(address)` and selector `[169, 40, 192, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "confirmAggregator", abi = "confirmAggregator(address)")]
    pub struct ConfirmAggregatorCall {
        pub aggregator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `description`function with signature `description()` and selector `[114, 132, 228, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    #[doc = "Container type for all input parameters for the `getAnswer`function with signature `getAnswer(uint256)` and selector `[181, 171, 88, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAnswer", abi = "getAnswer(uint256)")]
    pub struct GetAnswerCall {
        pub round_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRoundData`function with signature `getRoundData(uint80)` and selector `[154, 111, 200, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall {
        pub round_id: u128,
    }
    #[doc = "Container type for all input parameters for the `getTimestamp`function with signature `getTimestamp(uint256)` and selector `[182, 51, 98, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(uint256)")]
    pub struct GetTimestampCall {
        pub round_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `latestAnswer`function with signature `latestAnswer()` and selector `[80, 210, 91, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestAnswer", abi = "latestAnswer()")]
    pub struct LatestAnswerCall;
    #[doc = "Container type for all input parameters for the `latestRound`function with signature `latestRound()` and selector `[102, 138, 15, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestRound", abi = "latestRound()")]
    pub struct LatestRoundCall;
    #[doc = "Container type for all input parameters for the `latestRoundData`function with signature `latestRoundData()` and selector `[254, 175, 150, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    #[doc = "Container type for all input parameters for the `latestTimestamp`function with signature `latestTimestamp()` and selector `[130, 5, 191, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestTimestamp", abi = "latestTimestamp()")]
    pub struct LatestTimestampCall;
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `phaseAggregators`function with signature `phaseAggregators(uint16)` and selector `[193, 89, 115, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "phaseAggregators", abi = "phaseAggregators(uint16)")]
    pub struct PhaseAggregatorsCall(pub u16);
    #[doc = "Container type for all input parameters for the `phaseId`function with signature `phaseId()` and selector `[88, 48, 59, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "phaseId", abi = "phaseId()")]
    pub struct PhaseIdCall;
    #[doc = "Container type for all input parameters for the `proposeAggregator`function with signature `proposeAggregator(address)` and selector `[248, 162, 171, 211]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "proposeAggregator", abi = "proposeAggregator(address)")]
    pub struct ProposeAggregatorCall {
        pub aggregator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `proposedAggregator`function with signature `proposedAggregator()` and selector `[232, 196, 190, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "proposedAggregator", abi = "proposedAggregator()")]
    pub struct ProposedAggregatorCall;
    #[doc = "Container type for all input parameters for the `proposedGetRoundData`function with signature `proposedGetRoundData(uint80)` and selector `[96, 1, 172, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "proposedGetRoundData", abi = "proposedGetRoundData(uint80)")]
    pub struct ProposedGetRoundDataCall {
        pub round_id: u128,
    }
    #[doc = "Container type for all input parameters for the `proposedLatestRoundData`function with signature `proposedLatestRoundData()` and selector `[143, 107, 77, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "proposedLatestRoundData", abi = "proposedLatestRoundData()")]
    pub struct ProposedLatestRoundDataCall;
    #[doc = "Container type for all input parameters for the `setController`function with signature `setController(address)` and selector `[146, 238, 254, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setController", abi = "setController(address)")]
    pub struct SetControllerCall {
        pub access_controller: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub to: ethers::core::types::Address,
    }
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
    pub enum IEACAggregatorProxyCalls {
        AcceptOwnership(AcceptOwnershipCall),
        AccessController(AccessControllerCall),
        Aggregator(AggregatorCall),
        ConfirmAggregator(ConfirmAggregatorCall),
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetAnswer(GetAnswerCall),
        GetRoundData(GetRoundDataCall),
        GetTimestamp(GetTimestampCall),
        LatestAnswer(LatestAnswerCall),
        LatestRound(LatestRoundCall),
        LatestRoundData(LatestRoundDataCall),
        LatestTimestamp(LatestTimestampCall),
        Owner(OwnerCall),
        PhaseAggregators(PhaseAggregatorsCall),
        PhaseId(PhaseIdCall),
        ProposeAggregator(ProposeAggregatorCall),
        ProposedAggregator(ProposedAggregatorCall),
        ProposedGetRoundData(ProposedGetRoundDataCall),
        ProposedLatestRoundData(ProposedLatestRoundDataCall),
        SetController(SetControllerCall),
        TransferOwnership(TransferOwnershipCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for IEACAggregatorProxyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::AcceptOwnership(decoded));
            }
            if let Ok(decoded) =
                <AccessControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::AccessController(decoded));
            }
            if let Ok(decoded) =
                <AggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::Aggregator(decoded));
            }
            if let Ok(decoded) =
                <ConfirmAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::ConfirmAggregator(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DescriptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::Description(decoded));
            }
            if let Ok(decoded) =
                <GetAnswerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::GetAnswer(decoded));
            }
            if let Ok(decoded) =
                <GetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::GetRoundData(decoded));
            }
            if let Ok(decoded) =
                <GetTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::GetTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LatestAnswerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::LatestAnswer(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::LatestRound(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::LatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::LatestTimestamp(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PhaseAggregatorsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::PhaseAggregators(decoded));
            }
            if let Ok(decoded) =
                <PhaseIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::PhaseId(decoded));
            }
            if let Ok(decoded) =
                <ProposeAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::ProposeAggregator(decoded));
            }
            if let Ok(decoded) =
                <ProposedAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::ProposedAggregator(decoded));
            }
            if let Ok(decoded) =
                <ProposedGetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::ProposedGetRoundData(decoded));
            }
            if let Ok(decoded) =
                <ProposedLatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::ProposedLatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <SetControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::SetController(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IEACAggregatorProxyCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IEACAggregatorProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IEACAggregatorProxyCalls::AcceptOwnership(element) => element.encode(),
                IEACAggregatorProxyCalls::AccessController(element) => element.encode(),
                IEACAggregatorProxyCalls::Aggregator(element) => element.encode(),
                IEACAggregatorProxyCalls::ConfirmAggregator(element) => element.encode(),
                IEACAggregatorProxyCalls::Decimals(element) => element.encode(),
                IEACAggregatorProxyCalls::Description(element) => element.encode(),
                IEACAggregatorProxyCalls::GetAnswer(element) => element.encode(),
                IEACAggregatorProxyCalls::GetRoundData(element) => element.encode(),
                IEACAggregatorProxyCalls::GetTimestamp(element) => element.encode(),
                IEACAggregatorProxyCalls::LatestAnswer(element) => element.encode(),
                IEACAggregatorProxyCalls::LatestRound(element) => element.encode(),
                IEACAggregatorProxyCalls::LatestRoundData(element) => element.encode(),
                IEACAggregatorProxyCalls::LatestTimestamp(element) => element.encode(),
                IEACAggregatorProxyCalls::Owner(element) => element.encode(),
                IEACAggregatorProxyCalls::PhaseAggregators(element) => element.encode(),
                IEACAggregatorProxyCalls::PhaseId(element) => element.encode(),
                IEACAggregatorProxyCalls::ProposeAggregator(element) => element.encode(),
                IEACAggregatorProxyCalls::ProposedAggregator(element) => element.encode(),
                IEACAggregatorProxyCalls::ProposedGetRoundData(element) => element.encode(),
                IEACAggregatorProxyCalls::ProposedLatestRoundData(element) => element.encode(),
                IEACAggregatorProxyCalls::SetController(element) => element.encode(),
                IEACAggregatorProxyCalls::TransferOwnership(element) => element.encode(),
                IEACAggregatorProxyCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IEACAggregatorProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IEACAggregatorProxyCalls::AcceptOwnership(element) => element.fmt(f),
                IEACAggregatorProxyCalls::AccessController(element) => element.fmt(f),
                IEACAggregatorProxyCalls::Aggregator(element) => element.fmt(f),
                IEACAggregatorProxyCalls::ConfirmAggregator(element) => element.fmt(f),
                IEACAggregatorProxyCalls::Decimals(element) => element.fmt(f),
                IEACAggregatorProxyCalls::Description(element) => element.fmt(f),
                IEACAggregatorProxyCalls::GetAnswer(element) => element.fmt(f),
                IEACAggregatorProxyCalls::GetRoundData(element) => element.fmt(f),
                IEACAggregatorProxyCalls::GetTimestamp(element) => element.fmt(f),
                IEACAggregatorProxyCalls::LatestAnswer(element) => element.fmt(f),
                IEACAggregatorProxyCalls::LatestRound(element) => element.fmt(f),
                IEACAggregatorProxyCalls::LatestRoundData(element) => element.fmt(f),
                IEACAggregatorProxyCalls::LatestTimestamp(element) => element.fmt(f),
                IEACAggregatorProxyCalls::Owner(element) => element.fmt(f),
                IEACAggregatorProxyCalls::PhaseAggregators(element) => element.fmt(f),
                IEACAggregatorProxyCalls::PhaseId(element) => element.fmt(f),
                IEACAggregatorProxyCalls::ProposeAggregator(element) => element.fmt(f),
                IEACAggregatorProxyCalls::ProposedAggregator(element) => element.fmt(f),
                IEACAggregatorProxyCalls::ProposedGetRoundData(element) => element.fmt(f),
                IEACAggregatorProxyCalls::ProposedLatestRoundData(element) => element.fmt(f),
                IEACAggregatorProxyCalls::SetController(element) => element.fmt(f),
                IEACAggregatorProxyCalls::TransferOwnership(element) => element.fmt(f),
                IEACAggregatorProxyCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptOwnershipCall> for IEACAggregatorProxyCalls {
        fn from(var: AcceptOwnershipCall) -> Self {
            IEACAggregatorProxyCalls::AcceptOwnership(var)
        }
    }
    impl ::std::convert::From<AccessControllerCall> for IEACAggregatorProxyCalls {
        fn from(var: AccessControllerCall) -> Self {
            IEACAggregatorProxyCalls::AccessController(var)
        }
    }
    impl ::std::convert::From<AggregatorCall> for IEACAggregatorProxyCalls {
        fn from(var: AggregatorCall) -> Self {
            IEACAggregatorProxyCalls::Aggregator(var)
        }
    }
    impl ::std::convert::From<ConfirmAggregatorCall> for IEACAggregatorProxyCalls {
        fn from(var: ConfirmAggregatorCall) -> Self {
            IEACAggregatorProxyCalls::ConfirmAggregator(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for IEACAggregatorProxyCalls {
        fn from(var: DecimalsCall) -> Self {
            IEACAggregatorProxyCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DescriptionCall> for IEACAggregatorProxyCalls {
        fn from(var: DescriptionCall) -> Self {
            IEACAggregatorProxyCalls::Description(var)
        }
    }
    impl ::std::convert::From<GetAnswerCall> for IEACAggregatorProxyCalls {
        fn from(var: GetAnswerCall) -> Self {
            IEACAggregatorProxyCalls::GetAnswer(var)
        }
    }
    impl ::std::convert::From<GetRoundDataCall> for IEACAggregatorProxyCalls {
        fn from(var: GetRoundDataCall) -> Self {
            IEACAggregatorProxyCalls::GetRoundData(var)
        }
    }
    impl ::std::convert::From<GetTimestampCall> for IEACAggregatorProxyCalls {
        fn from(var: GetTimestampCall) -> Self {
            IEACAggregatorProxyCalls::GetTimestamp(var)
        }
    }
    impl ::std::convert::From<LatestAnswerCall> for IEACAggregatorProxyCalls {
        fn from(var: LatestAnswerCall) -> Self {
            IEACAggregatorProxyCalls::LatestAnswer(var)
        }
    }
    impl ::std::convert::From<LatestRoundCall> for IEACAggregatorProxyCalls {
        fn from(var: LatestRoundCall) -> Self {
            IEACAggregatorProxyCalls::LatestRound(var)
        }
    }
    impl ::std::convert::From<LatestRoundDataCall> for IEACAggregatorProxyCalls {
        fn from(var: LatestRoundDataCall) -> Self {
            IEACAggregatorProxyCalls::LatestRoundData(var)
        }
    }
    impl ::std::convert::From<LatestTimestampCall> for IEACAggregatorProxyCalls {
        fn from(var: LatestTimestampCall) -> Self {
            IEACAggregatorProxyCalls::LatestTimestamp(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for IEACAggregatorProxyCalls {
        fn from(var: OwnerCall) -> Self {
            IEACAggregatorProxyCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PhaseAggregatorsCall> for IEACAggregatorProxyCalls {
        fn from(var: PhaseAggregatorsCall) -> Self {
            IEACAggregatorProxyCalls::PhaseAggregators(var)
        }
    }
    impl ::std::convert::From<PhaseIdCall> for IEACAggregatorProxyCalls {
        fn from(var: PhaseIdCall) -> Self {
            IEACAggregatorProxyCalls::PhaseId(var)
        }
    }
    impl ::std::convert::From<ProposeAggregatorCall> for IEACAggregatorProxyCalls {
        fn from(var: ProposeAggregatorCall) -> Self {
            IEACAggregatorProxyCalls::ProposeAggregator(var)
        }
    }
    impl ::std::convert::From<ProposedAggregatorCall> for IEACAggregatorProxyCalls {
        fn from(var: ProposedAggregatorCall) -> Self {
            IEACAggregatorProxyCalls::ProposedAggregator(var)
        }
    }
    impl ::std::convert::From<ProposedGetRoundDataCall> for IEACAggregatorProxyCalls {
        fn from(var: ProposedGetRoundDataCall) -> Self {
            IEACAggregatorProxyCalls::ProposedGetRoundData(var)
        }
    }
    impl ::std::convert::From<ProposedLatestRoundDataCall> for IEACAggregatorProxyCalls {
        fn from(var: ProposedLatestRoundDataCall) -> Self {
            IEACAggregatorProxyCalls::ProposedLatestRoundData(var)
        }
    }
    impl ::std::convert::From<SetControllerCall> for IEACAggregatorProxyCalls {
        fn from(var: SetControllerCall) -> Self {
            IEACAggregatorProxyCalls::SetController(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for IEACAggregatorProxyCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            IEACAggregatorProxyCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<VersionCall> for IEACAggregatorProxyCalls {
        fn from(var: VersionCall) -> Self {
            IEACAggregatorProxyCalls::Version(var)
        }
    }
}
