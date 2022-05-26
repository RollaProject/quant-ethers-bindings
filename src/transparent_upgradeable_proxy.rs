pub use transparentupgradeableproxy_mod::*;
#[allow(clippy::too_many_arguments)]
mod transparentupgradeableproxy_mod {
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
    #[doc = "TransparentUpgradeableProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TRANSPARENTUPGRADEABLEPROXY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_logic\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"admin_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"beacon\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BeaconUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"admin_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeAdmin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static TRANSPARENTUPGRADEABLEPROXY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405262000ff7803803806200001781620000d6565b9283398101606082820312620000ba5762000032826200010b565b9162000041602082016200010b565b604082015190916001600160401b038211620000ba57019282601f85011215620000ba578351936200007d620000778662000120565b620000d6565b9385855260208683010111620000ba57620000aa94620000a491602080870191016200014e565b62000186565b604051610b1f9081620004d88239f35b600080fd5b50634e487b7160e01b600052604160045260246000fd5b6040519190601f01601f191682016001600160401b03811183821017620000fc57604052565b62000106620000bf565b604052565b51906001600160a01b0382168203620000ba57565b6020906001600160401b0381116200013e575b601f01601f19160190565b62000148620000bf565b62000133565b918091926000905b8282106200017057501162000169575050565b6000910152565b9150806020918301518186015201829162000156565b91823b1562000240577f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc80546001600160a01b0319166001600160a01b038516908117909155604051620002169491907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a281511580159062000237575b62000218575b50506200029b565b565b6200022e916200022762000369565b91620003d4565b5038806200020e565b50600062000208565b60405162461bcd60e51b815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201526c1bdd08184818dbdb9d1c9858dd609a1b6064820152608490fd5b7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61038054604080516001600160a01b0380841682529094166020850181905292937f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f9190a1811562000315576001600160a01b031916179055565b60405162461bcd60e51b815260206004820152602660248201527f455243313936373a206e65772061646d696e20697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b60405190606082016001600160401b03811183821017620003c4575b60405260278252660819985a5b195960ca1b6040837f416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c60208201520152565b620003ce620000bf565b62000385565b9190823b156200042557600081620004199460208394519201905af43d156200041c573d9062000408620000778362000120565b9182523d6000602084013e62000479565b90565b60609062000479565b60405162461bcd60e51b815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f6044820152651b9d1c9858dd60d21b6064820152608490fd5b9091901562000486575090565b815115620004975750805190602001fd5b6044604051809262461bcd60e51b825260206004830152620004c981518092816024860152602086860191016200014e565b601f01601f19168101030190fdfe6080604052600436101561002c575b361561001f575b61001d6104df565b005b6100276104df565b610015565b6000803560e01c9081633659cfe614610093575080634f1ef2861461008a5780635c60da1b146100815780638f283970146100785763f851a4400361000e57610073610457565b61000e565b506100736102f2565b5061007361023d565b50610073610159565b3461012e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261012e576100ca610131565b73ffffffffffffffffffffffffffffffffffffffff7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d610354163314600014610125576101209061011761063b565b908382526106f5565b604051f35b506101206104df565b80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361015457565b600080fd5b5060407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101545761018c610131565b60243567ffffffffffffffff9182821161015457366023830112156101545781600401359283116101545736602484840101116101545773ffffffffffffffffffffffffffffffffffffffff7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61035416331460001461023257600060208480602461022061021b61001d996106ac565b610668565b96828852018387013784010152610837565b50505061001d6104df565b50346101545760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610154576020600073ffffffffffffffffffffffffffffffffffffffff90817fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103541633146000146102e45750807f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5416905b60405191168152f35b906102ed6104df565b6102db565b50346101545760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101545761032a610131565b73ffffffffffffffffffffffffffffffffffffffff907fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d610391808354163314600014610232577f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f604084549281519481851686521693846020820152a181156103d3577fffffffffffffffffffffffff000000000000000000000000000000000000000016179055005b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f455243313936373a206e65772061646d696e20697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b50346101545760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610154576020600073ffffffffffffffffffffffffffffffffffffffff7fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61038181541633146000146104da5754604051911681529050f35b506102ed5b5073ffffffffffffffffffffffffffffffffffffffff807fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d610354163314610561577f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc54166000808092368280378136915af43d82803e1561055d573d90f35b3d90fd5b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152604260248201527f5472616e73706172656e745570677261646561626c6550726f78793a2061646d60448201527f696e2063616e6e6f742066616c6c6261636b20746f2070726f7879207461726760648201527f65740000000000000000000000000000000000000000000000000000000000006084820152fd5b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051906020820182811067ffffffffffffffff82111761065b57604052565b61066361060b565b604052565b907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f604051930116820182811067ffffffffffffffff82111761065b57604052565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f60209267ffffffffffffffff81116106e8575b01160190565b6106f061060b565b6106e2565b803b156107b35773ffffffffffffffffffffffffffffffffffffffff81167f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc817fffffffffffffffffffffffff00000000000000000000000000000000000000008254161790557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b6000604051a28151158015906107ab575b610796575050565b6107a8916107a26108df565b9161095d565b50565b50600061078e565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602d60248201527f455243313936373a206e657720696d706c656d656e746174696f6e206973206e60448201527f6f74206120636f6e7472616374000000000000000000000000000000000000006064820152fd5b803b156107b35773ffffffffffffffffffffffffffffffffffffffff81167f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc817fffffffffffffffffffffffff00000000000000000000000000000000000000008254161790557fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b6000604051a28151158015906108d757610796575050565b50600161078e565b604051906060820182811067ffffffffffffffff821117610950575b604052602782527f206661696c6564000000000000000000000000000000000000000000000000006040837f416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c60208201520152565b61095861060b565b6108fb565b9190823b156109a65760008161099b9460208394519201905af43d1561099e573d9061098b61021b836106ac565b9182523d6000602084013e610a2a565b90565b606090610a2a565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f416464726573733a2064656c65676174652063616c6c20746f206e6f6e2d636f60448201527f6e747261637400000000000000000000000000000000000000000000000000006064820152fd5b90919015610a36575090565b815115610a465750805190602001fd5b604051907f08c379a00000000000000000000000000000000000000000000000000000000082528160208060048301528251928360248401526000915b848310610ad0575050601f836044947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe09311610ac3575b01168101030190fd5b6000858286010152610aba565b8183018101518684016044015285935091820191610a8356fea2646970667358221220d4535acaedc780010a92d38b84f2326f05e0ad15466364267aa45376153c881764736f6c634300080e0033" . parse () . expect ("invalid bytecode")
    });
    pub struct TransparentUpgradeableProxy<M>(ethers::contract::Contract<M>);
    impl<M> Clone for TransparentUpgradeableProxy<M> {
        fn clone(&self) -> Self {
            TransparentUpgradeableProxy(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for TransparentUpgradeableProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for TransparentUpgradeableProxy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TransparentUpgradeableProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> TransparentUpgradeableProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                TRANSPARENTUPGRADEABLEPROXY_ABI.clone(),
                client,
            )
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
                TRANSPARENTUPGRADEABLEPROXY_ABI.clone(),
                TRANSPARENTUPGRADEABLEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeAdmin` (0x8f283970) function"]
        pub fn change_admin(
            &self,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 40, 57, 112], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `implementation` (0x5c60da1b) function"]
        pub fn implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeTo` (0x3659cfe6) function"]
        pub fn upgrade_to(
            &self,
            new_implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeToAndCall` (0x4f1ef286) function"]
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AdminChanged` event"]
        pub fn admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BeaconUpgraded` event"]
        pub fn beacon_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BeaconUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, TransparentUpgradeableProxyEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for TransparentUpgradeableProxy<M>
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ethers::core::types::Address,
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "BeaconUpgraded", abi = "BeaconUpgraded(address)")]
    pub struct BeaconUpgradedFilter {
        #[ethevent(indexed)]
        pub beacon: ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TransparentUpgradeableProxyEvents {
        AdminChangedFilter(AdminChangedFilter),
        BeaconUpgradedFilter(BeaconUpgradedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for TransparentUpgradeableProxyEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(TransparentUpgradeableProxyEvents::AdminChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = BeaconUpgradedFilter::decode_log(log) {
                return Ok(TransparentUpgradeableProxyEvents::BeaconUpgradedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(TransparentUpgradeableProxyEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for TransparentUpgradeableProxyEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TransparentUpgradeableProxyEvents::AdminChangedFilter(element) => element.fmt(f),
                TransparentUpgradeableProxyEvents::BeaconUpgradedFilter(element) => element.fmt(f),
                TransparentUpgradeableProxyEvents::UpgradedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `admin`function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `changeAdmin`function with signature `changeAdmin(address)` and selector `[143, 40, 57, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "changeAdmin", abi = "changeAdmin(address)")]
    pub struct ChangeAdminCall {
        pub new_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `implementation`function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    #[doc = "Container type for all input parameters for the `upgradeTo`function with signature `upgradeTo(address)` and selector `[54, 89, 207, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeToAndCall`function with signature `upgradeToAndCall(address,bytes)` and selector `[79, 30, 242, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TransparentUpgradeableProxyCalls {
        Admin(AdminCall),
        ChangeAdmin(ChangeAdminCall),
        Implementation(ImplementationCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ethers::core::abi::AbiDecode for TransparentUpgradeableProxyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TransparentUpgradeableProxyCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ChangeAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TransparentUpgradeableProxyCalls::ChangeAdmin(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TransparentUpgradeableProxyCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TransparentUpgradeableProxyCalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TransparentUpgradeableProxyCalls::UpgradeToAndCall(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TransparentUpgradeableProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TransparentUpgradeableProxyCalls::Admin(element) => element.encode(),
                TransparentUpgradeableProxyCalls::ChangeAdmin(element) => element.encode(),
                TransparentUpgradeableProxyCalls::Implementation(element) => element.encode(),
                TransparentUpgradeableProxyCalls::UpgradeTo(element) => element.encode(),
                TransparentUpgradeableProxyCalls::UpgradeToAndCall(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TransparentUpgradeableProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TransparentUpgradeableProxyCalls::Admin(element) => element.fmt(f),
                TransparentUpgradeableProxyCalls::ChangeAdmin(element) => element.fmt(f),
                TransparentUpgradeableProxyCalls::Implementation(element) => element.fmt(f),
                TransparentUpgradeableProxyCalls::UpgradeTo(element) => element.fmt(f),
                TransparentUpgradeableProxyCalls::UpgradeToAndCall(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AdminCall> for TransparentUpgradeableProxyCalls {
        fn from(var: AdminCall) -> Self {
            TransparentUpgradeableProxyCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ChangeAdminCall> for TransparentUpgradeableProxyCalls {
        fn from(var: ChangeAdminCall) -> Self {
            TransparentUpgradeableProxyCalls::ChangeAdmin(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for TransparentUpgradeableProxyCalls {
        fn from(var: ImplementationCall) -> Self {
            TransparentUpgradeableProxyCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for TransparentUpgradeableProxyCalls {
        fn from(var: UpgradeToCall) -> Self {
            TransparentUpgradeableProxyCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for TransparentUpgradeableProxyCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            TransparentUpgradeableProxyCalls::UpgradeToAndCall(var)
        }
    }
}
