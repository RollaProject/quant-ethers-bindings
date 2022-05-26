pub use assetsregistry_mod::*;
#[allow(clippy::too_many_arguments)]
mod assetsregistry_mod {
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
    #[doc = "AssetsRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ASSETSREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AssetAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlying\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetWithOptionalERC20Methods\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetProperties\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isRegistered\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"registeredAssets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ASSETSREGISTRY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461005b5760008054336001600160a01b0319821681178355916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09084a361144390816100618239f35b600080fdfe604060808152600436101561001357600080fd5b6000803560e01c9182634f636842146100b8575050806363e4d51d146100af578063715018a6146100a65780638da5cb5b1461009d578063a083bd3c14610094578063a89d490c1461008b578063bb9453a5146100825763f2fde38b1461007a575b600080fd5b6100756109fa565b50610075610941565b506100756106fa565b50610075610674565b50610075610621565b50610075610579565b5061007561039e565b3461033e576020807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261033a577fea959577311e4d4defc79842b187bda7f80b0cff41b46787aa57a66532e7879f610111610342565b73ffffffffffffffffffffffffffffffffffffffff610134818754163314610aec565b8116928386526001815261015460ff6002878920015460081c1615610b51565b8451907f06fdde030000000000000000000000000000000000000000000000000000000082528682600481885afa91821561032d575b8792610311575b5061019e82511515611256565b8551927f95d89b410000000000000000000000000000000000000000000000000000000084528784600481895afa938415610304575b88946102da575b5090610292916101ed855115156112e1565b8751907f313ce56700000000000000000000000000000000000000000000000000000000825282826004818b5afa9182156102cd575b8a92610298575b5061028261028792938761023c610ca6565b8881529182015260ff85168b8201525b6001606082015261027d8373ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b610ed0565b6110ab565b865193849384611380565b0390a251f35b61028792506102bf61028291853d87116102c6575b6102b781836107ba565b81019061136b565b925061022a565b503d6102ad565b6102d5611249565b610223565b610292929194506102fc903d808b833e6102f481836107ba565b8101906111e7565b9390916101db565b61030c611249565b6101d4565b6103269192503d8089833e6102f481836107ba565b9087610191565b610335611249565b61018a565b8280fd5b5080fd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361007557565b9181601f840112156100755782359167ffffffffffffffff8311610075576020838186019501011161007557565b60ff81160361007557565b50346100755760807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610075576103d6610342565b67ffffffffffffffff90602435828111610075576103f8903690600401610365565b909260443590811161007557610412903690600401610365565b93909260643561042181610393565b73ffffffffffffffffffffffffffffffffffffffff61044581600054163314610aec565b82169586600052600160205261046960ff60026040600020015460081c1615610b51565b86156104f5577fea959577311e4d4defc79842b187bda7f80b0cff41b46787aa57a66532e7879f956104e46104f0946104a3881515610bdc565b6104ae841515610c41565b6102826104b9610ca6565b6104c4368b8b610d0f565b81526104d1368787610d0f565b602082015260ff8716604082015261024c565b604051958695866111b2565b0390a2005b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602a60248201527f41737365747352656769737472793a20696e76616c696420756e6465726c796960448201527f6e672061646472657373000000000000000000000000000000000000000000006064820152fd5b5034610075576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261061e5780547fffffffffffffffffffffffff000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff8216916105f0338414610aec565b16825581604051917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b80fd5b50346100755760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007557602073ffffffffffffffffffffffffffffffffffffffff60005416604051908152f35b50346100755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610075576004356002548110156100755773ffffffffffffffffffffffffffffffffffffffff60209160026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace015416604051908152f35b50346100755760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610075576020600254604051908152f35b90600182811c92168015610780575b602083101461075157565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602260045260246000fd5b91607f1691610746565b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176107fb57604052565b61080361078a565b604052565b906040519182600082549261081c84610737565b90818452600194858116908160001461088b5750600114610848575b5050610846925003836107ba565b565b9093915060005260209081600020936000915b81831061087357505061084693508201013880610838565b8554888401850152948501948794509183019161085b565b9450505050507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00915016602083015261084682604081013880610838565b918091926000905b8282106108e95750116108e2575050565b6000910152565b915080602091830151818601520182916108d1565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f60209361093a815180928187528780880191016108c9565b0116010190565b50346100755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100755773ffffffffffffffffffffffffffffffffffffffff61098e610342565b1660005260016020526109d460406000206109a881610808565b9060ff6109e260026109bc60018501610808565b930154926040519586956080875260808701906108fe565b9085820360208701526108fe565b91818116604085015260081c16151560608301520390f35b50346100755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007557610a32610342565b73ffffffffffffffffffffffffffffffffffffffff610a5681600054163314610aec565b811615610a6857610a669061139e565b005b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b15610af357565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b15610b5857565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f41737365747352656769737472793a20617373657420616c726561647920616460448201527f64656400000000000000000000000000000000000000000000000000000000006064820152fd5b15610be357565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f41737365747352656769737472793a20696e76616c6964206e616d65000000006044820152fd5b15610c4857565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f41737365747352656769737472793a20696e76616c69642073796d626f6c00006044820152fd5b604051906080820182811067ffffffffffffffff8211176107fb57604052565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f60209267ffffffffffffffff8111610d02575b01160190565b610d0a61078a565b610cfc565b929192610d1b82610cc6565b91610d2960405193846107ba565b829481845281830111610075578281602093846000960137010152565b90601f8111610d5457505050565b600091825260208220906020601f850160051c83019410610d90575b601f0160051c01915b828110610d8557505050565b818155600101610d79565b9092508290610d70565b919091825167ffffffffffffffff8111610ec3575b610dc381610dbd8454610737565b84610d46565b602080601f8311600114610e1c575081929394600092610e11575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c1916179055565b015190503880610dde565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0831695610e5085600052602060002090565b926000905b888210610eab57505083600195969710610e74575b505050811b019055565b01517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88460031b161c19169055388080610e6a565b80600185968294968601518155019501930190610e55565b610ecb61078a565b610daf565b9190805192835167ffffffffffffffff811161109e575b610ef581610dbd8454610737565b6020948590601f8311600114610fe55792610f5f610846969784606095600295610fa798600092610fda575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c19161783555b86015160018301610d9a565b0192610fa0610f72604083015160ff1690565b859060ff167fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00825416179055565b0151151590565b81547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00ff1690151560081b61ff0016179055565b015190503880610f21565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe083169161101985600052602060002090565b9260005b818110611087575097600185600295610fa79895610f5f956060996108469d9e10611050575b505050811b018355610f53565b01517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88460031b161c19169055388080611043565b92938960018192878601518155019501930161101d565b6110a661078a565b610ee7565b60025468010000000000000000811015611166575b60018101806002558110156111375773ffffffffffffffffffffffffffffffffffffffff9060026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace0191167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b61116e61078a565b6110c0565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0938186528686013760008582860101520116010190565b93926040936111d26111e09360ff95999899606089526060890191611173565b918683036020880152611173565b9416910152565b6020818303126100755780519067ffffffffffffffff8211610075570181601f8201121561007557805161121a81610cc6565b9261122860405194856107ba565b818452602082840101116100755761124691602080850191016108c9565b90565b506040513d6000823e3d90fd5b1561125d57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f41737365747352656769737472793a20696e76616c696420656d707479206e6160448201527f6d650000000000000000000000000000000000000000000000000000000000006064820152fd5b156112e857565b60846040517f08c379a0000000000000000000000000000000000000000000000000000000008152602060048201526024808201527f41737365747352656769737472793a20696e76616c696420656d70747920737960448201527f6d626f6c000000000000000000000000000000000000000000000000000000006064820152fd5b90816020910312610075575161124681610393565b916111e060ff916109d46040949796976060875260608701906108fe565b6000549073ffffffffffffffffffffffffffffffffffffffff80911691827fffffffffffffffffffffffff0000000000000000000000000000000000000000821617600055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e06000604051a356fea2646970667358221220b52347872ab0423624082c793f05289a4ffce7ba76cac6d43b0c6ead1d577d6564736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct AssetsRegistry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for AssetsRegistry<M> {
        fn clone(&self) -> Self {
            AssetsRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for AssetsRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AssetsRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AssetsRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> AssetsRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ASSETSREGISTRY_ABI.clone(), client)
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
                ASSETSREGISTRY_ABI.clone(),
                ASSETSREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addAsset` (0x63e4d51d) function"]
        pub fn add_asset(
            &self,
            underlying: ethers::core::types::Address,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 228, 213, 29], (underlying, name, symbol, decimals))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addAssetWithOptionalERC20Methods` (0x4f636842) function"]
        pub fn add_asset_with_optional_erc20_methods(
            &self,
            underlying: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 99, 104, 66], underlying)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetProperties` (0xbb9453a5) function"]
        pub fn asset_properties(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (String, String, u8, bool)> {
            self.0
                .method_hash([187, 148, 83, 165], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetsLength` (0xa89d490c) function"]
        pub fn get_assets_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([168, 157, 73, 12], ())
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
        #[doc = "Calls the contract's `registeredAssets` (0xa083bd3c) function"]
        pub fn registered_assets(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([160, 131, 189, 60], p0)
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
        #[doc = "Gets the contract's `AssetAdded` event"]
        pub fn asset_added_filter(&self) -> ethers::contract::builders::Event<M, AssetAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AssetsRegistryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for AssetsRegistry<M> {
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
    #[ethevent(name = "AssetAdded", abi = "AssetAdded(address,string,string,uint8)")]
    pub struct AssetAddedFilter {
        #[ethevent(indexed)]
        pub underlying: ethers::core::types::Address,
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
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
    pub enum AssetsRegistryEvents {
        AssetAddedFilter(AssetAddedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for AssetsRegistryEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AssetAddedFilter::decode_log(log) {
                return Ok(AssetsRegistryEvents::AssetAddedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AssetsRegistryEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AssetsRegistryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AssetsRegistryEvents::AssetAddedFilter(element) => element.fmt(f),
                AssetsRegistryEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addAsset`function with signature `addAsset(address,string,string,uint8)` and selector `[99, 228, 213, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addAsset", abi = "addAsset(address,string,string,uint8)")]
    pub struct AddAssetCall {
        pub underlying: ethers::core::types::Address,
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
    }
    #[doc = "Container type for all input parameters for the `addAssetWithOptionalERC20Methods`function with signature `addAssetWithOptionalERC20Methods(address)` and selector `[79, 99, 104, 66]`"]
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
        name = "addAssetWithOptionalERC20Methods",
        abi = "addAssetWithOptionalERC20Methods(address)"
    )]
    pub struct AddAssetWithOptionalERC20MethodsCall {
        pub underlying: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `assetProperties`function with signature `assetProperties(address)` and selector `[187, 148, 83, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assetProperties", abi = "assetProperties(address)")]
    pub struct AssetPropertiesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `getAssetsLength`function with signature `getAssetsLength()` and selector `[168, 157, 73, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetsLength", abi = "getAssetsLength()")]
    pub struct GetAssetsLengthCall;
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
    #[doc = "Container type for all input parameters for the `registeredAssets`function with signature `registeredAssets(uint256)` and selector `[160, 131, 189, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "registeredAssets", abi = "registeredAssets(uint256)")]
    pub struct RegisteredAssetsCall(pub ethers::core::types::U256);
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
    pub enum AssetsRegistryCalls {
        AddAsset(AddAssetCall),
        AddAssetWithOptionalERC20Methods(AddAssetWithOptionalERC20MethodsCall),
        AssetProperties(AssetPropertiesCall),
        GetAssetsLength(GetAssetsLengthCall),
        Owner(OwnerCall),
        RegisteredAssets(RegisteredAssetsCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for AssetsRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssetsRegistryCalls::AddAsset(decoded));
            }
            if let Ok(decoded) =
                <AddAssetWithOptionalERC20MethodsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AssetsRegistryCalls::AddAssetWithOptionalERC20Methods(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AssetPropertiesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssetsRegistryCalls::AssetProperties(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssetsRegistryCalls::GetAssetsLength(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssetsRegistryCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RegisteredAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssetsRegistryCalls::RegisteredAssets(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssetsRegistryCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AssetsRegistryCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AssetsRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AssetsRegistryCalls::AddAsset(element) => element.encode(),
                AssetsRegistryCalls::AddAssetWithOptionalERC20Methods(element) => element.encode(),
                AssetsRegistryCalls::AssetProperties(element) => element.encode(),
                AssetsRegistryCalls::GetAssetsLength(element) => element.encode(),
                AssetsRegistryCalls::Owner(element) => element.encode(),
                AssetsRegistryCalls::RegisteredAssets(element) => element.encode(),
                AssetsRegistryCalls::RenounceOwnership(element) => element.encode(),
                AssetsRegistryCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AssetsRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AssetsRegistryCalls::AddAsset(element) => element.fmt(f),
                AssetsRegistryCalls::AddAssetWithOptionalERC20Methods(element) => element.fmt(f),
                AssetsRegistryCalls::AssetProperties(element) => element.fmt(f),
                AssetsRegistryCalls::GetAssetsLength(element) => element.fmt(f),
                AssetsRegistryCalls::Owner(element) => element.fmt(f),
                AssetsRegistryCalls::RegisteredAssets(element) => element.fmt(f),
                AssetsRegistryCalls::RenounceOwnership(element) => element.fmt(f),
                AssetsRegistryCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddAssetCall> for AssetsRegistryCalls {
        fn from(var: AddAssetCall) -> Self {
            AssetsRegistryCalls::AddAsset(var)
        }
    }
    impl ::std::convert::From<AddAssetWithOptionalERC20MethodsCall> for AssetsRegistryCalls {
        fn from(var: AddAssetWithOptionalERC20MethodsCall) -> Self {
            AssetsRegistryCalls::AddAssetWithOptionalERC20Methods(var)
        }
    }
    impl ::std::convert::From<AssetPropertiesCall> for AssetsRegistryCalls {
        fn from(var: AssetPropertiesCall) -> Self {
            AssetsRegistryCalls::AssetProperties(var)
        }
    }
    impl ::std::convert::From<GetAssetsLengthCall> for AssetsRegistryCalls {
        fn from(var: GetAssetsLengthCall) -> Self {
            AssetsRegistryCalls::GetAssetsLength(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for AssetsRegistryCalls {
        fn from(var: OwnerCall) -> Self {
            AssetsRegistryCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RegisteredAssetsCall> for AssetsRegistryCalls {
        fn from(var: RegisteredAssetsCall) -> Self {
            AssetsRegistryCalls::RegisteredAssets(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for AssetsRegistryCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            AssetsRegistryCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for AssetsRegistryCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            AssetsRegistryCalls::TransferOwnership(var)
        }
    }
}
