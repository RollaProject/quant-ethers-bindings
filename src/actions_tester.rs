pub use actionstester_mod::*;
#[allow(clippy::too_many_arguments)]
mod actionstester_mod {
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
    #[doc = "ActionsTester was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ACTIONSTESTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseCallArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseClaimCollateralArgsTest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseCollateralTokenApprovalArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseExerciseArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseMintOptionArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseMintSpreadArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseNeutralizeArgsTest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseQTokenPermitArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ACTIONSTESTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001657610751908161001c8239f35b600080fdfe60806040526004361015610013575b600080fd5b60003560e01c80631cc21e15146100b3578063554fd854146100aa5780636482b5b8146100a157806367e68ebf1461009857806381e620231461008f57806398f251a014610086578063b58b337b1461007d5763bc43519e1461007557600080fd5b61000e610617565b5061000e61055d565b5061000e610520565b5061000e610440565b5061000e61038f565b5061000e6102ed565b5061000e6102db565b3461000e576100d26100c436610210565b90608060a083015192015190565b60408051928352602083019190915290f35b0390f35b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040519060e0820182811067ffffffffffffffff82111761013857604052565b6101406100e8565b604052565b3590600882101561000e57565b359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b81601f8201121561000e5780359067ffffffffffffffff92838311610203575b604051937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f81601f8701160116850190858210908211176101f6575b6040528284526020838301011161000e57816000926020809301838601378301015290565b6101fe6100e8565b6101d1565b61020b6100e8565b610193565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc60208183011261000e576004359167ffffffffffffffff9182841161000e5760e090848303011261000e57610264610118565b9261027181600401610145565b845261027f60248201610152565b602085015261029060448201610152565b60408501526102a160648201610152565b60608501526084810135608085015260a481013560a085015260c481013592831161000e576102d39201600401610173565b60c082015290565b503461000e576100d26100c436610210565b503461000e576102fc36610210565b60c08101519060608280518101031261000e578161031f6020610100940161070d565b9060ff606060408301519201519273ffffffffffffffffffffffffffffffffffffffff8060208701511695816040820151169160608201511660a060808301519201519260405198895260208901526040880152606087015260808601521660a084015260c083015260e0820152f35b503461000e5761039e36610210565b606060c073ffffffffffffffffffffffffffffffffffffffff828401511692015191604051928391825260206040818401528151918260408501526000915b838310610428575050817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe092601f921161041b575b01168101030190f35b6000858286010152610412565b818301810151878401870152869450918201916103dd565b503461000e5761044f36610210565b608081019081511561049c576020818101516040928301519351835173ffffffffffffffffffffffffffffffffffffffff9283168152919094169181019190915290810191909152606090f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602b60248201527f416374696f6e733a2063616e6e6f74206d696e742030206f7074696f6e73206660448201527f726f6d20737072656164730000000000000000000000000000000000000000006064820152fd5b503461000e57604061053136610210565b608073ffffffffffffffffffffffffffffffffffffffff60208301511691015182519182526020820152f35b503461000e5761056c36610210565b60808101908151156105b95760608181015160209283015193516040805173ffffffffffffffffffffffffffffffffffffffff938416815292909516938201939093529283019190915290f35b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f416374696f6e733a2063616e6e6f74206d696e742030206f7074696f6e7300006044820152fd5b503461000e5761062636610210565b60c081015160808180518101031261000e57602081015190811515820361000e578061065760406100e4930161070d565b906080606082015191015191610684604087015173ffffffffffffffffffffffffffffffffffffffff1690565b946106a6606088015173ffffffffffffffffffffffffffffffffffffffff1690565b9660a06080820151910151916040519889988994909360ff9360e097939a99989561010088019b73ffffffffffffffffffffffffffffffffffffffff809216895216602088015215156040870152606086015260808501521660a083015260c08201520152565b519060ff8216820361000e5756fea2646970667358221220f52610c16594673a3b97c4f0503ccc89ca89e01495c0b73a7bea1e8d418c581b64736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ActionsTester<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ActionsTester<M> {
        fn clone(&self) -> Self {
            ActionsTester(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ActionsTester<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ActionsTester<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ActionsTester))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ActionsTester<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ACTIONSTESTER_ABI.clone(), client)
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
                ACTIONSTESTER_ABI.clone(),
                ACTIONSTESTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `parseCallArgsTest` (0x67e68ebf) function"]
        pub fn parse_call_args_test(
            &self,
            args: ActionArgs,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([103, 230, 142, 191], (args,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `parseClaimCollateralArgsTest` (0x554fd854) function"]
        pub fn parse_claim_collateral_args_test(
            &self,
            args: ActionArgs,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([85, 79, 216, 84], (args,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `parseCollateralTokenApprovalArgsTest` (0xbc43519e) function"]
        pub fn parse_collateral_token_approval_args_test(
            &self,
            args: ActionArgs,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                bool,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u8,
                [u8; 32],
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([188, 67, 81, 158], (args,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `parseExerciseArgsTest` (0x98f251a0) function"]
        pub fn parse_exercise_args_test(
            &self,
            args: ActionArgs,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, ethers::core::types::U256),
        > {
            self.0
                .method_hash([152, 242, 81, 160], (args,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `parseMintOptionArgsTest` (0xb58b337b) function"]
        pub fn parse_mint_option_args_test(
            &self,
            args: ActionArgs,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([181, 139, 51, 123], (args,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `parseMintSpreadArgsTest` (0x81e62023) function"]
        pub fn parse_mint_spread_args_test(
            &self,
            args: ActionArgs,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([129, 230, 32, 35], (args,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `parseNeutralizeArgsTest` (0x1cc21e15) function"]
        pub fn parse_neutralize_args_test(
            &self,
            args: ActionArgs,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([28, 194, 30, 21], (args,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `parseQTokenPermitArgsTest` (0x6482b5b8) function"]
        pub fn parse_q_token_permit_args_test(
            &self,
            args: ActionArgs,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u8,
                [u8; 32],
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([100, 130, 181, 184], (args,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ActionsTester<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `parseCallArgsTest`function with signature `parseCallArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[103, 230, 142, 191]`"]
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
        name = "parseCallArgsTest",
        abi = "parseCallArgsTest((uint8,address,address,address,uint256,uint256,bytes))"
    )]
    pub struct ParseCallArgsTestCall {
        pub args: ActionArgs,
    }
    #[doc = "Container type for all input parameters for the `parseClaimCollateralArgsTest`function with signature `parseClaimCollateralArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[85, 79, 216, 84]`"]
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
        name = "parseClaimCollateralArgsTest",
        abi = "parseClaimCollateralArgsTest((uint8,address,address,address,uint256,uint256,bytes))"
    )]
    pub struct ParseClaimCollateralArgsTestCall {
        pub args: ActionArgs,
    }
    #[doc = "Container type for all input parameters for the `parseCollateralTokenApprovalArgsTest`function with signature `parseCollateralTokenApprovalArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[188, 67, 81, 158]`"]
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
        name = "parseCollateralTokenApprovalArgsTest",
        abi = "parseCollateralTokenApprovalArgsTest((uint8,address,address,address,uint256,uint256,bytes))"
    )]
    pub struct ParseCollateralTokenApprovalArgsTestCall {
        pub args: ActionArgs,
    }
    #[doc = "Container type for all input parameters for the `parseExerciseArgsTest`function with signature `parseExerciseArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[152, 242, 81, 160]`"]
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
        name = "parseExerciseArgsTest",
        abi = "parseExerciseArgsTest((uint8,address,address,address,uint256,uint256,bytes))"
    )]
    pub struct ParseExerciseArgsTestCall {
        pub args: ActionArgs,
    }
    #[doc = "Container type for all input parameters for the `parseMintOptionArgsTest`function with signature `parseMintOptionArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[181, 139, 51, 123]`"]
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
        name = "parseMintOptionArgsTest",
        abi = "parseMintOptionArgsTest((uint8,address,address,address,uint256,uint256,bytes))"
    )]
    pub struct ParseMintOptionArgsTestCall {
        pub args: ActionArgs,
    }
    #[doc = "Container type for all input parameters for the `parseMintSpreadArgsTest`function with signature `parseMintSpreadArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[129, 230, 32, 35]`"]
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
        name = "parseMintSpreadArgsTest",
        abi = "parseMintSpreadArgsTest((uint8,address,address,address,uint256,uint256,bytes))"
    )]
    pub struct ParseMintSpreadArgsTestCall {
        pub args: ActionArgs,
    }
    #[doc = "Container type for all input parameters for the `parseNeutralizeArgsTest`function with signature `parseNeutralizeArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[28, 194, 30, 21]`"]
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
        name = "parseNeutralizeArgsTest",
        abi = "parseNeutralizeArgsTest((uint8,address,address,address,uint256,uint256,bytes))"
    )]
    pub struct ParseNeutralizeArgsTestCall {
        pub args: ActionArgs,
    }
    #[doc = "Container type for all input parameters for the `parseQTokenPermitArgsTest`function with signature `parseQTokenPermitArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[100, 130, 181, 184]`"]
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
        name = "parseQTokenPermitArgsTest",
        abi = "parseQTokenPermitArgsTest((uint8,address,address,address,uint256,uint256,bytes))"
    )]
    pub struct ParseQTokenPermitArgsTestCall {
        pub args: ActionArgs,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ActionsTesterCalls {
        ParseCallArgsTest(ParseCallArgsTestCall),
        ParseClaimCollateralArgsTest(ParseClaimCollateralArgsTestCall),
        ParseCollateralTokenApprovalArgsTest(ParseCollateralTokenApprovalArgsTestCall),
        ParseExerciseArgsTest(ParseExerciseArgsTestCall),
        ParseMintOptionArgsTest(ParseMintOptionArgsTestCall),
        ParseMintSpreadArgsTest(ParseMintSpreadArgsTestCall),
        ParseNeutralizeArgsTest(ParseNeutralizeArgsTestCall),
        ParseQTokenPermitArgsTest(ParseQTokenPermitArgsTestCall),
    }
    impl ethers::core::abi::AbiDecode for ActionsTesterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ParseCallArgsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActionsTesterCalls::ParseCallArgsTest(decoded));
            }
            if let Ok(decoded) =
                <ParseClaimCollateralArgsTestCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActionsTesterCalls::ParseClaimCollateralArgsTest(decoded));
            }
            if let Ok(decoded) =
                <ParseCollateralTokenApprovalArgsTestCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActionsTesterCalls::ParseCollateralTokenApprovalArgsTest(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ParseExerciseArgsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActionsTesterCalls::ParseExerciseArgsTest(decoded));
            }
            if let Ok(decoded) =
                <ParseMintOptionArgsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActionsTesterCalls::ParseMintOptionArgsTest(decoded));
            }
            if let Ok(decoded) =
                <ParseMintSpreadArgsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActionsTesterCalls::ParseMintSpreadArgsTest(decoded));
            }
            if let Ok(decoded) =
                <ParseNeutralizeArgsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ActionsTesterCalls::ParseNeutralizeArgsTest(decoded));
            }
            if let Ok(decoded) =
                <ParseQTokenPermitArgsTestCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ActionsTesterCalls::ParseQTokenPermitArgsTest(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ActionsTesterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ActionsTesterCalls::ParseCallArgsTest(element) => element.encode(),
                ActionsTesterCalls::ParseClaimCollateralArgsTest(element) => element.encode(),
                ActionsTesterCalls::ParseCollateralTokenApprovalArgsTest(element) => {
                    element.encode()
                }
                ActionsTesterCalls::ParseExerciseArgsTest(element) => element.encode(),
                ActionsTesterCalls::ParseMintOptionArgsTest(element) => element.encode(),
                ActionsTesterCalls::ParseMintSpreadArgsTest(element) => element.encode(),
                ActionsTesterCalls::ParseNeutralizeArgsTest(element) => element.encode(),
                ActionsTesterCalls::ParseQTokenPermitArgsTest(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ActionsTesterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ActionsTesterCalls::ParseCallArgsTest(element) => element.fmt(f),
                ActionsTesterCalls::ParseClaimCollateralArgsTest(element) => element.fmt(f),
                ActionsTesterCalls::ParseCollateralTokenApprovalArgsTest(element) => element.fmt(f),
                ActionsTesterCalls::ParseExerciseArgsTest(element) => element.fmt(f),
                ActionsTesterCalls::ParseMintOptionArgsTest(element) => element.fmt(f),
                ActionsTesterCalls::ParseMintSpreadArgsTest(element) => element.fmt(f),
                ActionsTesterCalls::ParseNeutralizeArgsTest(element) => element.fmt(f),
                ActionsTesterCalls::ParseQTokenPermitArgsTest(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ParseCallArgsTestCall> for ActionsTesterCalls {
        fn from(var: ParseCallArgsTestCall) -> Self {
            ActionsTesterCalls::ParseCallArgsTest(var)
        }
    }
    impl ::std::convert::From<ParseClaimCollateralArgsTestCall> for ActionsTesterCalls {
        fn from(var: ParseClaimCollateralArgsTestCall) -> Self {
            ActionsTesterCalls::ParseClaimCollateralArgsTest(var)
        }
    }
    impl ::std::convert::From<ParseCollateralTokenApprovalArgsTestCall> for ActionsTesterCalls {
        fn from(var: ParseCollateralTokenApprovalArgsTestCall) -> Self {
            ActionsTesterCalls::ParseCollateralTokenApprovalArgsTest(var)
        }
    }
    impl ::std::convert::From<ParseExerciseArgsTestCall> for ActionsTesterCalls {
        fn from(var: ParseExerciseArgsTestCall) -> Self {
            ActionsTesterCalls::ParseExerciseArgsTest(var)
        }
    }
    impl ::std::convert::From<ParseMintOptionArgsTestCall> for ActionsTesterCalls {
        fn from(var: ParseMintOptionArgsTestCall) -> Self {
            ActionsTesterCalls::ParseMintOptionArgsTest(var)
        }
    }
    impl ::std::convert::From<ParseMintSpreadArgsTestCall> for ActionsTesterCalls {
        fn from(var: ParseMintSpreadArgsTestCall) -> Self {
            ActionsTesterCalls::ParseMintSpreadArgsTest(var)
        }
    }
    impl ::std::convert::From<ParseNeutralizeArgsTestCall> for ActionsTesterCalls {
        fn from(var: ParseNeutralizeArgsTestCall) -> Self {
            ActionsTesterCalls::ParseNeutralizeArgsTest(var)
        }
    }
    impl ::std::convert::From<ParseQTokenPermitArgsTestCall> for ActionsTesterCalls {
        fn from(var: ParseQTokenPermitArgsTestCall) -> Self {
            ActionsTesterCalls::ParseQTokenPermitArgsTest(var)
        }
    }
}
