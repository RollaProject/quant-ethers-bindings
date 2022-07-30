pub use oracle_registry::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod oracle_registry {
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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ActivatedOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint248\",\"name\":\"oracleId\",\"type\":\"uint248\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddedOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DeactivatedOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"activateOracle\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addOracle\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deactivateOracle\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOracleId\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOraclesLength\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOracleActive\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOracleRegistered\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleInfo\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isActive\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint248\",\"name\":\"oracleId\",\"type\":\"uint248\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracles\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ORACLEREGISTRY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461005b5760008054336001600160a01b0319821681178355916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09084a3610c7f90816100618239f35b600080fdfe6080604081815260048036101561001557600080fd5b600092833560e01c90816305f29d0f14610ad9575080635b69a7d814610a57578063715018a6146109b85780637a8a0147146108665780638da5cb5b14610815578063b9b55c9a146107ad578063bc13872a146106b7578063bc623feb14610647578063df5dd1a5146103a1578063e308b2751461033e578063ed69c604146101e65763f2fde38b146100a757600080fd5b346101e25760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e25780359073ffffffffffffffffffffffffffffffffffffffff908183168093036101de57610101610bf3565b821561015b5750835492827fffffffffffffffffffffffff000000000000000000000000000000000000000085161785555192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08484a3f35b60849060208551917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b8480fd5b8280fd5b5091903461033a57602092837ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e25780359073ffffffffffffffffffffffffffffffffffffffff821680920361033657610243610bf3565b8184526001855260ff8385205416156102b457507fa01a58f2b7a077c4648592a2fc9c01cfed5a7541ec58e0f0d9e8caf036b2063e848351838152a18252600183528082207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00815416905551908152f35b608490858451917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602d60248201527f4f7261636c6552656769737472793a204f7261636c6520697320616c7265616460448201527f79206465616374697661746564000000000000000000000000000000000000006064820152fd5b8380fd5b5080fd5b50503461033a57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261033a576020907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff610399610b40565b915191168152f35b5091903461033a57602092837ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e257803573ffffffffffffffffffffffffffffffffffffffff8116809103610336576103fd610bf3565b808452600185528284205460081c6105c5576002546801000000000000000081101561059957600181018060025581101561056d57600285527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace01817fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055610488610b40565b937fc821da941aa41c0cfcd005d3f23c324fe5252dcb884a86b64e36ac7f84075fe5847effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81519785895216968789820152a18351928484019084821067ffffffffffffffff8311176105415750847fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff009260ff92825280865288860194888652815260018952209351151516915160081b1617905551908152f35b8260416024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b6024856032857f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b6024856041857f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b508360849251917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603160248201527f4f7261636c6552656769737472793a204f7261636c6520616c7265616479206560448201527f786973747320696e2072656769737472790000000000000000000000000000006064820152fd5b50346101e25760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e2573573ffffffffffffffffffffffffffffffffffffffff81168091036101e25791819281526001602052205481519060ff81161515825260081c6020820152f35b5082346107aa5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126107aa57813573ffffffffffffffffffffffffffffffffffffffff811680910361033a57815260016020528290205460081c908115610727575060209151908152f35b60849060208451917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603060248201527f4f7261636c6552656769737472793a204f7261636c6520646f65736e2774206560448201527f7869737420696e207265676973747279000000000000000000000000000000006064820152fd5b80fd5b50346101e25760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e2573573ffffffffffffffffffffffffffffffffffffffff81168091036101e2578282916020945260018452205460081c15159051908152f35b50503461033a57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261033a5773ffffffffffffffffffffffffffffffffffffffff60209254169051908152f35b5091903461033a57602091827ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126107aa57833573ffffffffffffffffffffffffffffffffffffffff811680910361033a576108c2610bf3565b8082526001845260ff8383205416610936579082917f40266c55de14e097b98dc2e8da50210e4d81330aae93762f266844369e760df7858451838152a18152600184522060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008254161790555160018152f35b608485858551917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602b60248201527f4f7261636c6552656769737472793a204f7261636c6520697320616c7265616460448201527f79206163746976617465640000000000000000000000000000000000000000006064820152fd5b50503461033a57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261033a576109f0610bf3565b8173ffffffffffffffffffffffffffffffffffffffff8154927fffffffffffffffffffffffff0000000000000000000000000000000000000000841683555192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b50346101e25760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101e257356002548110156101e25773ffffffffffffffffffffffffffffffffffffffff906002602094527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace0154169051908152f35b92919050346103365760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610336573573ffffffffffffffffffffffffffffffffffffffff81168091036103365783526001602090815292205460ff1615158152f35b6002547effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff90818111610b6f571690565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f7261636c6552656769737472793a206f7261636c6573206c696d697420657860448201527f63656564656400000000000000000000000000000000000000000000000000006064820152fd5b73ffffffffffffffffffffffffffffffffffffffff600054163303610c1457565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fdfea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
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
    #[doc = "Container type for all input parameters for the `activateOracle` function with signature `activateOracle(address)` and selector `[122, 138, 1, 71]`"]
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
    #[doc = "Container type for all input parameters for the `addOracle` function with signature `addOracle(address)` and selector `[223, 93, 209, 165]`"]
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
    #[doc = "Container type for all input parameters for the `deactivateOracle` function with signature `deactivateOracle(address)` and selector `[237, 105, 198, 4]`"]
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
    #[doc = "Container type for all input parameters for the `getOracleId` function with signature `getOracleId(address)` and selector `[188, 19, 135, 42]`"]
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
    #[doc = "Container type for all input parameters for the `getOraclesLength` function with signature `getOraclesLength()` and selector `[227, 8, 178, 117]`"]
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
    #[doc = "Container type for all input parameters for the `isOracleActive` function with signature `isOracleActive(address)` and selector `[5, 242, 157, 15]`"]
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
    #[doc = "Container type for all input parameters for the `isOracleRegistered` function with signature `isOracleRegistered(address)` and selector `[185, 181, 92, 154]`"]
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
    #[doc = "Container type for all input parameters for the `oracleInfo` function with signature `oracleInfo(address)` and selector `[188, 98, 63, 235]`"]
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
    #[doc = "Container type for all input parameters for the `oracles` function with signature `oracles(uint256)` and selector `[91, 105, 167, 216]`"]
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
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
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
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
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
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
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
    #[doc = "Container type for all return fields from the `activateOracle` function with signature `activateOracle(address)` and selector `[122, 138, 1, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ActivateOracleReturn(pub bool);
    #[doc = "Container type for all return fields from the `addOracle` function with signature `addOracle(address)` and selector `[223, 93, 209, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddOracleReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `deactivateOracle` function with signature `deactivateOracle(address)` and selector `[237, 105, 198, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DeactivateOracleReturn(pub bool);
    #[doc = "Container type for all return fields from the `getOracleId` function with signature `getOracleId(address)` and selector `[188, 19, 135, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetOracleIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getOraclesLength` function with signature `getOraclesLength()` and selector `[227, 8, 178, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetOraclesLengthReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isOracleActive` function with signature `isOracleActive(address)` and selector `[5, 242, 157, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsOracleActiveReturn(pub bool);
    #[doc = "Container type for all return fields from the `isOracleRegistered` function with signature `isOracleRegistered(address)` and selector `[185, 181, 92, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsOracleRegisteredReturn(pub bool);
    #[doc = "Container type for all return fields from the `oracleInfo` function with signature `oracleInfo(address)` and selector `[188, 98, 63, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OracleInfoReturn {
        pub is_active: bool,
        pub oracle_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `oracles` function with signature `oracles(uint256)` and selector `[91, 105, 167, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OraclesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
}
