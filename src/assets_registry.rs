pub use assets_registry::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod assets_registry {
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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AssetAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlying\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetWithOptionalERC20Methods\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetProperties\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isRegistered\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"registeredAssets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ASSETSREGISTRY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461005b5760008054336001600160a01b0319821681178355916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09084a361178990816100618239f35b600080fdfe6080604052600436101561001257600080fd5b60003560e01c80634f63684214610b0857806363e4d51d1461041f578063715018a61461037e5780638da5cb5b1461032c578063a083bd3c146102a7578063a89d490c1461026b578063bb9453a5146101b35763f2fde38b1461007457600080fd5b346101ae5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101ae576100ab611268565b6100b36116fd565b73ffffffffffffffffffffffffffffffffffffffff80911690811561012a57600091825491817fffffffffffffffffffffffff0000000000000000000000000000000000000000841617845560405192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08484a3f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b600080fd5b346101ae5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101ae5773ffffffffffffffffffffffffffffffffffffffff6101ff611268565b166000526001602052610245604060002061021981611369565b9060ff610253600261022d60018501611369565b93015492604051958695608087526080870190611463565b908582036020870152611463565b91818116604085015260081c16151560608301520390f35b346101ae5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101ae576020600254604051908152f35b346101ae5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101ae576004356002548110156101ae5773ffffffffffffffffffffffffffffffffffffffff60209160026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace015416604051908152f35b346101ae5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101ae57602073ffffffffffffffffffffffffffffffffffffffff60005416604051908152f35b346101ae5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101ae576103b56116fd565b600080547fffffffffffffffffffffffff0000000000000000000000000000000000000000811682558173ffffffffffffffffffffffffffffffffffffffff60405192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b346101ae5760807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101ae57610456611268565b60243567ffffffffffffffff81116101ae5761047690369060040161128b565b9060443567ffffffffffffffff81116101ae5761049790369060040161128b565b92906064359360ff851685036101ae576104af6116fd565b73ffffffffffffffffffffffffffffffffffffffff861660005260016020526104e660ff60026040600020015460081c16156114a6565b73ffffffffffffffffffffffffffffffffffffffff861615610a84578215610a265780156109c85760405161051a8161130c565b61052536858761156b565b815261053236838561156b565b602082015260ff861660408201526001606082015273ffffffffffffffffffffffffffffffffffffffff871660005260016020526040600020815180519067ffffffffffffffff821161087857819061058b84546112b9565b601f8111610978575b50602090601f83116001146108b2576000926108a7575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c19161781555b602082015180519067ffffffffffffffff82116108785761060160018401546112b9565b601f8111610831575b50602090601f83116001146107245793600261070b948473ffffffffffffffffffffffffffffffffffffffff99957fea959577311e4d4defc79842b187bda7f80b0cff41b46787aa57a66532e7879f9d9e999560ff99600092610719575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c19161760018201555b0190856040820151167fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000061ff0060608554940151151560081b169216171790556106e6856115a2565b6106fd60405198899860608a5260608a019161165c565b91878303602089015261165c565b9616604084015216930390a2005b015190508f80610668565b906001840160005260206000209160005b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe085168110610819575061070b946001857fea959577311e4d4defc79842b187bda7f80b0cff41b46787aa57a66532e7879f9d9e999560ff9995600295837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe073ffffffffffffffffffffffffffffffffffffffff9f9b16106107e2575b505050811b01600182015561069d565b01517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88460031b161c191690558f80806107d2565b91926020600181928685015181550194019201610735565b600184016000526020600020601f840160051c810160208510610871575b601f830160051c8201811061086557505061060a565b6000815560010161084f565b508061084f565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b015190508a806105ab565b9250836000526020600020906000935b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08416851061095d5760019450837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0811610610926575b505050811b0181556105dd565b01517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88460031b161c191690558a8080610919565b818101518355602094850194600190930192909101906108c2565b909150836000526020600020601f840160051c8101602085106109c1575b90849392915b601f830160051c820181106109b2575050610594565b6000815585945060010161099c565b5080610996565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f41737365747352656769737472793a20696e76616c69642073796d626f6c00006044820152fd5b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f41737365747352656769737472793a20696e76616c6964206e616d65000000006044820152fd5b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602a60248201527f41737365747352656769737472793a20696e76616c696420756e6465726c796960448201527f6e672061646472657373000000000000000000000000000000000000000000006064820152fd5b346101ae5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101ae57610b3f611268565b610b476116fd565b73ffffffffffffffffffffffffffffffffffffffff81166000526001602052610b7e60ff60026040600020015460081c16156114a6565b6040517f06fdde0300000000000000000000000000000000000000000000000000000000815260008160048173ffffffffffffffffffffffffffffffffffffffff86165afa9081156111155760009161124d575b508051156111c957604051917f95d89b4100000000000000000000000000000000000000000000000000000000835260008360048173ffffffffffffffffffffffffffffffffffffffff85165afa928315611115576000936111a4575b5082511561112157604051927f313ce56700000000000000000000000000000000000000000000000000000000845260208460048173ffffffffffffffffffffffffffffffffffffffff86165afa938415611115576000946110d7575b50604051610c998161130c565b83815281602082015260ff851660408201526001606082015273ffffffffffffffffffffffffffffffffffffffff831660005260016020526040600020815180519067ffffffffffffffff8211610878578190610cf684546112b9565b601f8111611087575b50602090601f8311600114610fc157600092610fb6575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c19161781555b602082015180519067ffffffffffffffff821161087857610d6c60018401546112b9565b601f8111610f6f575b50602090601f8311600114610e80579473ffffffffffffffffffffffffffffffffffffffff946002610e6798958560ff9661070b967fea959577311e4d4defc79842b187bda7f80b0cff41b46787aa57a66532e7879f9d9b600092610e75575b50507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8260011b9260031b1c19161760018201555b0190846040820151167fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000061ff0060608554940151151560081b16921617179055610e53846115a2565b604051968796606088526060880190611463565b908682036020880152611463565b015190508e80610dd5565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08316916001850160005260206000209260005b818110610f575750610e67989560018661070b967fea959577311e4d4defc79842b187bda7f80b0cff41b46787aa57a66532e7879f9d9b9673ffffffffffffffffffffffffffffffffffffffff9b9660ff9a60029710610f20575b505050811b016001820155610e0a565b01517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88460031b161c191690558e8080610f10565b92936020600181928786015181550195019301610eb5565b600184016000526020600020601f840160051c810160208510610faf575b601f830160051c82018110610fa3575050610d75565b60008155600101610f8d565b5080610f8d565b015190508880610d16565b9250836000526020600020906000935b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08416851061106c5760019450837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0811610611035575b505050811b018155610d48565b01517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60f88460031b161c19169055888080611028565b81810151835560209485019460019093019290910190610fd1565b909150836000526020600020601f840160051c8101602085106110d0575b90849392915b601f830160051c820181106110c1575050610cff565b600081558594506001016110ab565b50806110a5565b9093506020813d60201161110d575b816110f360209383611328565b810103126101ae575160ff811681036101ae579284610c8c565b3d91506110e6565b6040513d6000823e3d90fd5b60846040517f08c379a0000000000000000000000000000000000000000000000000000000008152602060048201526024808201527f41737365747352656769737472793a20696e76616c696420656d70747920737960448201527f6d626f6c000000000000000000000000000000000000000000000000000000006064820152fd5b6111c29193503d806000833e6111ba8183611328565b81019061169b565b9183610c2f565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f41737365747352656769737472793a20696e76616c696420656d707479206e6160448201527f6d650000000000000000000000000000000000000000000000000000000000006064820152fd5b61126291503d806000833e6111ba8183611328565b82610bd2565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036101ae57565b9181601f840112156101ae5782359167ffffffffffffffff83116101ae57602083818601950101116101ae57565b90600182811c92168015611302575b60208310146112d357565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602260045260246000fd5b91607f16916112c8565b6080810190811067ffffffffffffffff82111761087857604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761087857604052565b906040519182600082549261137d846112b9565b9081845260019485811690816000146113ec57506001146113a9575b50506113a792500383611328565b565b9093915060005260209081600020936000915b8183106113d45750506113a793508201013880611399565b855488840185015294850194879450918301916113bc565b90506113a79550602093507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0091501682840152151560051b8201013880611399565b918091926000905b82821061144e575011611447575050565b6000910152565b91508060209183015181860152018291611436565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f60209361149f8151809281875287808801910161142e565b0116010190565b156114ad57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f41737365747352656769737472793a20617373657420616c726561647920616460448201527f64656400000000000000000000000000000000000000000000000000000000006064820152fd5b67ffffffffffffffff811161087857601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b92919261157782611531565b916115856040519384611328565b8294818452818301116101ae578281602093846000960137010152565b6002546801000000000000000081101561087857600181018060025581101561162d5773ffffffffffffffffffffffffffffffffffffffff9060026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace0191167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0938186528686013760008582860101520116010190565b6020818303126101ae5780519067ffffffffffffffff82116101ae570181601f820112156101ae5780516116ce81611531565b926116dc6040519485611328565b818452602082840101116101ae576116fa916020808501910161142e565b90565b73ffffffffffffffffffffffffffffffffffffffff60005416330361171e57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fdfea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
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
    #[doc = "Container type for all input parameters for the `addAsset` function with signature `addAsset(address,string,string,uint8)` and selector `[99, 228, 213, 29]`"]
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
    #[doc = "Container type for all input parameters for the `addAssetWithOptionalERC20Methods` function with signature `addAssetWithOptionalERC20Methods(address)` and selector `[79, 99, 104, 66]`"]
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
    #[doc = "Container type for all input parameters for the `assetProperties` function with signature `assetProperties(address)` and selector `[187, 148, 83, 165]`"]
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
    #[doc = "Container type for all input parameters for the `getAssetsLength` function with signature `getAssetsLength()` and selector `[168, 157, 73, 12]`"]
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
    #[doc = "Container type for all input parameters for the `registeredAssets` function with signature `registeredAssets(uint256)` and selector `[160, 131, 189, 60]`"]
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
    #[doc = "Container type for all return fields from the `assetProperties` function with signature `assetProperties(address)` and selector `[187, 148, 83, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssetPropertiesReturn {
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
        pub is_registered: bool,
    }
    #[doc = "Container type for all return fields from the `getAssetsLength` function with signature `getAssetsLength()` and selector `[168, 157, 73, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAssetsLengthReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `registeredAssets` function with signature `registeredAssets(uint256)` and selector `[160, 131, 189, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RegisteredAssetsReturn(pub ethers::core::types::Address);
}
