pub use actions_tester::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod actions_tester {
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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseCallArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseClaimCollateralArgsTest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseCollateralTokenApprovalArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseExerciseArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseMintOptionArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseMintSpreadArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseNeutralizeArgsTest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ActionArgs\",\"name\":\"args\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ActionType\",\"name\":\"actionType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"secondaryAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"secondaryUint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseQTokenPermitArgsTest\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ACTIONSTESTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001657610655908161001c8239f35b600080fdfe604060808152600436101561001357600080fd5b6000803560e01c80631cc21e1514610449578063554fd854146104495780636482b5b8146103a257806367e68ebf146102f257806381e620231461021357806398f251a0146101d7578063b58b337b146101235763bc43519e1461007657600080fd5b3461011c5761008436610495565b60c081015160808180518101031261011f5760208101519283151580940361011c575060ff846100b861010096840161063a565b90608060608501519401519473ffffffffffffffffffffffffffffffffffffffff96878383015116976060830151169260a06080840151930151938151998a5260208a0152880152606087015260808601521660a084015260c083015260e0820152f35b80fd5b8280fd5b503461011c575061013336610495565b608081019182511561017a576060828101516020938401519451925173ffffffffffffffffffffffffffffffffffffffff9182168152941692840192909252604083015290f35b606490517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f416374696f6e733a2063616e6e6f74206d696e742030206f7074696f6e7300006044820152fd5b503461011c57506101e736610495565b608073ffffffffffffffffffffffffffffffffffffffff60208301511691015182519182526020820152f35b503461011c575061022336610495565b608081019182511561026f57602082810151928201519351915173ffffffffffffffffffffffffffffffffffffffff938416815292909316928201929092526040810191909152606090f35b608490517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602b60248201527f416374696f6e733a2063616e6e6f74206d696e742030206f7074696f6e73206660448201527f726f6d20737072656164730000000000000000000000000000000000000000006064820152fd5b503461011c579061030236610495565b60609160c073ffffffffffffffffffffffffffffffffffffffff8484015116920151938151948593845260209083828601528051809486015282915b84831061038a57505091601f91817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0941161037e575b5001168101030190f35b85828601015238610374565b8183018101518884018801528795509182019161033e565b509034610445576103b236610495565b60c081015160608180518101031261044157610100935060ff6103d76020830161063a565b6060858401519301519373ffffffffffffffffffffffffffffffffffffffff908160208201511696828183015116926060830151169060a06080840151930151938151998a5260208a0152880152606087015260808601521660a084015260c083015260e0820152f35b8380fd5b5080fd5b503461011c575061045936610495565b608060a082015191015182519182526020820152f35b359073ffffffffffffffffffffffffffffffffffffffff8216820361049057565b600080fd5b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc6020818301126104905760049081359267ffffffffffffffff918285116104905760e0908583030112610490576040519360e085018581108482111761060c576040528084013560088110156104905785526105146024820161046f565b60208601526105256044820161046f565b60408601526105366064820161046f565b60608601526084810135608086015260a481013560a086015260c4810135908382116104905701908060238301121561049057838201359280841161060c57604051947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f81601f8801160116860191868310908311176105de5750604052828452602483830101116104905781600092602460209301838601378301015260c082015290565b6041907f4e487b71000000000000000000000000000000000000000000000000000000006000525260246000fd5b6041857f4e487b71000000000000000000000000000000000000000000000000000000006000525260246000fd5b519060ff821682036104905756fea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
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
    #[doc = "Container type for all input parameters for the `parseCallArgsTest` function with signature `parseCallArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[103, 230, 142, 191]`"]
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
    #[doc = "Container type for all input parameters for the `parseClaimCollateralArgsTest` function with signature `parseClaimCollateralArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[85, 79, 216, 84]`"]
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
    #[doc = "Container type for all input parameters for the `parseCollateralTokenApprovalArgsTest` function with signature `parseCollateralTokenApprovalArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[188, 67, 81, 158]`"]
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
    #[doc = "Container type for all input parameters for the `parseExerciseArgsTest` function with signature `parseExerciseArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[152, 242, 81, 160]`"]
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
    #[doc = "Container type for all input parameters for the `parseMintOptionArgsTest` function with signature `parseMintOptionArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[181, 139, 51, 123]`"]
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
    #[doc = "Container type for all input parameters for the `parseMintSpreadArgsTest` function with signature `parseMintSpreadArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[129, 230, 32, 35]`"]
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
    #[doc = "Container type for all input parameters for the `parseNeutralizeArgsTest` function with signature `parseNeutralizeArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[28, 194, 30, 21]`"]
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
    #[doc = "Container type for all input parameters for the `parseQTokenPermitArgsTest` function with signature `parseQTokenPermitArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[100, 130, 181, 184]`"]
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
    #[doc = "Container type for all return fields from the `parseCallArgsTest` function with signature `parseCallArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[103, 230, 142, 191]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ParseCallArgsTestReturn(
        pub ethers::core::types::Address,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all return fields from the `parseClaimCollateralArgsTest` function with signature `parseClaimCollateralArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[85, 79, 216, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ParseClaimCollateralArgsTestReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `parseCollateralTokenApprovalArgsTest` function with signature `parseCollateralTokenApprovalArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[188, 67, 81, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ParseCollateralTokenApprovalArgsTestReturn(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub bool,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub u8,
        pub [u8; 32],
        pub [u8; 32],
    );
    #[doc = "Container type for all return fields from the `parseExerciseArgsTest` function with signature `parseExerciseArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[152, 242, 81, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ParseExerciseArgsTestReturn(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `parseMintOptionArgsTest` function with signature `parseMintOptionArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[181, 139, 51, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ParseMintOptionArgsTestReturn(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `parseMintSpreadArgsTest` function with signature `parseMintSpreadArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[129, 230, 32, 35]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ParseMintSpreadArgsTestReturn(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `parseNeutralizeArgsTest` function with signature `parseNeutralizeArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[28, 194, 30, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ParseNeutralizeArgsTestReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `parseQTokenPermitArgsTest` function with signature `parseQTokenPermitArgsTest((uint8,address,address,address,uint256,uint256,bytes))` and selector `[100, 130, 181, 184]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ParseQTokenPermitArgsTestReturn(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub u8,
        pub [u8; 32],
        pub [u8; 32],
    );
}
