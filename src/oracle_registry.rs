pub use oracleregistry_mod::*;
#[allow(clippy::too_many_arguments)]
mod oracleregistry_mod {
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
    #[doc = "OracleRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ORACLEREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ActivatedOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint248\",\"name\":\"oracleId\",\"type\":\"uint248\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddedOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DeactivatedOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"activateOracle\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addOracle\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deactivateOracle\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOracleId\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclesLength\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOracleActive\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOracleRegistered\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleInfo\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isActive\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint248\",\"name\":\"oracleId\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracles\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ORACLEREGISTRY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461005b5760008054336001600160a01b0319821681178355916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09084a3610d4c90816100618239f35b600080fdfe60806040526004361015610013575b600080fd5b60003560e01c806305f29d0f146101035780635b69a7d8146100fa578063715018a6146100f15780637a8a0147146100e85780638da5cb5b146100df578063b9b55c9a146100d6578063bc13872a146100cd578063bc623feb146100c4578063df5dd1a5146100bb578063e308b275146100b2578063ed69c604146100a95763f2fde38b146100a157600080fd5b61000e6109c4565b5061000e610871565b5061000e61080e565b5061000e6105f0565b5061000e6105a7565b5061000e6104dc565b5061000e61049a565b5061000e610447565b5061000e6102c9565b5061000e610221565b5061000e61019b565b5061000e610159565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc602091011261000e5760043573ffffffffffffffffffffffffffffffffffffffff8116810361000e5790565b503461000e5773ffffffffffffffffffffffffffffffffffffffff61017d3661010c565b166000526001602052602060ff604060002054166040519015158152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043560025481101561000e5773ffffffffffffffffffffffffffffffffffffffff60209160026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace015416604051908152f35b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102c65780547fffffffffffffffffffffffff000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff821691610298338414610ad9565b16825581604051917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b80fd5b503461000e576102d83661010c565b73ffffffffffffffffffffffffffffffffffffffff6102fc81600054163314610ad9565b8116600052600160205260ff604060002054166103c35760405173ffffffffffffffffffffffffffffffffffffffff821681526103b391610388917f40266c55de14e097b98dc2e8da50210e4d81330aae93762f266844369e760df79080602081015b0390a173ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b60017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00825416179055565b60405160018152602090f35b0390f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602b60248201527f4f7261636c6552656769737472793a204f7261636c6520697320616c7265616460448201527f79206163746976617465640000000000000000000000000000000000000000006064820152fd5b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602073ffffffffffffffffffffffffffffffffffffffff60005416604051908152f35b503461000e5773ffffffffffffffffffffffffffffffffffffffff6104be3661010c565b166000526001602052602060406000205460081c1515604051908152f35b503461000e5773ffffffffffffffffffffffffffffffffffffffff6105003661010c565b16600052600160205260406000205460081c801561052357602090604051908152f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603060248201527f4f7261636c6552656769737472793a204f7261636c6520646f65736e2774206560448201527f7869737420696e207265676973747279000000000000000000000000000000006064820152fd5b503461000e5773ffffffffffffffffffffffffffffffffffffffff6105cb3661010c565b1660005260016020526040806000205481519060ff81161515825260081c6020820152f35b503461000e576105ff3661010c565b73ffffffffffffffffffffffffffffffffffffffff61062381600054163314610ad9565b8116600052600160205260406000205460081c61078a57806106476103bf92610b6e565b610758610652610c63565b6040805173ffffffffffffffffffffffffffffffffffffffff851681527effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831660208201529193917fc821da941aa41c0cfcd005d3f23c324fe5252dcb884a86b64e36ac7f84075fe59190a161071a6106c8610c36565b600081527effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff851660208201529173ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b815160209092015160081b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660ff92151592909216919091179055565b6040517effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff90911681529081906020820190565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603160248201527f4f7261636c6552656769737472793a204f7261636c6520616c7265616479206560448201527f786973747320696e2072656769737472790000000000000000000000000000006064820152fd5b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610848610c63565b7effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60405191168152f35b503461000e576108803661010c565b73ffffffffffffffffffffffffffffffffffffffff6108a481600054163314610ad9565b8116600052600160205260ff60406000205416156109405760405173ffffffffffffffffffffffffffffffffffffffff821681526109349161090c917fa01a58f2b7a077c4648592a2fc9c01cfed5a7541ec58e0f0d9e8caf036b2063e90806020810161035f565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008154169055565b60405160008152602090f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602d60248201527f4f7261636c6552656769737472793a204f7261636c6520697320616c7265616460448201527f79206465616374697661746564000000000000000000000000000000000000006064820152fd5b503461000e576109d33661010c565b6000549073ffffffffffffffffffffffffffffffffffffffff808316916109fb338414610ad9565b168015610a5557807fffffffffffffffffffffffff000000000000000000000000000000000000000060009416178355604051917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08484a3f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b15610ae057565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60025468010000000000000000811015610c29575b6001810180600255811015610bfa5773ffffffffffffffffffffffffffffffffffffffff9060026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace0191167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b610c31610b3e565b610b83565b604051906040820182811067ffffffffffffffff821117610c5657604052565b610c5e610b3e565b604052565b6002547effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff90818111610c92571690565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f7261636c6552656769737472793a206f7261636c6573206c696d697420657860448201527f63656564656400000000000000000000000000000000000000000000000000006064820152fdfea2646970667358221220a07447a058f485a43ac6814d401ec00cc21b4767ee89b46fda85e2e28c879f0364736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct OracleRegistry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for OracleRegistry<M> {
        fn clone(&self) -> Self {
            OracleRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for OracleRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for OracleRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(OracleRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> OracleRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ORACLEREGISTRY_ABI.clone(), client)
                .into()
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
                ORACLEREGISTRY_ABI.clone(),
                ORACLEREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `activateOracle` (0x7a8a0147) function"]
        pub fn activate_oracle(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([122, 138, 1, 71], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addOracle` (0xdf5dd1a5) function"]
        pub fn add_oracle(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([223, 93, 209, 165], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deactivateOracle` (0xed69c604) function"]
        pub fn deactivate_oracle(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 105, 198, 4], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOracleId` (0xbc13872a) function"]
        pub fn get_oracle_id(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([188, 19, 135, 42], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOraclesLength` (0xe308b275) function"]
        pub fn get_oracles_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([227, 8, 178, 117], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOracleActive` (0x05f29d0f) function"]
        pub fn is_oracle_active(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([5, 242, 157, 15], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOracleRegistered` (0xb9b55c9a) function"]
        pub fn is_oracle_registered(
            &self,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([185, 181, 92, 154], oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracleInfo` (0xbc623feb) function"]
        pub fn oracle_info(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::U256)>
        {
            self.0
                .method_hash([188, 98, 63, 235], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracles` (0x5b69a7d8) function"]
        pub fn oracles(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([91, 105, 167, 216], p0)
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
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ActivatedOracle` event"]
        pub fn activated_oracle_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ActivatedOracleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AddedOracle` event"]
        pub fn added_oracle_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddedOracleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DeactivatedOracle` event"]
        pub fn deactivated_oracle_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DeactivatedOracleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, OracleRegistryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for OracleRegistry<M> {
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
    #[ethevent(name = "ActivatedOracle", abi = "ActivatedOracle(address)")]
    pub struct ActivatedOracleFilter {
        pub oracle: ethers::core::types::Address,
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
    #[ethevent(name = "AddedOracle", abi = "AddedOracle(address,uint248)")]
    pub struct AddedOracleFilter {
        pub oracle: ethers::core::types::Address,
        pub oracle_id: ethers::core::types::U256,
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
    #[ethevent(name = "DeactivatedOracle", abi = "DeactivatedOracle(address)")]
    pub struct DeactivatedOracleFilter {
        pub oracle: ethers::core::types::Address,
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
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum OracleRegistryEvents {
        ActivatedOracleFilter(ActivatedOracleFilter),
        AddedOracleFilter(AddedOracleFilter),
        DeactivatedOracleFilter(DeactivatedOracleFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for OracleRegistryEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ActivatedOracleFilter::decode_log(log) {
                return Ok(OracleRegistryEvents::ActivatedOracleFilter(decoded));
            }
            if let Ok(decoded) = AddedOracleFilter::decode_log(log) {
                return Ok(OracleRegistryEvents::AddedOracleFilter(decoded));
            }
            if let Ok(decoded) = DeactivatedOracleFilter::decode_log(log) {
                return Ok(OracleRegistryEvents::DeactivatedOracleFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(OracleRegistryEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for OracleRegistryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OracleRegistryEvents::ActivatedOracleFilter(element) => element.fmt(f),
                OracleRegistryEvents::AddedOracleFilter(element) => element.fmt(f),
                OracleRegistryEvents::DeactivatedOracleFilter(element) => element.fmt(f),
                OracleRegistryEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `activateOracle`function with signature `activateOracle(address)` and selector `[122, 138, 1, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "activateOracle", abi = "activateOracle(address)")]
    pub struct ActivateOracleCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addOracle`function with signature `addOracle(address)` and selector `[223, 93, 209, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addOracle", abi = "addOracle(address)")]
    pub struct AddOracleCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `deactivateOracle`function with signature `deactivateOracle(address)` and selector `[237, 105, 198, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deactivateOracle", abi = "deactivateOracle(address)")]
    pub struct DeactivateOracleCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getOracleId`function with signature `getOracleId(address)` and selector `[188, 19, 135, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getOracleId", abi = "getOracleId(address)")]
    pub struct GetOracleIdCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getOraclesLength`function with signature `getOraclesLength()` and selector `[227, 8, 178, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getOraclesLength", abi = "getOraclesLength()")]
    pub struct GetOraclesLengthCall;
    #[doc = "Container type for all input parameters for the `isOracleActive`function with signature `isOracleActive(address)` and selector `[5, 242, 157, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isOracleActive", abi = "isOracleActive(address)")]
    pub struct IsOracleActiveCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isOracleRegistered`function with signature `isOracleRegistered(address)` and selector `[185, 181, 92, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isOracleRegistered", abi = "isOracleRegistered(address)")]
    pub struct IsOracleRegisteredCall {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `oracleInfo`function with signature `oracleInfo(address)` and selector `[188, 98, 63, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracleInfo", abi = "oracleInfo(address)")]
    pub struct OracleInfoCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `oracles`function with signature `oracles(uint256)` and selector `[91, 105, 167, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracles", abi = "oracles(uint256)")]
    pub struct OraclesCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `renounceOwnership`function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum OracleRegistryCalls {
        ActivateOracle(ActivateOracleCall),
        AddOracle(AddOracleCall),
        DeactivateOracle(DeactivateOracleCall),
        GetOracleId(GetOracleIdCall),
        GetOraclesLength(GetOraclesLengthCall),
        IsOracleActive(IsOracleActiveCall),
        IsOracleRegistered(IsOracleRegisteredCall),
        OracleInfo(OracleInfoCall),
        Oracles(OraclesCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for OracleRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ActivateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::ActivateOracle(decoded));
            }
            if let Ok(decoded) =
                <AddOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::AddOracle(decoded));
            }
            if let Ok(decoded) =
                <DeactivateOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::DeactivateOracle(decoded));
            }
            if let Ok(decoded) =
                <GetOracleIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::GetOracleId(decoded));
            }
            if let Ok(decoded) =
                <GetOraclesLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::GetOraclesLength(decoded));
            }
            if let Ok(decoded) =
                <IsOracleActiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::IsOracleActive(decoded));
            }
            if let Ok(decoded) =
                <IsOracleRegisteredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::IsOracleRegistered(decoded));
            }
            if let Ok(decoded) =
                <OracleInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::OracleInfo(decoded));
            }
            if let Ok(decoded) =
                <OraclesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::Oracles(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(OracleRegistryCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for OracleRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                OracleRegistryCalls::ActivateOracle(element) => element.encode(),
                OracleRegistryCalls::AddOracle(element) => element.encode(),
                OracleRegistryCalls::DeactivateOracle(element) => element.encode(),
                OracleRegistryCalls::GetOracleId(element) => element.encode(),
                OracleRegistryCalls::GetOraclesLength(element) => element.encode(),
                OracleRegistryCalls::IsOracleActive(element) => element.encode(),
                OracleRegistryCalls::IsOracleRegistered(element) => element.encode(),
                OracleRegistryCalls::OracleInfo(element) => element.encode(),
                OracleRegistryCalls::Oracles(element) => element.encode(),
                OracleRegistryCalls::Owner(element) => element.encode(),
                OracleRegistryCalls::RenounceOwnership(element) => element.encode(),
                OracleRegistryCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for OracleRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OracleRegistryCalls::ActivateOracle(element) => element.fmt(f),
                OracleRegistryCalls::AddOracle(element) => element.fmt(f),
                OracleRegistryCalls::DeactivateOracle(element) => element.fmt(f),
                OracleRegistryCalls::GetOracleId(element) => element.fmt(f),
                OracleRegistryCalls::GetOraclesLength(element) => element.fmt(f),
                OracleRegistryCalls::IsOracleActive(element) => element.fmt(f),
                OracleRegistryCalls::IsOracleRegistered(element) => element.fmt(f),
                OracleRegistryCalls::OracleInfo(element) => element.fmt(f),
                OracleRegistryCalls::Oracles(element) => element.fmt(f),
                OracleRegistryCalls::Owner(element) => element.fmt(f),
                OracleRegistryCalls::RenounceOwnership(element) => element.fmt(f),
                OracleRegistryCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ActivateOracleCall> for OracleRegistryCalls {
        fn from(var: ActivateOracleCall) -> Self {
            OracleRegistryCalls::ActivateOracle(var)
        }
    }
    impl ::std::convert::From<AddOracleCall> for OracleRegistryCalls {
        fn from(var: AddOracleCall) -> Self {
            OracleRegistryCalls::AddOracle(var)
        }
    }
    impl ::std::convert::From<DeactivateOracleCall> for OracleRegistryCalls {
        fn from(var: DeactivateOracleCall) -> Self {
            OracleRegistryCalls::DeactivateOracle(var)
        }
    }
    impl ::std::convert::From<GetOracleIdCall> for OracleRegistryCalls {
        fn from(var: GetOracleIdCall) -> Self {
            OracleRegistryCalls::GetOracleId(var)
        }
    }
    impl ::std::convert::From<GetOraclesLengthCall> for OracleRegistryCalls {
        fn from(var: GetOraclesLengthCall) -> Self {
            OracleRegistryCalls::GetOraclesLength(var)
        }
    }
    impl ::std::convert::From<IsOracleActiveCall> for OracleRegistryCalls {
        fn from(var: IsOracleActiveCall) -> Self {
            OracleRegistryCalls::IsOracleActive(var)
        }
    }
    impl ::std::convert::From<IsOracleRegisteredCall> for OracleRegistryCalls {
        fn from(var: IsOracleRegisteredCall) -> Self {
            OracleRegistryCalls::IsOracleRegistered(var)
        }
    }
    impl ::std::convert::From<OracleInfoCall> for OracleRegistryCalls {
        fn from(var: OracleInfoCall) -> Self {
            OracleRegistryCalls::OracleInfo(var)
        }
    }
    impl ::std::convert::From<OraclesCall> for OracleRegistryCalls {
        fn from(var: OraclesCall) -> Self {
            OracleRegistryCalls::Oracles(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for OracleRegistryCalls {
        fn from(var: OwnerCall) -> Self {
            OracleRegistryCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for OracleRegistryCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            OracleRegistryCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for OracleRegistryCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            OracleRegistryCalls::TransferOwnership(var)
        }
    }
}
