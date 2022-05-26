pub use externalqtoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod externalqtoken_mod {
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
    #[doc = "ExternalQToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static EXTERNALQTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_controller\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"expiryTime\",\"outputs\":[{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isCall\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"_isCall\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"nameStr\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permissionlessMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strikeAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_strikeAsset\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strikePrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_strikePrice\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"symbolStr\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"underlyingAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static EXTERNALQTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001657611778908161001c8239f35b600080fdfe60806040526004361015610013575b600080fd5b60003560e01c806306fdde03146101cb578063095ea7b3146101c257806316920f19146101b957806317d69bc8146101b057806318160ddd146101a757806323b872dd1461019e578063313ce567146101955780633644e5151461018c57806340c10f19146101835780636d6364781461017a57806370a08231146101715780637158da7c146101685780637dc0d1d01461015f5780637ecebe001461015657806395d89b411461014d57806399bc0aea146101445780639dc29fac1461013b578063a9059cbb14610132578063c52987cf14610129578063d505accf14610120578063dd62ed3e146101175763f77c47911461010f57600080fd5b61000e6112df565b5061000e611249565b5061000e610fc9565b5061000e610f59565b5061000e610e92565b5061000e610da9565b5061000e610d36565b5061000e610bca565b5061000e610b64565b5061000e610afe565b5061000e610a98565b5061000e610a32565b5061000e6109bd565b5061000e6108d4565b5061000e610892565b5061000e61081f565b5061000e61064e565b5061000e610611565b5061000e61059e565b5061000e6104ea565b5061000e610427565b5061000e61024a565b919091602080825283519081818401526000945b828610610234575050601f817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0926040959611610227575b0116010190565b6000858286010152610220565b85810182015184870160400152948101946101e8565b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126103de576102ac7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b90604051916102ba836113f1565b600492838152602090818101926080368537845b67ffffffffffffffff81168781101561035a578160051b908451111561032e57908185681fffffffffffffffe060019486013592168601015267ffffffffffffffff809116908114610321575b016102ce565b610329611345565b61031b565b60248760328a7f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b87858888608088015160f81c9081604051947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f8301168601604052818652850191865afa156103b757604051806103b384826101d4565b0390f35b90507f3a008ffa000000000000000000000000000000000000000000000000000000008152fd5b80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761045f6103e1565b73ffffffffffffffffffffffffffffffffffffffff60243591336000526002602052826104b08260406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b5560405192835216907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560203392a3602060405160018152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff6105376103e1565b602435907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6020600094859385855481198111610591575b01855516938484526001825260408420818154019055604051908152a3604051f35b610599611345565b61056f565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206101156106047ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560601c604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020600054604051908152f35b503461000e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576106866103e1565b61068e610404565b907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6044359273ffffffffffffffffffffffffffffffffffffffff90610759828516948560005260026020528661070860406000203373ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff81036107ae575b505073ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b610764868254611375565b90556107908173ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b8054860190556040519485521692602090a360405160018152602090f35b818110610812575b0361080a336107e58473ffffffffffffffffffffffffffffffffffffffff166000526002602052604060002090565b9073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b558638610731565b61081a611345565b6107b6565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206101006108857ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560f81c604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206108cc61150b565b604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff6109216103e1565b6024359061096861016961095c7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560601c33146116dd565b7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000948593858554811981116105915701855516938484526001825260408420818154019055604051908152a3604051f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610148610a237ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560f81c6040519015158152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff610a7f6103e1565b1660005260016020526020604060002054604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206101016106047ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206101296106047ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff610bb16103e1565b1660005260036020526020604060002054604051908152f35b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126103de57610c2c7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b9060405191610c3a836113f1565b6004928381526020908181019260809081368637855b67ffffffffffffffff811688811015610cde578160051b9085511115610cb257908186681fffffffffffffffe0866001958701013592168701015267ffffffffffffffff809116908114610ca5575b01610c50565b610cad611345565b610c9f565b60248860328b7f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b888689898789015160f81c9081604051947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f8301168601604052818652850191865afa156103b757604051806103b384826101d4565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061013d610d9c7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560a81c604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610de36103e1565b73ffffffffffffffffffffffffffffffffffffffff60243591610e3361016961095c7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60009384938385526001825260408520818154818110610e85575b039055808554038555604051908152a3604051f35b610e8d611345565b610e70565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610eca6103e1565b73ffffffffffffffffffffffffffffffffffffffff602435913360005260016020526040600020838154818110610f4c575b039055169081600052600160205260406000208181540190556040519081527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60203392a3602060405160018152f35b610f54611345565b610efc565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610149610fbf7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b0135604051908152f35b503461000e5760e07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576110016103e1565b611009610404565b604435906064359060843560ff8116810361000e5760006020917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925946110514282101561138c565b61117d61118961105f61150b565b928a8a61110761108f8373ffffffffffffffffffffffffffffffffffffffff166000526003602052604060002090565b938454946001860190556040519485938d8d8601968791959493909260a09360c08401977f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9855273ffffffffffffffffffffffffffffffffffffffff8092166020860152166040840152606083015260808201520152565b03916111397fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093848101835282611458565b519020604051938491898301968790916042927f19010000000000000000000000000000000000000000000000000000000000008352600283015260228201520190565b03908101835282611458565b5190206040805191825260ff92909216602082015260a4359181019190915260c435606082015281805260809060015afa1561123c575b600051908361121e826107e573ffffffffffffffffffffffffffffffffffffffff956111f88782168015159081611230575b506114a6565b73ffffffffffffffffffffffffffffffffffffffff166000526002602052604060002090565b556040519384528116931691602090a3005b9050888c1614386111f2565b611244611499565b6111c0565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206112d66112866103e1565b73ffffffffffffffffffffffffffffffffffffffff6112a3610404565b91166000526002835260406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b54604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206101696106047ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b507f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b818110611380570390565b611388611345565b0390565b1561139357565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5045524d49545f444541444c494e455f455850495245440000000000000000006044820152fd5b60a0810190811067ffffffffffffffff82111761140d57604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60c0810190811067ffffffffffffffff82111761140d57604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761140d57604052565b506040513d6000823e3d90fd5b156114ad57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f494e56414c49445f5349474e45520000000000000000000000000000000000006044820152fd5b61153c7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b90604051611549816113f1565b60049081815260209081810192608036853760005b67ffffffffffffffff8116828110156115ec578160051b90845111156115be57908185681fffffffffffffffe06001948b013592168601015267ffffffffffffffff8091169081146115b1575b0161155e565b6115b9611345565b6115ab565b6032837f4e487b71000000000000000000000000000000000000000000000000000000006000525260246000fd5b505092919394506080015160f81c9182604051927fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f8301168401604052818452858401948591845afa156116b5575051902090604051908101917f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835260408201527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260a081526116af8161143c565b51902090565b7f3a008ffa000000000000000000000000000000000000000000000000000000006000526000fd5b156116e457565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f51546f6b656e3a2063616c6c657220213d20636f6e74726f6c6c6572000000006044820152fdfea2646970667358221220950a3d1486f7c6e4e0072f7b695936d4e59fdcd31c5a3e4358078b763dd9cfb464736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ExternalQToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ExternalQToken<M> {
        fn clone(&self) -> Self {
            ExternalQToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ExternalQToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ExternalQToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ExternalQToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ExternalQToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), EXTERNALQTOKEN_ABI.clone(), client)
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
                EXTERNALQTOKEN_ABI.clone(),
                EXTERNALQTOKEN_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `permissionlessMint` (0x16920f19) function"]
        pub fn permissionless_mint(
            &self,
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 146, 15, 25], (account, amount))
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ExternalQTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ExternalQToken<M> {
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
    pub enum ExternalQTokenEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for ExternalQTokenEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ExternalQTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ExternalQTokenEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ExternalQTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ExternalQTokenEvents::ApprovalFilter(element) => element.fmt(f),
                ExternalQTokenEvents::TransferFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `permissionlessMint`function with signature `permissionlessMint(address,uint256)` and selector `[22, 146, 15, 25]`"]
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
        name = "permissionlessMint",
        abi = "permissionlessMint(address,uint256)"
    )]
    pub struct PermissionlessMintCall {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
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
    pub enum ExternalQTokenCalls {
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
        PermissionlessMint(PermissionlessMintCall),
        Permit(PermitCall),
        StrikeAsset(StrikeAssetCall),
        StrikePrice(StrikePriceCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        UnderlyingAsset(UnderlyingAssetCall),
    }
    impl ethers::core::abi::AbiDecode for ExternalQTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ExternalQTokenCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <ControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::Controller(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <ExpiryTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::ExpiryTime(decoded));
            }
            if let Ok(decoded) = <IsCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::IsCall(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ExternalQTokenCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ExternalQTokenCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::Nonces(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <PermissionlessMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::PermissionlessMint(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <StrikeAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::StrikeAsset(decoded));
            }
            if let Ok(decoded) =
                <StrikePriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::StrikePrice(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExternalQTokenCalls::UnderlyingAsset(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ExternalQTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ExternalQTokenCalls::DomainSeparator(element) => element.encode(),
                ExternalQTokenCalls::Allowance(element) => element.encode(),
                ExternalQTokenCalls::Approve(element) => element.encode(),
                ExternalQTokenCalls::BalanceOf(element) => element.encode(),
                ExternalQTokenCalls::Burn(element) => element.encode(),
                ExternalQTokenCalls::Controller(element) => element.encode(),
                ExternalQTokenCalls::Decimals(element) => element.encode(),
                ExternalQTokenCalls::ExpiryTime(element) => element.encode(),
                ExternalQTokenCalls::IsCall(element) => element.encode(),
                ExternalQTokenCalls::Mint(element) => element.encode(),
                ExternalQTokenCalls::Name(element) => element.encode(),
                ExternalQTokenCalls::Nonces(element) => element.encode(),
                ExternalQTokenCalls::Oracle(element) => element.encode(),
                ExternalQTokenCalls::PermissionlessMint(element) => element.encode(),
                ExternalQTokenCalls::Permit(element) => element.encode(),
                ExternalQTokenCalls::StrikeAsset(element) => element.encode(),
                ExternalQTokenCalls::StrikePrice(element) => element.encode(),
                ExternalQTokenCalls::Symbol(element) => element.encode(),
                ExternalQTokenCalls::TotalSupply(element) => element.encode(),
                ExternalQTokenCalls::Transfer(element) => element.encode(),
                ExternalQTokenCalls::TransferFrom(element) => element.encode(),
                ExternalQTokenCalls::UnderlyingAsset(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ExternalQTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ExternalQTokenCalls::DomainSeparator(element) => element.fmt(f),
                ExternalQTokenCalls::Allowance(element) => element.fmt(f),
                ExternalQTokenCalls::Approve(element) => element.fmt(f),
                ExternalQTokenCalls::BalanceOf(element) => element.fmt(f),
                ExternalQTokenCalls::Burn(element) => element.fmt(f),
                ExternalQTokenCalls::Controller(element) => element.fmt(f),
                ExternalQTokenCalls::Decimals(element) => element.fmt(f),
                ExternalQTokenCalls::ExpiryTime(element) => element.fmt(f),
                ExternalQTokenCalls::IsCall(element) => element.fmt(f),
                ExternalQTokenCalls::Mint(element) => element.fmt(f),
                ExternalQTokenCalls::Name(element) => element.fmt(f),
                ExternalQTokenCalls::Nonces(element) => element.fmt(f),
                ExternalQTokenCalls::Oracle(element) => element.fmt(f),
                ExternalQTokenCalls::PermissionlessMint(element) => element.fmt(f),
                ExternalQTokenCalls::Permit(element) => element.fmt(f),
                ExternalQTokenCalls::StrikeAsset(element) => element.fmt(f),
                ExternalQTokenCalls::StrikePrice(element) => element.fmt(f),
                ExternalQTokenCalls::Symbol(element) => element.fmt(f),
                ExternalQTokenCalls::TotalSupply(element) => element.fmt(f),
                ExternalQTokenCalls::Transfer(element) => element.fmt(f),
                ExternalQTokenCalls::TransferFrom(element) => element.fmt(f),
                ExternalQTokenCalls::UnderlyingAsset(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for ExternalQTokenCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            ExternalQTokenCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for ExternalQTokenCalls {
        fn from(var: AllowanceCall) -> Self {
            ExternalQTokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for ExternalQTokenCalls {
        fn from(var: ApproveCall) -> Self {
            ExternalQTokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ExternalQTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            ExternalQTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for ExternalQTokenCalls {
        fn from(var: BurnCall) -> Self {
            ExternalQTokenCalls::Burn(var)
        }
    }
    impl ::std::convert::From<ControllerCall> for ExternalQTokenCalls {
        fn from(var: ControllerCall) -> Self {
            ExternalQTokenCalls::Controller(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for ExternalQTokenCalls {
        fn from(var: DecimalsCall) -> Self {
            ExternalQTokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<ExpiryTimeCall> for ExternalQTokenCalls {
        fn from(var: ExpiryTimeCall) -> Self {
            ExternalQTokenCalls::ExpiryTime(var)
        }
    }
    impl ::std::convert::From<IsCallCall> for ExternalQTokenCalls {
        fn from(var: IsCallCall) -> Self {
            ExternalQTokenCalls::IsCall(var)
        }
    }
    impl ::std::convert::From<MintCall> for ExternalQTokenCalls {
        fn from(var: MintCall) -> Self {
            ExternalQTokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for ExternalQTokenCalls {
        fn from(var: NameCall) -> Self {
            ExternalQTokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for ExternalQTokenCalls {
        fn from(var: NoncesCall) -> Self {
            ExternalQTokenCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<OracleCall> for ExternalQTokenCalls {
        fn from(var: OracleCall) -> Self {
            ExternalQTokenCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<PermissionlessMintCall> for ExternalQTokenCalls {
        fn from(var: PermissionlessMintCall) -> Self {
            ExternalQTokenCalls::PermissionlessMint(var)
        }
    }
    impl ::std::convert::From<PermitCall> for ExternalQTokenCalls {
        fn from(var: PermitCall) -> Self {
            ExternalQTokenCalls::Permit(var)
        }
    }
    impl ::std::convert::From<StrikeAssetCall> for ExternalQTokenCalls {
        fn from(var: StrikeAssetCall) -> Self {
            ExternalQTokenCalls::StrikeAsset(var)
        }
    }
    impl ::std::convert::From<StrikePriceCall> for ExternalQTokenCalls {
        fn from(var: StrikePriceCall) -> Self {
            ExternalQTokenCalls::StrikePrice(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for ExternalQTokenCalls {
        fn from(var: SymbolCall) -> Self {
            ExternalQTokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for ExternalQTokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            ExternalQTokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for ExternalQTokenCalls {
        fn from(var: TransferCall) -> Self {
            ExternalQTokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for ExternalQTokenCalls {
        fn from(var: TransferFromCall) -> Self {
            ExternalQTokenCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnderlyingAssetCall> for ExternalQTokenCalls {
        fn from(var: UnderlyingAssetCall) -> Self {
            ExternalQTokenCalls::UnderlyingAsset(var)
        }
    }
}
