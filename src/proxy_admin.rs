pub use proxyadmin_mod::*;
#[allow(clippy::too_many_arguments)]
mod proxyadmin_mod {
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
    #[doc = "ProxyAdmin was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PROXYADMIN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract TransparentUpgradeableProxy\",\"name\":\"proxy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeProxyAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract TransparentUpgradeableProxy\",\"name\":\"proxy\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProxyAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract TransparentUpgradeableProxy\",\"name\":\"proxy\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProxyImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract TransparentUpgradeableProxy\",\"name\":\"proxy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract TransparentUpgradeableProxy\",\"name\":\"proxy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeAndCall\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PROXYADMIN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461005b5760008054336001600160a01b0319821681178355916001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09084a36109c090816100618239f35b600080fdfe608080604052600436101561001357600080fd5b6000803560e01c918263204e1c7a146100b85750508063715018a6146100af5780637eff275e146100a65780638da5cb5b1461009d5780639623609d1461009457806399a88ec41461008b578063f2fde38b146100825763f3b7dead1461007a575b600080fd5b6100756106a1565b506100756105ad565b50610075610525565b5061007561047b565b5061007561034d565b506100756102a3565b506100756101b8565b3461018b5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261018b57818060048035936100f78561018f565b7f5c60da1b00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8095165afa9161013d610822565b9215610188575061015a8260208061018495518301019101610852565b166040519182918291909173ffffffffffffffffffffffffffffffffffffffff6020820193169052565b0390f35b80fd5b5080fd5b73ffffffffffffffffffffffffffffffffffffffff81160361007557565b600091031261007557565b5034610075576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101885780547fffffffffffffffffffffffff000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff82169161022f33841461074e565b16825581604051917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc6040910112610075576004356102938161018f565b906024356102a08161018f565b90565b5034610075576102b23661025d565b6000809273ffffffffffffffffffffffffffffffffffffffff80916102db82855416331461074e565b1692833b15610349576024908360405195869485937f8f2839700000000000000000000000000000000000000000000000000000000085521660048401525af1801561033c575b61032d575b50604051f35b610336906103d0565b38610327565b610344610867565b610322565b8280fd5b50346100755760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007557602073ffffffffffffffffffffffffffffffffffffffff60005416604051908152f35b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b67ffffffffffffffff81116103e457604052565b6103ec6103a0565b604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176103e457604052565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f60209267ffffffffffffffff811161046e575b01160190565b6104766103a0565b610468565b5060607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610075576004356104b28161018f565b602435906104bf8261018f565b6044359167ffffffffffffffff83116100755736602384011215610075578260040135916104ec83610432565b926104fa60405194856103f1565b808452366024828701011161007557602081600092602461052398018388013785010152610874565b005b5034610075576105343661025d565b6000809273ffffffffffffffffffffffffffffffffffffffff809161055d82855416331461074e565b1692833b15610349576024908360405195869485937f3659cfe60000000000000000000000000000000000000000000000000000000085521660048401525af1801561033c5761032d5750604051f35b50346100755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610075576004356105e98161018f565b73ffffffffffffffffffffffffffffffffffffffff61060d8160005416331461074e565b81161561061d57610523906107b3565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b50346100755760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610075576004356106dd8161018f565b60008060046040517ff851a44000000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8095165afa9061072b610822565b911561007557610745826020808095518301019101610852565b16604051908152f35b1561075557565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b6000549073ffffffffffffffffffffffffffffffffffffffff80911691827fffffffffffffffffffffffff0000000000000000000000000000000000000000821617600055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e06000604051a3565b3d1561084d573d9061083382610432565b9161084160405193846103f1565b82523d6000602084013e565b606090565b9081602091031261007557516102a08161018f565b506040513d6000823e3d90fd5b909173ffffffffffffffffffffffffffffffffffffffff908160009361089e82865416331461074e565b1691823b15610986579391906040519485937f4f1ef2860000000000000000000000000000000000000000000000000000000085521660048401526040602484015280519081604485015284905b82821061096b5750837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8483956064951161095f575b0116810103019134905af18015610952575b61093d5750565b8061094a610950926103d0565b806101ad565b565b61095a610867565b610936565b88858286010152610924565b602081830181015160648985010152879550909101906108ec565b8380fdfea26469706673582212202b61887f2428a64b05496bb1a63ea716c832d2d04788ca9fd5f2e3b7ac2c271764736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ProxyAdmin<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ProxyAdmin<M> {
        fn clone(&self) -> Self {
            ProxyAdmin(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ProxyAdmin<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ProxyAdmin<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ProxyAdmin))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ProxyAdmin<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PROXYADMIN_ABI.clone(), client).into()
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
                PROXYADMIN_ABI.clone(),
                PROXYADMIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `changeProxyAdmin` (0x7eff275e) function"]
        pub fn change_proxy_admin(
            &self,
            proxy: ethers::core::types::Address,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 255, 39, 94], (proxy, new_admin))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProxyAdmin` (0xf3b7dead) function"]
        pub fn get_proxy_admin(
            &self,
            proxy: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([243, 183, 222, 173], proxy)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProxyImplementation` (0x204e1c7a) function"]
        pub fn get_proxy_implementation(
            &self,
            proxy: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([32, 78, 28, 122], proxy)
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
        #[doc = "Calls the contract's `upgrade` (0x99a88ec4) function"]
        pub fn upgrade(
            &self,
            proxy: ethers::core::types::Address,
            implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 168, 142, 196], (proxy, implementation))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeAndCall` (0x9623609d) function"]
        pub fn upgrade_and_call(
            &self,
            proxy: ethers::core::types::Address,
            implementation: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 35, 96, 157], (proxy, implementation, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ProxyAdmin<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `changeProxyAdmin`function with signature `changeProxyAdmin(address,address)` and selector `[126, 255, 39, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "changeProxyAdmin", abi = "changeProxyAdmin(address,address)")]
    pub struct ChangeProxyAdminCall {
        pub proxy: ethers::core::types::Address,
        pub new_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getProxyAdmin`function with signature `getProxyAdmin(address)` and selector `[243, 183, 222, 173]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getProxyAdmin", abi = "getProxyAdmin(address)")]
    pub struct GetProxyAdminCall {
        pub proxy: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getProxyImplementation`function with signature `getProxyImplementation(address)` and selector `[32, 78, 28, 122]`"]
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
        name = "getProxyImplementation",
        abi = "getProxyImplementation(address)"
    )]
    pub struct GetProxyImplementationCall {
        pub proxy: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `upgrade`function with signature `upgrade(address,address)` and selector `[153, 168, 142, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgrade", abi = "upgrade(address,address)")]
    pub struct UpgradeCall {
        pub proxy: ethers::core::types::Address,
        pub implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeAndCall`function with signature `upgradeAndCall(address,address,bytes)` and selector `[150, 35, 96, 157]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgradeAndCall", abi = "upgradeAndCall(address,address,bytes)")]
    pub struct UpgradeAndCallCall {
        pub proxy: ethers::core::types::Address,
        pub implementation: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ProxyAdminCalls {
        ChangeProxyAdmin(ChangeProxyAdminCall),
        GetProxyAdmin(GetProxyAdminCall),
        GetProxyImplementation(GetProxyImplementationCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        Upgrade(UpgradeCall),
        UpgradeAndCall(UpgradeAndCallCall),
    }
    impl ethers::core::abi::AbiDecode for ProxyAdminCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ChangeProxyAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProxyAdminCalls::ChangeProxyAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetProxyAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProxyAdminCalls::GetProxyAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetProxyImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProxyAdminCalls::GetProxyImplementation(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProxyAdminCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProxyAdminCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProxyAdminCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProxyAdminCalls::Upgrade(decoded));
            }
            if let Ok(decoded) =
                <UpgradeAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ProxyAdminCalls::UpgradeAndCall(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ProxyAdminCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ProxyAdminCalls::ChangeProxyAdmin(element) => element.encode(),
                ProxyAdminCalls::GetProxyAdmin(element) => element.encode(),
                ProxyAdminCalls::GetProxyImplementation(element) => element.encode(),
                ProxyAdminCalls::Owner(element) => element.encode(),
                ProxyAdminCalls::RenounceOwnership(element) => element.encode(),
                ProxyAdminCalls::TransferOwnership(element) => element.encode(),
                ProxyAdminCalls::Upgrade(element) => element.encode(),
                ProxyAdminCalls::UpgradeAndCall(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ProxyAdminCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ProxyAdminCalls::ChangeProxyAdmin(element) => element.fmt(f),
                ProxyAdminCalls::GetProxyAdmin(element) => element.fmt(f),
                ProxyAdminCalls::GetProxyImplementation(element) => element.fmt(f),
                ProxyAdminCalls::Owner(element) => element.fmt(f),
                ProxyAdminCalls::RenounceOwnership(element) => element.fmt(f),
                ProxyAdminCalls::TransferOwnership(element) => element.fmt(f),
                ProxyAdminCalls::Upgrade(element) => element.fmt(f),
                ProxyAdminCalls::UpgradeAndCall(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ChangeProxyAdminCall> for ProxyAdminCalls {
        fn from(var: ChangeProxyAdminCall) -> Self {
            ProxyAdminCalls::ChangeProxyAdmin(var)
        }
    }
    impl ::std::convert::From<GetProxyAdminCall> for ProxyAdminCalls {
        fn from(var: GetProxyAdminCall) -> Self {
            ProxyAdminCalls::GetProxyAdmin(var)
        }
    }
    impl ::std::convert::From<GetProxyImplementationCall> for ProxyAdminCalls {
        fn from(var: GetProxyImplementationCall) -> Self {
            ProxyAdminCalls::GetProxyImplementation(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ProxyAdminCalls {
        fn from(var: OwnerCall) -> Self {
            ProxyAdminCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ProxyAdminCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ProxyAdminCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ProxyAdminCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ProxyAdminCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UpgradeCall> for ProxyAdminCalls {
        fn from(var: UpgradeCall) -> Self {
            ProxyAdminCalls::Upgrade(var)
        }
    }
    impl ::std::convert::From<UpgradeAndCallCall> for ProxyAdminCalls {
        fn from(var: UpgradeAndCallCall) -> Self {
            ProxyAdminCalls::UpgradeAndCall(var)
        }
    }
}
