pub use q_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod q_token {
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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_controller\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"expiryTime\",\"outputs\":[{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isCall\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"_isCall\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"nameStr\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strikeAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_strikeAsset\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strikePrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_strikePrice\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"symbolStr\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"underlyingAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static QTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60808060405234610016576110fc908161001c8239f35b600080fdfe608060408181526004918236101561001657600080fd5b600092833560e01c91826306fdde0314610df557508163095ea7b314610d5857816317d69bc814610cf157816318160ddd14610cb657816323b872dd14610b9b578163313ce56714610b285781633644e51514610ae657816340c10f19146109e15781636d6364781461096c57816370a082311461090a5781637158da7c146108a35781637dc0d1d01461083c5781637ecebe00146107da57816395d89b411461076457816399bc0aea146106f15781639dc29fac14610611578163a9059cbb14610564578163c52987cf146104f4578163d505accf146101f457508063dd62ed3e146101815763f77c47911461010c57600080fd5b3461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d576020906101696101717ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560601c9051908152f35b5080fd5b503461017d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d57806020926101bc610ead565b6101c4610ed5565b73ffffffffffffffffffffffffffffffffffffffff91821683526002865283832091168252845220549051908152f35b9050346104f05760e07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104f05761022d610ead565b610235610ed5565b604435606435936084359360ff85168095036104ec5742861061048f578661025b610f32565b9473ffffffffffffffffffffffffffffffffffffffff80931696878b5260209660038852838c20998a549a60018c019055858551948b8b8701977f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98952870152169a8b606086015288608086015260a085015260c084015260c0835260e0830167ffffffffffffffff948482108683111761046357818d52845190206101008501927f19010000000000000000000000000000000000000000000000000000000000008452610102860152610122850152604281526101608401948186109086111761043757848c52519020835261018082015260a4356101a082015260c4356101c0909101528880528490899060809060015afa1561042d578751169081151580610424575b156103c85750907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9259291875260028252858720858852825280868820558551908152a351f35b606490848851917f08c379a0000000000000000000000000000000000000000000000000000000008352820152600e60248201527f494e56414c49445f5349474e45520000000000000000000000000000000000006044820152fd5b50848214610382565b86513d89823e3d90fd5b60248d6041897f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b60248e60418a7f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b60648260208951917f08c379a0000000000000000000000000000000000000000000000000000000008352820152601760248201527f5045524d49545f444541444c494e455f455850495245440000000000000000006044820152fd5b8780fd5b8280fd5b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d5760209061014961055b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b01359051908152f35b50503461017d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d5760209161059f610ead565b8273ffffffffffffffffffffffffffffffffffffffff60243592338552600187528285206105ce858254610ef8565b90551692838152600186522081815401905582519081527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef843392a35160018152f35b50503461017d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d578161064a610ead565b7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef602073ffffffffffffffffffffffffffffffffffffffff602435936106c96101696106bd7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560601c331461108a565b1692838552600182528585206106e0828254610ef8565b90558085540385558551908152a351f35b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d5760209061013d6107587ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560a81c9051908152f35b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d5751806107d660de367ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe81013560f01c90039081013560f81c90607e0183610e67565b0390f35b50503461017d5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d578060209273ffffffffffffffffffffffffffffffffffffffff61082c610ead565b1681526003845220549051908152f35b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d576020906101296101717ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d576020906101016101717ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b50503461017d5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d578060209273ffffffffffffffffffffffffffffffffffffffff61095c610ead565b1681526001845220549051908152f35b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d576020906101486109d37ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560f81c90519015158152f35b9050346104f057817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104f057610a19610ead565b60243591610a546101696106bd7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b84549083198211610aba5750602073ffffffffffffffffffffffffffffffffffffffff8693857fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef940185551693848452600182528584208181540190558551908152a351f35b8560116024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d57602090610b21610f32565b9051908152f35b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d57602090610100610b8f7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b013560f81c9051908152f35b50503461017d5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d57610bd4610ead565b91610bdd610ed5565b7fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6044359173ffffffffffffffffffffffffffffffffffffffff8096169283855285602097889360028552828820338952855282882054847fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203610c93575b505086885260018552828820610c74858254610ef8565b9055169586815260018452208181540190558551908152a35160018152f35b610c9c91610ef8565b87895260028652838920338a528652838920553884610c5d565b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d57602091549051908152f35b50503461017d57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d576020906101156101717ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c36030190565b50503461017d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261017d5760209181610d94610ead565b916024359182913381526002875273ffffffffffffffffffffffffffffffffffffffff8282209516948582528752205582519081527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925843392a35160018152f35b8434610e6457807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610e645750806107d67ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c360390605e82013560f81c910183610e67565b80fd5b90601f836040947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093602086528160208701528686013760008582860101520116010190565b6004359073ffffffffffffffffffffffffffffffffffffffff82168203610ed057565b600080fd5b6024359073ffffffffffffffffffffffffffffffffffffffff82168203610ed057565b818110610f03570390565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8036013560f01c360390605e82013560f81c9067ffffffffffffffff9283831161105b57604051917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f81601f870116011683018381108682111761105b57604052838352602083019336828285010111610ed0578060209260009401863783010152519020906040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f845260408301527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608301524660808301523060a083015260a0825260c08201908282109082111761105b5760405251902090565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b1561109157565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601c60248201527f51546f6b656e3a2063616c6c657220213d20636f6e74726f6c6c6572000000006044820152fdfea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
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
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
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
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
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
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
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
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
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
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,uint256)` and selector `[157, 194, 159, 172]`"]
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
    #[doc = "Container type for all input parameters for the `controller` function with signature `controller()` and selector `[247, 124, 71, 145]`"]
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
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
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
    #[doc = "Container type for all input parameters for the `expiryTime` function with signature `expiryTime()` and selector `[153, 188, 10, 234]`"]
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
    #[doc = "Container type for all input parameters for the `isCall` function with signature `isCall()` and selector `[109, 99, 100, 120]`"]
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
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
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
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
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
    #[doc = "Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
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
    #[doc = "Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
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
    #[doc = "Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
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
    #[doc = "Container type for all input parameters for the `strikeAsset` function with signature `strikeAsset()` and selector `[23, 214, 155, 200]`"]
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
    #[doc = "Container type for all input parameters for the `strikePrice` function with signature `strikePrice()` and selector `[197, 41, 135, 207]`"]
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
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
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
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
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
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
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
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
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
    #[doc = "Container type for all input parameters for the `underlyingAsset` function with signature `underlyingAsset()` and selector `[113, 88, 218, 124]`"]
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
    #[doc = "Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `controller` function with signature `controller()` and selector `[247, 124, 71, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ControllerReturn {
        pub controller: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `expiryTime` function with signature `expiryTime()` and selector `[153, 188, 10, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ExpiryTimeReturn {
        pub expiry_time: u128,
    }
    #[doc = "Container type for all return fields from the `isCall` function with signature `isCall()` and selector `[109, 99, 100, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsCallReturn {
        pub is_call: bool,
    }
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NameReturn {
        pub name_str: String,
    }
    #[doc = "Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NoncesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OracleReturn {
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `strikeAsset` function with signature `strikeAsset()` and selector `[23, 214, 155, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct StrikeAssetReturn {
        pub strike_asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `strikePrice` function with signature `strikePrice()` and selector `[197, 41, 135, 207]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct StrikePriceReturn {
        pub strike_price: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SymbolReturn {
        pub symbol_str: String,
    }
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferFromReturn(pub bool);
    #[doc = "Container type for all return fields from the `underlyingAsset` function with signature `underlyingAsset()` and selector `[113, 88, 218, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UnderlyingAssetReturn {
        pub underlying_asset: ethers::core::types::Address,
    }
}
