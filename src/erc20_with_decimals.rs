pub use erc20withdecimals_mod::*;
#[allow(clippy::too_many_arguments)]
mod erc20withdecimals_mod {
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
    #[doc = "ERC20WithDecimals was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ERC20WITHDECIMALS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ERC20WITHDECIMALS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052346200009857620012c0803803806200001d81620000b4565b928339810190606081830312620000985780516001600160401b03929083811162000098578162000050918401620000e9565b92602083015190811162000098576040916200006e918401620000e9565b9101519060ff82168203620000985762000088926200017d565b604051610e179081620004a98239f35b600080fd5b50634e487b7160e01b600052604160045260246000fd5b6040519190601f01601f191682016001600160401b03811183821017620000da57604052565b620000e46200009d565b604052565b81601f8201121562000098578051906001600160401b0382116200016d575b6020906200011f601f8401601f19168301620000b4565b93838552828483010111620000985782906000905b8383831062000154575050116200014a57505090565b6000918301015290565b8193508281939201015182828801015201839162000134565b620001776200009d565b62000108565b805190939291906001600160401b038111620002a5575b620001ac81620001a6600354620002b5565b620002f2565b602080601f83116001146200020c57509080620001ee9392620001fe969760009262000200575b50508160011b916000199060031b1c191617600355620003a4565b60ff1660ff196005541617600555565b565b015190503880620001d3565b6003600052601f198316969091907fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b926000905b8982106200028c575050918391600193620001ee9695620001fe999a1062000272575b505050811b01600355620003a4565b015160001960f88460031b161c1916905538808062000263565b8060018596829496860151815501950193019062000240565b620002af6200009d565b62000194565b90600182811c92168015620002e7575b6020831014620002d157565b634e487b7160e01b600052602260045260246000fd5b91607f1691620002c5565b601f8111620002ff575050565b6000906003825260208220906020601f850160051c8301941062000340575b601f0160051c01915b8281106200033457505050565b81815560010162000327565b90925082906200031e565b601f811162000358575050565b6000906004825260208220906020601f850160051c8301941062000399575b601f0160051c01915b8281106200038d57505050565b81815560010162000380565b909250829062000377565b80519091906001600160401b03811162000498575b620003d181620003cb600454620002b5565b6200034b565b602080601f831160011462000410575081929360009262000404575b50508160011b916000199060031b1c191617600455565b015190503880620003ed565b6004600052601f198316949091907f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b926000905b8782106200047f57505083600195961062000465575b505050811b01600455565b015160001960f88460031b161c191690553880806200045a565b8060018596829496860151815501950193019062000444565b620004a26200009d565b620003b956fe60806040526004361015610013575b600080fd5b60003560e01c806306fdde03146100ef578063095ea7b3146100e657806318160ddd146100dd57806323b872dd146100d4578063313ce567146100cb57806339509351146100c257806370a08231146100b957806395d89b41146100b0578063a457c2d7146100a7578063a9059cbb1461009e5763dd62ed3e1461009657600080fd5b61000e610847565b5061000e610802565b5061000e6106f6565b5061000e610614565b5061000e6105ae565b5061000e610529565b5061000e6104e9565b5061000e61039e565b5061000e610361565b5061000e610311565b5061000e61016e565b919091602080825283519081818401526000945b828610610158575050601f817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe092604095961161014b575b0116010190565b6000858286010152610144565b858101820151848701604001529481019461010c565b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102c85760405190806003549060019180831c928082169283156102be575b602092838610851461029157858852602088019490811561025757506001146101fe575b6101fa876101ee818903826108dd565b604051918291826100f8565b0390f35b600360005294509192917fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b5b83861061024657505050910190506101ee826101fa38806101de565b80548587015294820194810161022a565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001685525050500191506101ee9050826101fa38806101de565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526022600452fd5b93607f16936101ba565b80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761035661034c6102cb565b6024359033610c27565b602060405160018152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020600254604051908152f35b503461000e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576103d66102cb565b6103de6102ee565b6044359073ffffffffffffffffffffffffffffffffffffffff8316600052600160205261042f3360406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b54927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff840361046f575b6104639350610a9e565b60405160018152602090f35b82841061048b576104868361046395033383610c27565b610459565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e63650000006044820152fd5b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602060ff60055416604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576103566105646102cb565b3360005260016020526105a76024356105a18360406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b5461094d565b9033610c27565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff6105fb6102cb565b1660005260006020526020604060002054604051908152f35b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102c85760405190806004549060019180831c928082169283156106ec575b60209283861085146102915785885260208801949081156102575750600114610693576101fa876101ee818903826108dd565b600460005294509192917f8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b5b8386106106db57505050910190506101ee826101fa38806101de565b8054858701529482019481016106bf565b93607f1693610660565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761072e6102cb565b602435903360005260016020526107698160406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b549180831061077e5761046392039033610c27565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f7760448201527f207a65726f0000000000000000000000000000000000000000000000000000006064820152fd5b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761035661083d6102cb565b6024359033610a9e565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206108d46108846102cb565b73ffffffffffffffffffffffffffffffffffffffff6108a16102ee565b91166000526001835260406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b54604051908152f35b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761091e57604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b81198111610959570190565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b1561098f57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201527f65737300000000000000000000000000000000000000000000000000000000006064820152fd5b15610a1a57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e742065786365656473206260448201527f616c616e636500000000000000000000000000000000000000000000000000006064820152fd5b919073ffffffffffffffffffffffffffffffffffffffff90818416928315610ba357610b81827fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef94610b9e941696610af7881515610988565b610b5a84610b258373ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b54610b3282821015610a13565b039173ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b5573ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b610b8c82825461094d565b90556040519081529081906020820190565b0390a3565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f20616460448201527f64726573730000000000000000000000000000000000000000000000000000006064820152fd5b9073ffffffffffffffffffffffffffffffffffffffff91828116928315610d5e578216938415610cda5780610cc97f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92594610ca4610b9e9573ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b9073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b556040519081529081906020820190565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f20616464726560448201527f73730000000000000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a0000000000000000000000000000000000000000000000000000000008152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f2061646460448201527f72657373000000000000000000000000000000000000000000000000000000006064820152fdfea2646970667358221220630eb29db2a4e74cbb646e93c79e660231ddf9802517a7e282caf3d62072b57a64736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ERC20WithDecimals<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ERC20WithDecimals<M> {
        fn clone(&self) -> Self {
            ERC20WithDecimals(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ERC20WithDecimals<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ERC20WithDecimals<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC20WithDecimals))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ERC20WithDecimals<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERC20WITHDECIMALS_ABI.clone(), client)
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
                ERC20WITHDECIMALS_ABI.clone(),
                ERC20WITHDECIMALS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ERC20WithDecimalsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ERC20WithDecimals<M>
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC20WithDecimalsEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for ERC20WithDecimalsEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC20WithDecimalsEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC20WithDecimalsEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ERC20WithDecimalsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC20WithDecimalsEvents::ApprovalFilter(element) => element.fmt(f),
                ERC20WithDecimalsEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `decreaseAllowance`function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `increaseAllowance`function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC20WithDecimalsCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Name(NameCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for ERC20WithDecimalsCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ERC20WithDecimalsCalls::Name(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20WithDecimalsCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC20WithDecimalsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC20WithDecimalsCalls::Allowance(element) => element.encode(),
                ERC20WithDecimalsCalls::Approve(element) => element.encode(),
                ERC20WithDecimalsCalls::BalanceOf(element) => element.encode(),
                ERC20WithDecimalsCalls::Decimals(element) => element.encode(),
                ERC20WithDecimalsCalls::DecreaseAllowance(element) => element.encode(),
                ERC20WithDecimalsCalls::IncreaseAllowance(element) => element.encode(),
                ERC20WithDecimalsCalls::Name(element) => element.encode(),
                ERC20WithDecimalsCalls::Symbol(element) => element.encode(),
                ERC20WithDecimalsCalls::TotalSupply(element) => element.encode(),
                ERC20WithDecimalsCalls::Transfer(element) => element.encode(),
                ERC20WithDecimalsCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC20WithDecimalsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC20WithDecimalsCalls::Allowance(element) => element.fmt(f),
                ERC20WithDecimalsCalls::Approve(element) => element.fmt(f),
                ERC20WithDecimalsCalls::BalanceOf(element) => element.fmt(f),
                ERC20WithDecimalsCalls::Decimals(element) => element.fmt(f),
                ERC20WithDecimalsCalls::DecreaseAllowance(element) => element.fmt(f),
                ERC20WithDecimalsCalls::IncreaseAllowance(element) => element.fmt(f),
                ERC20WithDecimalsCalls::Name(element) => element.fmt(f),
                ERC20WithDecimalsCalls::Symbol(element) => element.fmt(f),
                ERC20WithDecimalsCalls::TotalSupply(element) => element.fmt(f),
                ERC20WithDecimalsCalls::Transfer(element) => element.fmt(f),
                ERC20WithDecimalsCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AllowanceCall> for ERC20WithDecimalsCalls {
        fn from(var: AllowanceCall) -> Self {
            ERC20WithDecimalsCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for ERC20WithDecimalsCalls {
        fn from(var: ApproveCall) -> Self {
            ERC20WithDecimalsCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ERC20WithDecimalsCalls {
        fn from(var: BalanceOfCall) -> Self {
            ERC20WithDecimalsCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for ERC20WithDecimalsCalls {
        fn from(var: DecimalsCall) -> Self {
            ERC20WithDecimalsCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for ERC20WithDecimalsCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            ERC20WithDecimalsCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for ERC20WithDecimalsCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            ERC20WithDecimalsCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<NameCall> for ERC20WithDecimalsCalls {
        fn from(var: NameCall) -> Self {
            ERC20WithDecimalsCalls::Name(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for ERC20WithDecimalsCalls {
        fn from(var: SymbolCall) -> Self {
            ERC20WithDecimalsCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for ERC20WithDecimalsCalls {
        fn from(var: TotalSupplyCall) -> Self {
            ERC20WithDecimalsCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for ERC20WithDecimalsCalls {
        fn from(var: TransferCall) -> Self {
            ERC20WithDecimalsCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for ERC20WithDecimalsCalls {
        fn from(var: TransferFromCall) -> Self {
            ERC20WithDecimalsCalls::TransferFrom(var)
        }
    }
}
