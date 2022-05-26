pub use qtoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod qtoken_mod {
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
    #[doc = "QToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static QTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_controller\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"expiryTime\",\"outputs\":[{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isCall\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"_isCall\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"nameStr\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strikeAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_strikeAsset\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strikePrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_strikePrice\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"symbolStr\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"underlyingAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static QTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60808060405234610016576116be908161001c8239f35b600080fdfe60806040526004361015610013575b600080fd5b60003560e01c806306fdde03146101b7578063095ea7b3146101ae57806317d69bc8146101a557806318160ddd1461019c57806323b872dd14610193578063313ce5671461018a5780633644e5151461018157806340c10f19146101785780636d6364781461016f57806370a08231146101665780637158da7c1461015d5780637dc0d1d0146101545780637ecebe001461014b57806395d89b411461014257806399bc0aea146101395780639dc29fac14610130578063a9059cbb14610127578063c52987cf1461011e578063d505accf14610115578063dd62ed3e1461010c5763f77c47911461010457600080fd5b61000e611225565b5061000e61118f565b5061000e610f0f565b5061000e610e9f565b5061000e610dd8565b5061000e610cef565b5061000e610c7c565b5061000e610b10565b5061000e610aaa565b5061000e610a44565b5061000e6109de565b5061000e610978565b5061000e610903565b5061000e61080c565b5061000e6107ca565b5061000e610757565b5061000e610586565b5061000e610549565b5061000e6104d6565b5061000e610413565b5061000e610236565b919091602080825283519081818401526000945b828610610220575050601f817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0926040959611610213575b0116010190565b600085828601015261020c565b85810182015184870160400152948101946101d4565b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126103ca576102987ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b90604051916102a683611337565b600492838152602090818101926080368537845b67ffffffffffffffff811687811015610346578160051b908451111561031a57908185681fffffffffffffffe060019486013592168601015267ffffffffffffffff80911690811461030d575b016102ba565b61031561128b565b610307565b60248760328a7f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b87858888608088015160f81c9081604051947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f8301168601604052818652850191865afa156103a3576040518061039f84826101c0565b0390f35b90507f3a008ffa000000000000000000000000000000000000000000000000000000008152fd5b80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761044b6103cd565b73ffffffffffffffffffffffffffffffffffffffff602435913360005260026020528261049c8260406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b5560405192835216907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560203392a3602060405160018152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061011561053c7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560601c604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020600054604051908152f35b503461000e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576105be6103cd565b6105c66103f0565b907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6044359273ffffffffffffffffffffffffffffffffffffffff90610691828516948560005260026020528661064060406000203373ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81036106e6575b505073ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b61069c8682546112bb565b90556106c88173ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b8054860190556040519485521692602090a360405160018152602090f35b81811061074a575b036107423361071d8473ffffffffffffffffffffffffffffffffffffffff166000526002602052604060002090565b9073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b558638610669565b61075261128b565b6106ee565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206101006107bd7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560f81c604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610804611451565b604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff6108596103cd565b602435906108a06101696108947ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560601c3314611623565b7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000948593858554811981116108f6575b01855516938484526001825260408420818154019055604051908152a3604051f35b6108fe61128b565b6108d4565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206101486109697ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560f81c6040519015158152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff6109c56103cd565b1660005260016020526020604060002054604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061010161053c7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061012961053c7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff610af76103cd565b1660005260036020526020604060002054604051908152f35b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126103ca57610b727ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b9060405191610b8083611337565b6004928381526020908181019260809081368637855b67ffffffffffffffff811688811015610c24578160051b9085511115610bf857908186681fffffffffffffffe0866001958701013592168701015267ffffffffffffffff809116908114610beb575b01610b96565b610bf361128b565b610be5565b60248860328b7f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b888689898789015160f81c9081604051947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f8301168601604052818652850191865afa156103a3576040518061039f84826101c0565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061013d610ce27ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560a81c604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610d296103cd565b73ffffffffffffffffffffffffffffffffffffffff60243591610d796101696108947ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60009384938385526001825260408520818154818110610dcb575b039055808554038555604051908152a3604051f35b610dd361128b565b610db6565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610e106103cd565b73ffffffffffffffffffffffffffffffffffffffff602435913360005260016020526040600020838154818110610e92575b039055169081600052600160205260406000208181540190556040519081527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60203392a3602060405160018152f35b610e9a61128b565b610e42565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610149610f057ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b0135604051908152f35b503461000e5760e07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610f476103cd565b610f4f6103f0565b604435906064359060843560ff8116810361000e5760006020917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92594610f97428210156112d2565b6110c36110cf610fa5611451565b928a8a61104d610fd58373ffffffffffffffffffffffffffffffffffffffff166000526003602052604060002090565b938454946001860190556040519485938d8d8601968791959493909260a09360c08401977f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9855273ffffffffffffffffffffffffffffffffffffffff8092166020860152166040840152606083015260808201520152565b039161107f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe09384810183528261139e565b519020604051938491898301968790916042927f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201520190565b0390810183528261139e565b5190206040805191825260ff92909216602082015260a4359181019190915260c435606082015281805260809060015afa15611182575b60005190836111648261071d73ffffffffffffffffffffffffffffffffffffffff9561113e8782168015159081611176575b506113ec565b73ffffffffffffffffffffffffffffffffffffffff166000526002602052604060002090565b556040519384528116931691602090a3005b9050888c161438611138565b61118a6113df565b611106565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061121c6111cc6103cd565b73ffffffffffffffffffffffffffffffffffffffff6111e96103f0565b91166000526002835260406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b54604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061016961053c7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b507f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b8181106112c6570390565b6112ce61128b565b0390565b156112d957565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5045524d49545f444541444c494e455f455850495245440000000000000000006044820152fd5b60a0810190811067ffffffffffffffff82111761135357604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60c0810190811067ffffffffffffffff82111761135357604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761135357604052565b506040513d6000823e3d90fd5b156113f357565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f494e56414c49445f5349474e45520000000000000000000000000000000000006044820152fd5b6114827ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b9060405161148f81611337565b60049081815260209081810192608036853760005b67ffffffffffffffff811682811015611532578160051b908451111561150457908185681fffffffffffffffe06001948b013592168601015267ffffffffffffffff8091169081146114f7575b016114a4565b6114ff61128b565b6114f1565b6032837f4e487b71000000000000000000000000000000000000000000000000000000006000525260246000fd5b505092919394506080015160f81c9182604051927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f8301168401604052818452858401948591845afa156115fb575051902090604051908101917f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835260408201527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260a081526115f581611382565b51902090565b7f3a008ffa000000000000000000000000000000000000000000000000000000006000526000fd5b1561162a57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f51546f6b656e3a2063616c6c657220213d20636f6e74726f6c6c6572000000006044820152fdfea264697066735822122049c52008ec724b5b2f32ad2b65bf0d742797b748b27554ed9374641390b7ae3a64736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct QToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for QToken<M> {
        fn clone(&self) -> Self {
            QToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for QToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for QToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(QToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> QToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), QTOKEN_ABI.clone(), client).into()
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
                QTOKEN_ABI.clone(),
                QTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
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
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x9dc29fac) function"]
        pub fn burn(
            &self,
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (account, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `controller` (0xf77c4791) function"]
        pub fn controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([247, 124, 71, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expiryTime` (0x99bc0aea) function"]
        pub fn expiry_time(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([153, 188, 10, 234], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isCall` (0x6d636478) function"]
        pub fn is_call(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 99, 100, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (account, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracle` (0x7dc0d1d0) function"]
        pub fn oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `strikeAsset` (0x17d69bc8) function"]
        pub fn strike_asset(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([23, 214, 155, 200], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `strikePrice` (0xc52987cf) function"]
        pub fn strike_price(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([197, 41, 135, 207], ())
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
        #[doc = "Calls the contract's `underlyingAsset` (0x7158da7c) function"]
        pub fn underlying_asset(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([113, 88, 218, 124], ())
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, QTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for QToken<M> {
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
        pub amount: ethers::core::types::U256,
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
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum QTokenEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for QTokenEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(QTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(QTokenEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for QTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                QTokenEvents::ApprovalFilter(element) => element.fmt(f),
                QTokenEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR`function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
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
    pub struct AllowanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
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
    pub struct BalanceOfCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(address,uint256)` and selector `[157, 194, 159, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `controller`function with signature `controller()` and selector `[247, 124, 71, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "controller", abi = "controller()")]
    pub struct ControllerCall;
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
    #[doc = "Container type for all input parameters for the `expiryTime`function with signature `expiryTime()` and selector `[153, 188, 10, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expiryTime", abi = "expiryTime()")]
    pub struct ExpiryTimeCall;
    #[doc = "Container type for all input parameters for the `isCall`function with signature `isCall()` and selector `[109, 99, 100, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isCall", abi = "isCall()")]
    pub struct IsCallCall;
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `nonces`function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `oracle`function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    #[doc = "Container type for all input parameters for the `permit`function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `strikeAsset`function with signature `strikeAsset()` and selector `[23, 214, 155, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "strikeAsset", abi = "strikeAsset()")]
    pub struct StrikeAssetCall;
    #[doc = "Container type for all input parameters for the `strikePrice`function with signature `strikePrice()` and selector `[197, 41, 135, 207]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "strikePrice", abi = "strikePrice()")]
    pub struct StrikePriceCall;
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
    #[doc = "Container type for all input parameters for the `underlyingAsset`function with signature `underlyingAsset()` and selector `[113, 88, 218, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "underlyingAsset", abi = "underlyingAsset()")]
    pub struct UnderlyingAssetCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum QTokenCalls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Controller(ControllerCall),
        Decimals(DecimalsCall),
        ExpiryTime(ExpiryTimeCall),
        IsCall(IsCallCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Oracle(OracleCall),
        Permit(PermitCall),
        StrikeAsset(StrikeAssetCall),
        StrikePrice(StrikePriceCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        UnderlyingAsset(UnderlyingAssetCall),
    }
    impl ethers::core::abi::AbiDecode for QTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(QTokenCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <ControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::Controller(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <ExpiryTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::ExpiryTime(decoded));
            }
            if let Ok(decoded) = <IsCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::IsCall(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(QTokenCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(QTokenCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::Nonces(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::Oracle(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <StrikeAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::StrikeAsset(decoded));
            }
            if let Ok(decoded) =
                <StrikePriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::StrikePrice(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QTokenCalls::UnderlyingAsset(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for QTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                QTokenCalls::DomainSeparator(element) => element.encode(),
                QTokenCalls::Allowance(element) => element.encode(),
                QTokenCalls::Approve(element) => element.encode(),
                QTokenCalls::BalanceOf(element) => element.encode(),
                QTokenCalls::Burn(element) => element.encode(),
                QTokenCalls::Controller(element) => element.encode(),
                QTokenCalls::Decimals(element) => element.encode(),
                QTokenCalls::ExpiryTime(element) => element.encode(),
                QTokenCalls::IsCall(element) => element.encode(),
                QTokenCalls::Mint(element) => element.encode(),
                QTokenCalls::Name(element) => element.encode(),
                QTokenCalls::Nonces(element) => element.encode(),
                QTokenCalls::Oracle(element) => element.encode(),
                QTokenCalls::Permit(element) => element.encode(),
                QTokenCalls::StrikeAsset(element) => element.encode(),
                QTokenCalls::StrikePrice(element) => element.encode(),
                QTokenCalls::Symbol(element) => element.encode(),
                QTokenCalls::TotalSupply(element) => element.encode(),
                QTokenCalls::Transfer(element) => element.encode(),
                QTokenCalls::TransferFrom(element) => element.encode(),
                QTokenCalls::UnderlyingAsset(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for QTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                QTokenCalls::DomainSeparator(element) => element.fmt(f),
                QTokenCalls::Allowance(element) => element.fmt(f),
                QTokenCalls::Approve(element) => element.fmt(f),
                QTokenCalls::BalanceOf(element) => element.fmt(f),
                QTokenCalls::Burn(element) => element.fmt(f),
                QTokenCalls::Controller(element) => element.fmt(f),
                QTokenCalls::Decimals(element) => element.fmt(f),
                QTokenCalls::ExpiryTime(element) => element.fmt(f),
                QTokenCalls::IsCall(element) => element.fmt(f),
                QTokenCalls::Mint(element) => element.fmt(f),
                QTokenCalls::Name(element) => element.fmt(f),
                QTokenCalls::Nonces(element) => element.fmt(f),
                QTokenCalls::Oracle(element) => element.fmt(f),
                QTokenCalls::Permit(element) => element.fmt(f),
                QTokenCalls::StrikeAsset(element) => element.fmt(f),
                QTokenCalls::StrikePrice(element) => element.fmt(f),
                QTokenCalls::Symbol(element) => element.fmt(f),
                QTokenCalls::TotalSupply(element) => element.fmt(f),
                QTokenCalls::Transfer(element) => element.fmt(f),
                QTokenCalls::TransferFrom(element) => element.fmt(f),
                QTokenCalls::UnderlyingAsset(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for QTokenCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            QTokenCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for QTokenCalls {
        fn from(var: AllowanceCall) -> Self {
            QTokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for QTokenCalls {
        fn from(var: ApproveCall) -> Self {
            QTokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for QTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            QTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for QTokenCalls {
        fn from(var: BurnCall) -> Self {
            QTokenCalls::Burn(var)
        }
    }
    impl ::std::convert::From<ControllerCall> for QTokenCalls {
        fn from(var: ControllerCall) -> Self {
            QTokenCalls::Controller(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for QTokenCalls {
        fn from(var: DecimalsCall) -> Self {
            QTokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<ExpiryTimeCall> for QTokenCalls {
        fn from(var: ExpiryTimeCall) -> Self {
            QTokenCalls::ExpiryTime(var)
        }
    }
    impl ::std::convert::From<IsCallCall> for QTokenCalls {
        fn from(var: IsCallCall) -> Self {
            QTokenCalls::IsCall(var)
        }
    }
    impl ::std::convert::From<MintCall> for QTokenCalls {
        fn from(var: MintCall) -> Self {
            QTokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for QTokenCalls {
        fn from(var: NameCall) -> Self {
            QTokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for QTokenCalls {
        fn from(var: NoncesCall) -> Self {
            QTokenCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<OracleCall> for QTokenCalls {
        fn from(var: OracleCall) -> Self {
            QTokenCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<PermitCall> for QTokenCalls {
        fn from(var: PermitCall) -> Self {
            QTokenCalls::Permit(var)
        }
    }
    impl ::std::convert::From<StrikeAssetCall> for QTokenCalls {
        fn from(var: StrikeAssetCall) -> Self {
            QTokenCalls::StrikeAsset(var)
        }
    }
    impl ::std::convert::From<StrikePriceCall> for QTokenCalls {
        fn from(var: StrikePriceCall) -> Self {
            QTokenCalls::StrikePrice(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for QTokenCalls {
        fn from(var: SymbolCall) -> Self {
            QTokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for QTokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            QTokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for QTokenCalls {
        fn from(var: TransferCall) -> Self {
            QTokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for QTokenCalls {
        fn from(var: TransferFromCall) -> Self {
            QTokenCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnderlyingAssetCall> for QTokenCalls {
        fn from(var: UnderlyingAssetCall) -> Self {
            QTokenCalls::UnderlyingAsset(var)
        }
    }
}
