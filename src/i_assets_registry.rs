pub use i_assets_registry::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_assets_registry {
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
    #[doc = "IAssetsRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IASSETSREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AssetAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlying\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetWithOptionalERC20Methods\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetProperties\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isRegistered\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"length\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"registeredAssets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IAssetsRegistry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IAssetsRegistry<M> {
        fn clone(&self) -> Self {
            IAssetsRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IAssetsRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAssetsRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAssetsRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IAssetsRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IASSETSREGISTRY_ABI.clone(), client)
                .into()
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
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (String, String, u8, bool)> {
            self.0
                .method_hash([187, 148, 83, 165], asset)
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
        #[doc = "Calls the contract's `registeredAssets` (0xa083bd3c) function"]
        pub fn registered_assets(
            &self,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([160, 131, 189, 60], index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AssetAdded` event"]
        pub fn asset_added_filter(&self) -> ethers::contract::builders::Event<M, AssetAddedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AssetAddedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IAssetsRegistry<M> {
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
    pub struct AssetPropertiesCall {
        pub asset: ethers::core::types::Address,
    }
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
    pub struct RegisteredAssetsCall {
        pub index: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAssetsRegistryCalls {
        AddAsset(AddAssetCall),
        AddAssetWithOptionalERC20Methods(AddAssetWithOptionalERC20MethodsCall),
        AssetProperties(AssetPropertiesCall),
        GetAssetsLength(GetAssetsLengthCall),
        RegisteredAssets(RegisteredAssetsCall),
    }
    impl ethers::core::abi::AbiDecode for IAssetsRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAssetsRegistryCalls::AddAsset(decoded));
            }
            if let Ok(decoded) =
                <AddAssetWithOptionalERC20MethodsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAssetsRegistryCalls::AddAssetWithOptionalERC20Methods(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AssetPropertiesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAssetsRegistryCalls::AssetProperties(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAssetsRegistryCalls::GetAssetsLength(decoded));
            }
            if let Ok(decoded) =
                <RegisteredAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAssetsRegistryCalls::RegisteredAssets(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAssetsRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAssetsRegistryCalls::AddAsset(element) => element.encode(),
                IAssetsRegistryCalls::AddAssetWithOptionalERC20Methods(element) => element.encode(),
                IAssetsRegistryCalls::AssetProperties(element) => element.encode(),
                IAssetsRegistryCalls::GetAssetsLength(element) => element.encode(),
                IAssetsRegistryCalls::RegisteredAssets(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAssetsRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAssetsRegistryCalls::AddAsset(element) => element.fmt(f),
                IAssetsRegistryCalls::AddAssetWithOptionalERC20Methods(element) => element.fmt(f),
                IAssetsRegistryCalls::AssetProperties(element) => element.fmt(f),
                IAssetsRegistryCalls::GetAssetsLength(element) => element.fmt(f),
                IAssetsRegistryCalls::RegisteredAssets(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddAssetCall> for IAssetsRegistryCalls {
        fn from(var: AddAssetCall) -> Self {
            IAssetsRegistryCalls::AddAsset(var)
        }
    }
    impl ::std::convert::From<AddAssetWithOptionalERC20MethodsCall> for IAssetsRegistryCalls {
        fn from(var: AddAssetWithOptionalERC20MethodsCall) -> Self {
            IAssetsRegistryCalls::AddAssetWithOptionalERC20Methods(var)
        }
    }
    impl ::std::convert::From<AssetPropertiesCall> for IAssetsRegistryCalls {
        fn from(var: AssetPropertiesCall) -> Self {
            IAssetsRegistryCalls::AssetProperties(var)
        }
    }
    impl ::std::convert::From<GetAssetsLengthCall> for IAssetsRegistryCalls {
        fn from(var: GetAssetsLengthCall) -> Self {
            IAssetsRegistryCalls::GetAssetsLength(var)
        }
    }
    impl ::std::convert::From<RegisteredAssetsCall> for IAssetsRegistryCalls {
        fn from(var: RegisteredAssetsCall) -> Self {
            IAssetsRegistryCalls::RegisteredAssets(var)
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
    pub struct GetAssetsLengthReturn {
        pub length: ethers::core::types::U256,
    }
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
    pub struct RegisteredAssetsReturn {
        pub asset: ethers::core::types::Address,
    }
}
