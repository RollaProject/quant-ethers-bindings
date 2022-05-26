pub use priceregistry_mod::*;
#[allow(clippy::too_many_arguments)]
mod priceregistry_mod {
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
    #[doc = "PriceRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PRICEREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"strikeAssetDecimals_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracleRegistry\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"_settlementPriceDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_settlementPrice\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceStored\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOptionPriceStatus\",\"outputs\":[{\"internalType\":\"enum PriceStatus\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSettlementPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSettlementPriceWithDecimals\",\"outputs\":[{\"internalType\":\"struct PriceWithDecimals\",\"name\":\"settlementPrice\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasSettlementPrice\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_settlementPriceDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_settlementPrice\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSettlementPrice\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PRICEREGISTRY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60c03461008f57601f610e9c38819003918201601f19168301916001600160401b0383118484101761009457808492604094855283398101031261008f5780519060ff8216820361008f57602001516001600160a01b038116810361008f57610067916100aa565b604051610d7e908161011e823960805181610154015260a05181818161028d01526104800152f35b600080fd5b634e487b7160e01b600052604160045260246000fd5b6001600160a01b038216156100c15760805260a052565b60405162461bcd60e51b815260206004820152602e60248201527f507269636552656769737472793a20696e76616c6964206f7261636c6520726560448201526d676973747279206164647265737360901b6064820152608490fdfe60806040526004361015610013575b600080fd5b6000803560e01c90816326fea5721461008e575080634bb93ab114610085578063818182261461007c5780638df6475d14610073578063eae65f081461006a5763f8ef0be61461006257600080fd5b61000e610697565b5061000e6103bc565b5061000e610362565b5061000e6102b1565b5061000e610241565b3461018e5761017a61015061011173ffffffffffffffffffffffffffffffffffffffff6100f561018a9560406100c3366101ce565b939195909516815280602052209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b906affffffffffffffffffffff16600052602052604060002090565b61014a60ff604051610122816106bb565b61014082600186549687855201541694602083019586521515610938565b5192511660ff1690565b90610abf565b60ff7f00000000000000000000000000000000000000000000000000000000000000001690610b5e565b6040519081529081906020820190565b0390f35b80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b602435906affffffffffffffffffffff8216820361000e57565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc606091011261000e5773ffffffffffffffffffffffffffffffffffffffff600435818116810361000e57916024356affffffffffffffffffffff8116810361000e5791604435908116810361000e5790565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461000e57604061032773ffffffffffffffffffffffffffffffffffffffff6100f56102dd366101ce565b916000602088969396516102f0816106bb565b8281520152166000526000602052846000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b60ff825191610335836106bb565b61035382600183549384875201541691602085019283521515610938565b83519251835251166020820152f35b503461000e5761037a610374366101ce565b91610cbc565b604051600382101561038d576020918152f35b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b503461000e5760807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576103f4610191565b6103fc6101b4565b9060443560ff8116810361000e576040517fb9b55c9a000000000000000000000000000000000000000000000000000000008152336004820152606435927f604702976861305fb540afd1bd3c79f3e44e5d1e63c8a4181a9aa3edbb1aaf479173ffffffffffffffffffffffffffffffffffffffff91906105c6906020906104c6907f000000000000000000000000000000000000000000000000000000000000000086168382602481845afa91821561068a575b60009261066b575b50816105e1575b50610788565b610527610520896100f5866104fb3373ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b9073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b5415610813565b61058d6affffffffffffffffffffff891698610545428b111561089e565b61055f88610551610929565b948b865285019060ff169052565b6100f5856104fb3373ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b6020600160ff928451815501920151167fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00825416179055565b6040805160ff959095168552602085019590955216923392a4005b6040517f05f29d0f00000000000000000000000000000000000000000000000000000000815233600482015291508390829060249082905afa90811561065e575b600091610631575b50386104c0565b6106519150833d8511610657575b6106498183610722565b810190610763565b3861062a565b503d61063f565b61066661077b565b610622565b610683919250843d8611610657576106498183610722565b90386104b9565b61069261077b565b6104b1565b503461000e5760206106b16106ab366101ce565b91610cf2565b6040519015158152f35b6040810190811067ffffffffffffffff8211176106d757604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6020810190811067ffffffffffffffff8211176106d757604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176106d757604052565b9081602091031261000e5751801515810361000e5790565b506040513d6000823e3d90fd5b1561078f57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603660248201527f507269636552656769737472793a205072696365207375626d6974746572206960448201527f73206e6f7420616e20616374697665206f7261636c65000000000000000000006064820152fd5b1561081a57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603460248201527f507269636552656769737472793a20536574746c656d656e742070726963652060448201527f68617320616c7265616479206265656e207365740000000000000000000000006064820152fd5b156108a557565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603960248201527f507269636552656769737472793a2043616e277420736574206120707269636560448201527f20666f7220612074696d6520696e2074686520667574757265000000000000006064820152fd5b60405190610936826106bb565b565b1561093f57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602f60248201527f507269636552656769737472793a204e6f20736574746c656d656e742070726960448201527f636520686173206265656e2073657400000000000000000000000000000000006064820152fd5b507f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe590601b8110610a21570190565b610a296109c3565b0190565b604d8111610a3c575b600a0a90565b610a446109c3565b610a36565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04821181151516610a7a570290565b610a826109c3565b0290565b8115610a90570490565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b906000604051610ace81610706565b526000604051610add81610706565b52601b8103610b035750610af090610bca565b60405190610afd82610706565b81525b90565b601b811115610b2f57610af091610b24610b1f610b2a936109f3565b610a2d565b90610a86565b610bca565b610af091610b4b82610b2a93601b10610b51575b601b03610a2d565b90610a49565b610b596109c3565b610b43565b90601b8103610b725750610b009051610c53565b601b811115610b9b5790610b4b610b95610b8e610b00946109f3565b9251610c53565b91610a2d565b610b24610bb4610b009383601b10610bbd575b51610c53565b91601b03610a2d565b610bc56109c3565b610bae565b7f8000000000000000000000000000000000000000000000000000000000000000811015610bf55790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f5175616e744d6174683a206f7574206f6620696e742072616e676500000000006044820152fd5b60008112610c5e5790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5175616e744d6174683a206e6567617469766520696e740000000000000000006044820152fd5b91906affffffffffffffffffffff8116421115610cea57610cdc92610cf2565b610ce557600190565b600290565b505050600090565b610d429273ffffffffffffffffffffffffffffffffffffffff6100f59216600052600060205260406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b5415159056fea264697066735822122083e5e00361b3409552b0ca4d17f20ca8cd1c2631ab2e84b95c5b5d067b2283be64736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct PriceRegistry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PriceRegistry<M> {
        fn clone(&self) -> Self {
            PriceRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PriceRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PriceRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PriceRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PriceRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PRICEREGISTRY_ABI.clone(), client)
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
                PRICEREGISTRY_ABI.clone(),
                PRICEREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getOptionPriceStatus` (0x8df6475d) function"]
        pub fn get_option_price_status(
            &self,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([141, 246, 71, 93], (oracle, expiry_time, asset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSettlementPrice` (0x26fea572) function"]
        pub fn get_settlement_price(
            &self,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([38, 254, 165, 114], (oracle, expiry_time, asset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSettlementPriceWithDecimals` (0x81818226) function"]
        pub fn get_settlement_price_with_decimals(
            &self,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, PriceWithDecimals> {
            self.0
                .method_hash([129, 129, 130, 38], (oracle, expiry_time, asset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasSettlementPrice` (0xf8ef0be6) function"]
        pub fn has_settlement_price(
            &self,
            oracle: ethers::core::types::Address,
            expiry_time: u128,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 239, 11, 230], (oracle, expiry_time, asset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracleRegistry` (0x4bb93ab1) function"]
        pub fn oracle_registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([75, 185, 58, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSettlementPrice` (0xeae65f08) function"]
        pub fn set_settlement_price(
            &self,
            asset: ethers::core::types::Address,
            expiry_time: u128,
            settlement_price_decimals: u8,
            settlement_price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [234, 230, 95, 8],
                    (
                        asset,
                        expiry_time,
                        settlement_price_decimals,
                        settlement_price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PriceStored` event"]
        pub fn price_stored_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PriceStoredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PriceStoredFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PriceRegistry<M> {
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
        name = "PriceStored",
        abi = "PriceStored(address,address,uint88,uint8,uint256)"
    )]
    pub struct PriceStoredFilter {
        #[ethevent(indexed)]
        pub oracle: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expiry_time: u128,
        pub settlement_price_decimals: u8,
        pub settlement_price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getOptionPriceStatus`function with signature `getOptionPriceStatus(address,uint88,address)` and selector `[141, 246, 71, 93]`"]
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
        name = "getOptionPriceStatus",
        abi = "getOptionPriceStatus(address,uint88,address)"
    )]
    pub struct GetOptionPriceStatusCall {
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getSettlementPrice`function with signature `getSettlementPrice(address,uint88,address)` and selector `[38, 254, 165, 114]`"]
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
        name = "getSettlementPrice",
        abi = "getSettlementPrice(address,uint88,address)"
    )]
    pub struct GetSettlementPriceCall {
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getSettlementPriceWithDecimals`function with signature `getSettlementPriceWithDecimals(address,uint88,address)` and selector `[129, 129, 130, 38]`"]
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
        name = "getSettlementPriceWithDecimals",
        abi = "getSettlementPriceWithDecimals(address,uint88,address)"
    )]
    pub struct GetSettlementPriceWithDecimalsCall {
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hasSettlementPrice`function with signature `hasSettlementPrice(address,uint88,address)` and selector `[248, 239, 11, 230]`"]
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
        name = "hasSettlementPrice",
        abi = "hasSettlementPrice(address,uint88,address)"
    )]
    pub struct HasSettlementPriceCall {
        pub oracle: ethers::core::types::Address,
        pub expiry_time: u128,
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `oracleRegistry`function with signature `oracleRegistry()` and selector `[75, 185, 58, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracleRegistry", abi = "oracleRegistry()")]
    pub struct OracleRegistryCall;
    #[doc = "Container type for all input parameters for the `setSettlementPrice`function with signature `setSettlementPrice(address,uint88,uint8,uint256)` and selector `[234, 230, 95, 8]`"]
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
        name = "setSettlementPrice",
        abi = "setSettlementPrice(address,uint88,uint8,uint256)"
    )]
    pub struct SetSettlementPriceCall {
        pub asset: ethers::core::types::Address,
        pub expiry_time: u128,
        pub settlement_price_decimals: u8,
        pub settlement_price: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PriceRegistryCalls {
        GetOptionPriceStatus(GetOptionPriceStatusCall),
        GetSettlementPrice(GetSettlementPriceCall),
        GetSettlementPriceWithDecimals(GetSettlementPriceWithDecimalsCall),
        HasSettlementPrice(HasSettlementPriceCall),
        OracleRegistry(OracleRegistryCall),
        SetSettlementPrice(SetSettlementPriceCall),
    }
    impl ethers::core::abi::AbiDecode for PriceRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetOptionPriceStatusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceRegistryCalls::GetOptionPriceStatus(decoded));
            }
            if let Ok(decoded) =
                <GetSettlementPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceRegistryCalls::GetSettlementPrice(decoded));
            }
            if let Ok(decoded) =
                <GetSettlementPriceWithDecimalsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PriceRegistryCalls::GetSettlementPriceWithDecimals(decoded));
            }
            if let Ok(decoded) =
                <HasSettlementPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceRegistryCalls::HasSettlementPrice(decoded));
            }
            if let Ok(decoded) =
                <OracleRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceRegistryCalls::OracleRegistry(decoded));
            }
            if let Ok(decoded) =
                <SetSettlementPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceRegistryCalls::SetSettlementPrice(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PriceRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PriceRegistryCalls::GetOptionPriceStatus(element) => element.encode(),
                PriceRegistryCalls::GetSettlementPrice(element) => element.encode(),
                PriceRegistryCalls::GetSettlementPriceWithDecimals(element) => element.encode(),
                PriceRegistryCalls::HasSettlementPrice(element) => element.encode(),
                PriceRegistryCalls::OracleRegistry(element) => element.encode(),
                PriceRegistryCalls::SetSettlementPrice(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PriceRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PriceRegistryCalls::GetOptionPriceStatus(element) => element.fmt(f),
                PriceRegistryCalls::GetSettlementPrice(element) => element.fmt(f),
                PriceRegistryCalls::GetSettlementPriceWithDecimals(element) => element.fmt(f),
                PriceRegistryCalls::HasSettlementPrice(element) => element.fmt(f),
                PriceRegistryCalls::OracleRegistry(element) => element.fmt(f),
                PriceRegistryCalls::SetSettlementPrice(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetOptionPriceStatusCall> for PriceRegistryCalls {
        fn from(var: GetOptionPriceStatusCall) -> Self {
            PriceRegistryCalls::GetOptionPriceStatus(var)
        }
    }
    impl ::std::convert::From<GetSettlementPriceCall> for PriceRegistryCalls {
        fn from(var: GetSettlementPriceCall) -> Self {
            PriceRegistryCalls::GetSettlementPrice(var)
        }
    }
    impl ::std::convert::From<GetSettlementPriceWithDecimalsCall> for PriceRegistryCalls {
        fn from(var: GetSettlementPriceWithDecimalsCall) -> Self {
            PriceRegistryCalls::GetSettlementPriceWithDecimals(var)
        }
    }
    impl ::std::convert::From<HasSettlementPriceCall> for PriceRegistryCalls {
        fn from(var: HasSettlementPriceCall) -> Self {
            PriceRegistryCalls::HasSettlementPrice(var)
        }
    }
    impl ::std::convert::From<OracleRegistryCall> for PriceRegistryCalls {
        fn from(var: OracleRegistryCall) -> Self {
            PriceRegistryCalls::OracleRegistry(var)
        }
    }
    impl ::std::convert::From<SetSettlementPriceCall> for PriceRegistryCalls {
        fn from(var: SetSettlementPriceCall) -> Self {
            PriceRegistryCalls::SetSettlementPrice(var)
        }
    }
}
