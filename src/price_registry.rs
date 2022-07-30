pub use price_registry::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod price_registry {
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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"strikeAssetDecimals_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracleRegistry\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"_settlementPriceDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_settlementPrice\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceStored\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOptionPriceStatus\",\"outputs\":[{\"internalType\":\"enum PriceStatus\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSettlementPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSettlementPriceWithDecimals\",\"outputs\":[{\"internalType\":\"struct PriceWithDecimals\",\"name\":\"settlementPrice\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasSettlementPrice\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_settlementPriceDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_settlementPrice\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSettlementPrice\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PRICEREGISTRY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60c0346100ef57601f610e6a38819003918201601f19168301916001600160401b038311848410176100f45780849260409485528339810103126100ef5780519060ff821682036100ef5760200151906001600160a01b0382168083036100ef57156100935760805260a052604051610d5f908161010b8239608051816107c5015260a0518181816101a701526106a50152f35b60405162461bcd60e51b815260206004820152602e60248201527f507269636552656769737472793a20696e76616c6964206f7261636c6520726560448201526d676973747279206164647265737360901b6064820152608490fd5b600080fd5b634e487b7160e01b600052604160045260246000fdfe608060408181526004918236101561001657600080fd5b600092833560e01c91826326fea572146106c9575081634bb93ab11461065a57816381818226146105805781638df6475d146104ee578163eae65f081461010d575063f8ef0be61461006757600080fd5b346101095760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610109576020906101006100a461098d565b6100ac6109d8565b6100b46109b5565b919060409273ffffffffffffffffffffffffffffffffffffffff8060009416845283602052848420911683526020526affffffffffffffffffffff838320911682526020522054151590565b90519015158152f35b5080fd5b9050346104ea5760807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104ea5761014661098d565b61014e6109d8565b916044359060ff82168092036104e65784517fb9b55c9a00000000000000000000000000000000000000000000000000000000815233818301526064359160209173ffffffffffffffffffffffffffffffffffffffff907f00000000000000000000000000000000000000000000000000000000000000008216908481602481855afa9182156104dc5785918c936104bd575b508261043c575b5050156103ba5733895288835287892095169485895282526affffffffffffffffffffff878920961695868952825286882054610339574286116102b857508551610232816109f2565b82815260ff600183830192868452338b528a8552898b20888c528552898b20898c528552898b2090518155019151167fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0082541617905585519283528201527f604702976861305fb540afd1bd3c79f3e44e5d1e63c8a4181a9aa3edbb1aaf47843392a451f35b6084918751917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603960248201527f507269636552656769737472793a2043616e277420736574206120707269636560448201527f20666f7220612074696d6520696e2074686520667574757265000000000000006064820152fd5b6084918751917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603460248201527f507269636552656769737472793a20536574746c656d656e742070726963652060448201527f68617320616c7265616479206265656e207365740000000000000000000000006064820152fd5b506084918751917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603660248201527f507269636552656769737472793a205072696365207375626d6974746572206960448201527f73206e6f7420616e20616374697665206f7261636c65000000000000000000006064820152fd5b602492508a51928380927f05f29d0f00000000000000000000000000000000000000000000000000000000825233888301525afa9081156104b3578a91610486575b5083386101e8565b6104a69150843d86116104ac575b61049e8183610a59565b810190610a9a565b3861047e565b503d610494565b89513d8c823e3d90fd5b6104d5919350823d84116104ac5761049e8183610a59565b91386101e1565b8a513d8d823e3d90fd5b8580fd5b8280fd5b8383346101095760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101095761054061052a61098d565b6105326109d8565b61053a6109b5565b91610cd6565b905191600382101561055457602083838152f35b806021857f4e487b71000000000000000000000000000000000000000000000000000000006024945252fd5b8284346106575760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261065757816105ba61098d565b916105c36109d8565b6105cb6109b5565b8351946105d7866109f2565b8386528360208097015273ffffffffffffffffffffffffffffffffffffffff80911684528386528484209116835284526affffffffffffffffffffff838320911682528352209060ff83519261062c846109f2565b610649826001835493848852015416918486019283521515610ab2565b845193518452511690820152f35b80fd5b50503461010957817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610109576020905173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b909150346109895760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126109895761070361098d565b61070b6109d8565b906107146109b5565b9073ffffffffffffffffffffffffffffffffffffffff809116875286602052858720911686526020526affffffffffffffffffffff8486209116855260205260ff83852091610762816109f2565b61078082600185549586855201541693602083019485521515610ab2565b519151169084845161079181610a3d565b5284845161079e81610a3d565b52601b918083036108ea57506107b390610be4565b8351906107bf82610a3d565b8152935b7f000000000000000000000000000000000000000000000000000000000000000060ff169180830361080757505050506107ff60209251610c6d565b905b51908152f35b808394969311600014610890578310610864575060209350906108587fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe561085161085e9451610c6d565b9201610b3d565b90610b7a565b90610801565b806011867f4e487b71000000000000000000000000000000000000000000000000000000006024945252fd5b9291908284106108be57506020945061085e92916108b16108b89251610c6d565b9203610b3d565b90610bab565b806011877f4e487b71000000000000000000000000000000000000000000000000000000006024945252fd5b828111156109705782811061094457610930916108b87fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe561092b9301610b3d565b610be4565b83519061093c82610a3d565b8152936107c3565b6024866011867f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b808310610944576109309161085861092b928503610b3d565b8380fd5b6004359073ffffffffffffffffffffffffffffffffffffffff821682036109b057565b600080fd5b6044359073ffffffffffffffffffffffffffffffffffffffff821682036109b057565b602435906affffffffffffffffffffff821682036109b057565b6040810190811067ffffffffffffffff821117610a0e57604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6020810190811067ffffffffffffffff821117610a0e57604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610a0e57604052565b908160209103126109b0575180151581036109b05790565b15610ab957565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602f60248201527f507269636552656769737472793a204e6f20736574746c656d656e742070726960448201527f636520686173206265656e2073657400000000000000000000000000000000006064820152fd5b604d8111610b4b57600a0a90565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04821181151516610b4b570290565b8115610bb5570490565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b7f8000000000000000000000000000000000000000000000000000000000000000811015610c0f5790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f5175616e744d6174683a206f7574206f6620696e742072616e676500000000006044820152fd5b60008112610c785790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5175616e744d6174683a206e6567617469766520696e740000000000000000006044820152fd5b91906affffffffffffffffffffff8116421115610d4a57610d3c929060409273ffffffffffffffffffffffffffffffffffffffff8060009416845283602052848420911683526020526affffffffffffffffffffff838320911682526020522054151590565b610d4557600190565b600290565b50505060009056fea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
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
    #[doc = "Container type for all input parameters for the `getOptionPriceStatus` function with signature `getOptionPriceStatus(address,uint88,address)` and selector `[141, 246, 71, 93]`"]
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
    #[doc = "Container type for all input parameters for the `getSettlementPrice` function with signature `getSettlementPrice(address,uint88,address)` and selector `[38, 254, 165, 114]`"]
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
    #[doc = "Container type for all input parameters for the `getSettlementPriceWithDecimals` function with signature `getSettlementPriceWithDecimals(address,uint88,address)` and selector `[129, 129, 130, 38]`"]
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
    #[doc = "Container type for all input parameters for the `hasSettlementPrice` function with signature `hasSettlementPrice(address,uint88,address)` and selector `[248, 239, 11, 230]`"]
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
    #[doc = "Container type for all input parameters for the `oracleRegistry` function with signature `oracleRegistry()` and selector `[75, 185, 58, 177]`"]
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
    #[doc = "Container type for all input parameters for the `setSettlementPrice` function with signature `setSettlementPrice(address,uint88,uint8,uint256)` and selector `[234, 230, 95, 8]`"]
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
    #[doc = "Container type for all return fields from the `getOptionPriceStatus` function with signature `getOptionPriceStatus(address,uint88,address)` and selector `[141, 246, 71, 93]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetOptionPriceStatusReturn(pub u8);
    #[doc = "Container type for all return fields from the `getSettlementPrice` function with signature `getSettlementPrice(address,uint88,address)` and selector `[38, 254, 165, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSettlementPriceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSettlementPriceWithDecimals` function with signature `getSettlementPriceWithDecimals(address,uint88,address)` and selector `[129, 129, 130, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSettlementPriceWithDecimalsReturn {
        pub settlement_price: (ethers::core::types::U256, u8),
    }
    #[doc = "Container type for all return fields from the `hasSettlementPrice` function with signature `hasSettlementPrice(address,uint88,address)` and selector `[248, 239, 11, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct HasSettlementPriceReturn(pub bool);
    #[doc = "Container type for all return fields from the `oracleRegistry` function with signature `oracleRegistry()` and selector `[75, 185, 58, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OracleRegistryReturn(pub ethers::core::types::Address);
}
