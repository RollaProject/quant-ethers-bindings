pub use mock_aggregator_proxy::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_aggregator_proxy {
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
    #[doc = "MockAggregatorProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKAGGREGATORPROXY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"current\",\"type\":\"int256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AnswerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"startedBy\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRound\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"accessController\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"aggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestAnswerValue\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRound\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundDataValue\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundValue\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestTimestampValue\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"phaseAggregators\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"phaseId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"proposedAggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"proposedGetRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"proposedLatestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roundIdAnswers\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roundTimestamps\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_latestAnswer\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLatestAnswer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_latestRound\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLatestRound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct MockAggregatorProxy.RoundData\",\"name\":\"_latestRoundData\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLatestRoundData\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_latestTimestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLatestTimestamp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct MockAggregatorProxy.RoundData\",\"name\":\"_roundData\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRoundData\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_answer\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRoundIdAnswer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_round\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_timestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTimestamp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKAGGREGATORPROXY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001657610b50908161001c8239f35b600080fdfe608060409080825260049081361015610018575b600080fd5b600090813560e01c9081630212dcae14610a9c5750806304ea97b014610a635780630b8f1e5514610500578063245a7bfc1461026e578063313ce56714610a29578063369be162146109e657806350d25bcd1461027a57806354fd4d50146109ad57806358303b10146109ad5780636001ac531461093f578063668a0f02146109025780637284e416146107e25780637eee1d95146107a55780638205bf6a146107a55780638da5cb5b1461026e5780638f6b4d911461074b5780639a6fc8f5146106345780639b440660146105805780639f51c50214610547578063b5ab58dc14610500578063b633620c146104ba578063bb946729146104ba578063bc43cbaf1461026e578063bc81bae814610419578063c1597304146103d3578063d160c7a914610391578063df4a00eb146102f8578063df61c3e2146102bb578063e44d846d1461027a578063e8c4be301461026e578063ea009699146101fd5763feaf968c1461018657600080fd5b346101fa57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101fa57506003549054600554600654600754945169ffffffffffffffffffff94851681526020810193909352604083019190915260608201529116608082015260a090f35b0390f35b80fd5b50346101fa57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101fa57506003549054600554600654600754945169ffffffffffffffffffff94851681526020810193909352604083019190915260608201529116608082015260a090f35b50505050610013610ad6565b5082346102b757817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102b7576020906008549051908152f35b5080fd5b509190346102f45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f45735600a5551f35b8280fd5b509190346102f45760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f45769ffffffffffffffffffff8061033e610b11565b16917fffffffffffffffffffffffffffffffffffffffffffff00000000000000000000928360035416176003556024359055604435600555606435600655610384610b2a565b1690600754161760075551f35b509190346102f457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f457358252816020526024358183205551f35b509190346102f45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f4573561ffff8116036102b75751908152602090f35b5091346102f45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f45781359269ffffffffffffffffffff938481168091036102b75781526002602081815291839020805460018201549282015460038301549690920154945169ffffffffffffffffffff918816821681529384019290925260408301526060820193909352921616608082015260a090f35b509190346102f45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f457602092829135815280845220549051908152f35b509190346102f45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f45760209282913581526001845220549051908152f35b509190346102f45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f4573560095551f35b509190346102f45760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f45769ffffffffffffffffffff90816105c7610b11565b168452600260205282842090826105dc610b11565b16917fffffffffffffffffffffffffffffffffffffffffffff0000000000000000000092838254161781556024356001820155604435600282015560643560038201550191610629610b2a565b169082541617905551f35b5082346102b75760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102b75782359269ffffffffffffffffffff9384811680910361074757835260026020528183209180519360a085019085821067ffffffffffffffff83111761071b575091816080936101f6959352868354169586815260018401549788602083015260028501549384848401526003860154958660608501520154169485910152519586958693608093969591929660a086019769ffffffffffffffffffff809516875260208701526040860152606085015216910152565b806041857f4e487b71000000000000000000000000000000000000000000000000000000006024945252fd5b8380fd5b8382346101fa57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101fa575051806101f681906000608060a08401938281528260208201528260408201528260608201520152565b5082346102b757817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102b7576020906009549051908152f35b508290346101fa57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101fa578151928284019084821067ffffffffffffffff8311176108d657508293919352600381526020907f2e2e2e0000000000000000000000000000000000000000000000000000000000828201528251938285938452825192838286015282915b8483106108be57505091601f91817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe094116108b2575b5001168101030190f35b858286010152866108a8565b81830181015188840188015287955091820191610872565b8260416024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b5082346102b757817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102b757602090600a549051908152f35b5090346102b75760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102b7573569ffffffffffffffffffff8116036101fa575051806101f681906000608060a08401938281528260208201528260408201528260608201520152565b5082346102b757817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102b75751908152602090f35b509190346102f457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f45735825260016020526024358183205551f35b5082346102b757817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102b7576020905160088152f35b509190346102f45760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102f4573560085551f35b9050346102b757817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102b757602090600a548152f35b50346100135760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261001357602060405160008152f35b60043569ffffffffffffffffffff811681036100135790565b60843569ffffffffffffffffffff81168103610013579056fea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
        });
    pub struct MockAggregatorProxy<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockAggregatorProxy<M> {
        fn clone(&self) -> Self {
            MockAggregatorProxy(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockAggregatorProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockAggregatorProxy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockAggregatorProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockAggregatorProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKAGGREGATORPROXY_ABI.clone(), client)
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
                MOCKAGGREGATORPROXY_ABI.clone(),
                MOCKAGGREGATORPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `accessController` (0xbc43cbaf) function"]
        pub fn access_controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([188, 67, 203, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `aggregator` (0x245a7bfc) function"]
        pub fn aggregator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([36, 90, 123, 252], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `description` (0x7284e416) function"]
        pub fn description(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAnswer` (0xb5ab58dc) function"]
        pub fn get_answer(
            &self,
            round_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([181, 171, 88, 220], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoundData` (0x9a6fc8f5) function"]
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTimestamp` (0xb633620c) function"]
        pub fn get_timestamp(
            &self,
            round_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 51, 98, 12], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestAnswer` (0x50d25bcd) function"]
        pub fn latest_answer(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([80, 210, 91, 205], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestAnswerValue` (0xe44d846d) function"]
        pub fn latest_answer_value(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([228, 77, 132, 109], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRound` (0x668a0f02) function"]
        pub fn latest_round(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([102, 138, 15, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRoundData` (0xfeaf968c) function"]
        pub fn latest_round_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRoundDataValue` (0xea009699) function"]
        pub fn latest_round_data_value(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([234, 0, 150, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRoundValue` (0x0212dcae) function"]
        pub fn latest_round_value(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([2, 18, 220, 174], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestTimestamp` (0x8205bf6a) function"]
        pub fn latest_timestamp(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([130, 5, 191, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestTimestampValue` (0x7eee1d95) function"]
        pub fn latest_timestamp_value(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 238, 29, 149], ())
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
        #[doc = "Calls the contract's `phaseAggregators` (0xc1597304) function"]
        pub fn phase_aggregators(
            &self,
            p0: u16,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([193, 89, 115, 4], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `phaseId` (0x58303b10) function"]
        pub fn phase_id(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([88, 48, 59, 16], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposedAggregator` (0xe8c4be30) function"]
        pub fn proposed_aggregator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([232, 196, 190, 48], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposedGetRoundData` (0x6001ac53) function"]
        pub fn proposed_get_round_data(
            &self,
            p0: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([96, 1, 172, 83], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposedLatestRoundData` (0x8f6b4d91) function"]
        pub fn proposed_latest_round_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([143, 107, 77, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roundData` (0xbc81bae8) function"]
        pub fn round_data(
            &self,
            p0: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([188, 129, 186, 232], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roundIdAnswers` (0x0b8f1e55) function"]
        pub fn round_id_answers(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([11, 143, 30, 85], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roundTimestamps` (0xbb946729) function"]
        pub fn round_timestamps(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([187, 148, 103, 41], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLatestAnswer` (0x04ea97b0) function"]
        pub fn set_latest_answer(
            &self,
            latest_answer: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 234, 151, 176], latest_answer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLatestRound` (0xdf61c3e2) function"]
        pub fn set_latest_round(
            &self,
            latest_round: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 97, 195, 226], latest_round)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLatestRoundData` (0xdf4a00eb) function"]
        pub fn set_latest_round_data(
            &self,
            latest_round_data: RoundData,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 74, 0, 235], (latest_round_data,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLatestTimestamp` (0x9f51c502) function"]
        pub fn set_latest_timestamp(
            &self,
            latest_timestamp: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 81, 197, 2], latest_timestamp)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRoundData` (0x9b440660) function"]
        pub fn set_round_data(
            &self,
            round_data: RoundData,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 68, 6, 96], (round_data,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRoundIdAnswer` (0x369be162) function"]
        pub fn set_round_id_answer(
            &self,
            round_id: ethers::core::types::U256,
            answer: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 155, 225, 98], (round_id, answer))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTimestamp` (0xd160c7a9) function"]
        pub fn set_timestamp(
            &self,
            round: ethers::core::types::U256,
            timestamp: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 96, 199, 169], (round, timestamp))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AnswerUpdated` event"]
        pub fn answer_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AnswerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewRound` event"]
        pub fn new_round_filter(&self) -> ethers::contract::builders::Event<M, NewRoundFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferRequested` event"]
        pub fn ownership_transfer_requested_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferRequestedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MockAggregatorProxyEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockAggregatorProxy<M>
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
    #[ethevent(name = "AnswerUpdated", abi = "AnswerUpdated(int256,uint256,uint256)")]
    pub struct AnswerUpdatedFilter {
        #[ethevent(indexed)]
        pub current: I256,
        #[ethevent(indexed)]
        pub round_id: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
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
    #[ethevent(name = "NewRound", abi = "NewRound(uint256,address,uint256)")]
    pub struct NewRoundFilter {
        #[ethevent(indexed)]
        pub round_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub started_by: ethers::core::types::Address,
        pub started_at: ethers::core::types::U256,
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
        name = "OwnershipTransferRequested",
        abi = "OwnershipTransferRequested(address,address)"
    )]
    pub struct OwnershipTransferRequestedFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
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
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockAggregatorProxyEvents {
        AnswerUpdatedFilter(AnswerUpdatedFilter),
        NewRoundFilter(NewRoundFilter),
        OwnershipTransferRequestedFilter(OwnershipTransferRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for MockAggregatorProxyEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AnswerUpdatedFilter::decode_log(log) {
                return Ok(MockAggregatorProxyEvents::AnswerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = NewRoundFilter::decode_log(log) {
                return Ok(MockAggregatorProxyEvents::NewRoundFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferRequestedFilter::decode_log(log) {
                return Ok(MockAggregatorProxyEvents::OwnershipTransferRequestedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MockAggregatorProxyEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MockAggregatorProxyEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockAggregatorProxyEvents::AnswerUpdatedFilter(element) => element.fmt(f),
                MockAggregatorProxyEvents::NewRoundFilter(element) => element.fmt(f),
                MockAggregatorProxyEvents::OwnershipTransferRequestedFilter(element) => {
                    element.fmt(f)
                }
                MockAggregatorProxyEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `accessController` function with signature `accessController()` and selector `[188, 67, 203, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "accessController", abi = "accessController()")]
    pub struct AccessControllerCall;
    #[doc = "Container type for all input parameters for the `aggregator` function with signature `aggregator()` and selector `[36, 90, 123, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "aggregator", abi = "aggregator()")]
    pub struct AggregatorCall;
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
    #[doc = "Container type for all input parameters for the `description` function with signature `description()` and selector `[114, 132, 228, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    #[doc = "Container type for all input parameters for the `getAnswer` function with signature `getAnswer(uint256)` and selector `[181, 171, 88, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAnswer", abi = "getAnswer(uint256)")]
    pub struct GetAnswerCall {
        pub round_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRoundData` function with signature `getRoundData(uint80)` and selector `[154, 111, 200, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall {
        pub round_id: u128,
    }
    #[doc = "Container type for all input parameters for the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `[182, 51, 98, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(uint256)")]
    pub struct GetTimestampCall {
        pub round_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `latestAnswer` function with signature `latestAnswer()` and selector `[80, 210, 91, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestAnswer", abi = "latestAnswer()")]
    pub struct LatestAnswerCall;
    #[doc = "Container type for all input parameters for the `latestAnswerValue` function with signature `latestAnswerValue()` and selector `[228, 77, 132, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestAnswerValue", abi = "latestAnswerValue()")]
    pub struct LatestAnswerValueCall;
    #[doc = "Container type for all input parameters for the `latestRound` function with signature `latestRound()` and selector `[102, 138, 15, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestRound", abi = "latestRound()")]
    pub struct LatestRoundCall;
    #[doc = "Container type for all input parameters for the `latestRoundData` function with signature `latestRoundData()` and selector `[254, 175, 150, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    #[doc = "Container type for all input parameters for the `latestRoundDataValue` function with signature `latestRoundDataValue()` and selector `[234, 0, 150, 153]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestRoundDataValue", abi = "latestRoundDataValue()")]
    pub struct LatestRoundDataValueCall;
    #[doc = "Container type for all input parameters for the `latestRoundValue` function with signature `latestRoundValue()` and selector `[2, 18, 220, 174]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestRoundValue", abi = "latestRoundValue()")]
    pub struct LatestRoundValueCall;
    #[doc = "Container type for all input parameters for the `latestTimestamp` function with signature `latestTimestamp()` and selector `[130, 5, 191, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestTimestamp", abi = "latestTimestamp()")]
    pub struct LatestTimestampCall;
    #[doc = "Container type for all input parameters for the `latestTimestampValue` function with signature `latestTimestampValue()` and selector `[126, 238, 29, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestTimestampValue", abi = "latestTimestampValue()")]
    pub struct LatestTimestampValueCall;
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
    #[doc = "Container type for all input parameters for the `phaseAggregators` function with signature `phaseAggregators(uint16)` and selector `[193, 89, 115, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "phaseAggregators", abi = "phaseAggregators(uint16)")]
    pub struct PhaseAggregatorsCall(pub u16);
    #[doc = "Container type for all input parameters for the `phaseId` function with signature `phaseId()` and selector `[88, 48, 59, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "phaseId", abi = "phaseId()")]
    pub struct PhaseIdCall;
    #[doc = "Container type for all input parameters for the `proposedAggregator` function with signature `proposedAggregator()` and selector `[232, 196, 190, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "proposedAggregator", abi = "proposedAggregator()")]
    pub struct ProposedAggregatorCall;
    #[doc = "Container type for all input parameters for the `proposedGetRoundData` function with signature `proposedGetRoundData(uint80)` and selector `[96, 1, 172, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "proposedGetRoundData", abi = "proposedGetRoundData(uint80)")]
    pub struct ProposedGetRoundDataCall(pub u128);
    #[doc = "Container type for all input parameters for the `proposedLatestRoundData` function with signature `proposedLatestRoundData()` and selector `[143, 107, 77, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "proposedLatestRoundData", abi = "proposedLatestRoundData()")]
    pub struct ProposedLatestRoundDataCall;
    #[doc = "Container type for all input parameters for the `roundData` function with signature `roundData(uint80)` and selector `[188, 129, 186, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "roundData", abi = "roundData(uint80)")]
    pub struct RoundDataCall(pub u128);
    #[doc = "Container type for all input parameters for the `roundIdAnswers` function with signature `roundIdAnswers(uint256)` and selector `[11, 143, 30, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "roundIdAnswers", abi = "roundIdAnswers(uint256)")]
    pub struct RoundIdAnswersCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `roundTimestamps` function with signature `roundTimestamps(uint256)` and selector `[187, 148, 103, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "roundTimestamps", abi = "roundTimestamps(uint256)")]
    pub struct RoundTimestampsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `setLatestAnswer` function with signature `setLatestAnswer(int256)` and selector `[4, 234, 151, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLatestAnswer", abi = "setLatestAnswer(int256)")]
    pub struct SetLatestAnswerCall {
        pub latest_answer: I256,
    }
    #[doc = "Container type for all input parameters for the `setLatestRound` function with signature `setLatestRound(uint256)` and selector `[223, 97, 195, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLatestRound", abi = "setLatestRound(uint256)")]
    pub struct SetLatestRoundCall {
        pub latest_round: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLatestRoundData` function with signature `setLatestRoundData((uint80,int256,uint256,uint256,uint80))` and selector `[223, 74, 0, 235]`"]
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
        name = "setLatestRoundData",
        abi = "setLatestRoundData((uint80,int256,uint256,uint256,uint80))"
    )]
    pub struct SetLatestRoundDataCall {
        pub latest_round_data: RoundData,
    }
    #[doc = "Container type for all input parameters for the `setLatestTimestamp` function with signature `setLatestTimestamp(uint256)` and selector `[159, 81, 197, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLatestTimestamp", abi = "setLatestTimestamp(uint256)")]
    pub struct SetLatestTimestampCall {
        pub latest_timestamp: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setRoundData` function with signature `setRoundData((uint80,int256,uint256,uint256,uint80))` and selector `[155, 68, 6, 96]`"]
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
        name = "setRoundData",
        abi = "setRoundData((uint80,int256,uint256,uint256,uint80))"
    )]
    pub struct SetRoundDataCall {
        pub round_data: RoundData,
    }
    #[doc = "Container type for all input parameters for the `setRoundIdAnswer` function with signature `setRoundIdAnswer(uint256,int256)` and selector `[54, 155, 225, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setRoundIdAnswer", abi = "setRoundIdAnswer(uint256,int256)")]
    pub struct SetRoundIdAnswerCall {
        pub round_id: ethers::core::types::U256,
        pub answer: I256,
    }
    #[doc = "Container type for all input parameters for the `setTimestamp` function with signature `setTimestamp(uint256,uint256)` and selector `[209, 96, 199, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setTimestamp", abi = "setTimestamp(uint256,uint256)")]
    pub struct SetTimestampCall {
        pub round: ethers::core::types::U256,
        pub timestamp: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockAggregatorProxyCalls {
        AccessController(AccessControllerCall),
        Aggregator(AggregatorCall),
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetAnswer(GetAnswerCall),
        GetRoundData(GetRoundDataCall),
        GetTimestamp(GetTimestampCall),
        LatestAnswer(LatestAnswerCall),
        LatestAnswerValue(LatestAnswerValueCall),
        LatestRound(LatestRoundCall),
        LatestRoundData(LatestRoundDataCall),
        LatestRoundDataValue(LatestRoundDataValueCall),
        LatestRoundValue(LatestRoundValueCall),
        LatestTimestamp(LatestTimestampCall),
        LatestTimestampValue(LatestTimestampValueCall),
        Owner(OwnerCall),
        PhaseAggregators(PhaseAggregatorsCall),
        PhaseId(PhaseIdCall),
        ProposedAggregator(ProposedAggregatorCall),
        ProposedGetRoundData(ProposedGetRoundDataCall),
        ProposedLatestRoundData(ProposedLatestRoundDataCall),
        RoundData(RoundDataCall),
        RoundIdAnswers(RoundIdAnswersCall),
        RoundTimestamps(RoundTimestampsCall),
        SetLatestAnswer(SetLatestAnswerCall),
        SetLatestRound(SetLatestRoundCall),
        SetLatestRoundData(SetLatestRoundDataCall),
        SetLatestTimestamp(SetLatestTimestampCall),
        SetRoundData(SetRoundDataCall),
        SetRoundIdAnswer(SetRoundIdAnswerCall),
        SetTimestamp(SetTimestampCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for MockAggregatorProxyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AccessControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::AccessController(decoded));
            }
            if let Ok(decoded) =
                <AggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::Aggregator(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DescriptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::Description(decoded));
            }
            if let Ok(decoded) =
                <GetAnswerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::GetAnswer(decoded));
            }
            if let Ok(decoded) =
                <GetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::GetRoundData(decoded));
            }
            if let Ok(decoded) =
                <GetTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::GetTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LatestAnswerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::LatestAnswer(decoded));
            }
            if let Ok(decoded) =
                <LatestAnswerValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::LatestAnswerValue(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::LatestRound(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::LatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::LatestRoundDataValue(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::LatestRoundValue(decoded));
            }
            if let Ok(decoded) =
                <LatestTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::LatestTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LatestTimestampValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::LatestTimestampValue(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PhaseAggregatorsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::PhaseAggregators(decoded));
            }
            if let Ok(decoded) =
                <PhaseIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::PhaseId(decoded));
            }
            if let Ok(decoded) =
                <ProposedAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::ProposedAggregator(decoded));
            }
            if let Ok(decoded) =
                <ProposedGetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::ProposedGetRoundData(decoded));
            }
            if let Ok(decoded) =
                <ProposedLatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::ProposedLatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <RoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::RoundData(decoded));
            }
            if let Ok(decoded) =
                <RoundIdAnswersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::RoundIdAnswers(decoded));
            }
            if let Ok(decoded) =
                <RoundTimestampsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::RoundTimestamps(decoded));
            }
            if let Ok(decoded) =
                <SetLatestAnswerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::SetLatestAnswer(decoded));
            }
            if let Ok(decoded) =
                <SetLatestRoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::SetLatestRound(decoded));
            }
            if let Ok(decoded) =
                <SetLatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::SetLatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <SetLatestTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::SetLatestTimestamp(decoded));
            }
            if let Ok(decoded) =
                <SetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::SetRoundData(decoded));
            }
            if let Ok(decoded) =
                <SetRoundIdAnswerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::SetRoundIdAnswer(decoded));
            }
            if let Ok(decoded) =
                <SetTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::SetTimestamp(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockAggregatorProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockAggregatorProxyCalls::AccessController(element) => element.encode(),
                MockAggregatorProxyCalls::Aggregator(element) => element.encode(),
                MockAggregatorProxyCalls::Decimals(element) => element.encode(),
                MockAggregatorProxyCalls::Description(element) => element.encode(),
                MockAggregatorProxyCalls::GetAnswer(element) => element.encode(),
                MockAggregatorProxyCalls::GetRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::GetTimestamp(element) => element.encode(),
                MockAggregatorProxyCalls::LatestAnswer(element) => element.encode(),
                MockAggregatorProxyCalls::LatestAnswerValue(element) => element.encode(),
                MockAggregatorProxyCalls::LatestRound(element) => element.encode(),
                MockAggregatorProxyCalls::LatestRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::LatestRoundDataValue(element) => element.encode(),
                MockAggregatorProxyCalls::LatestRoundValue(element) => element.encode(),
                MockAggregatorProxyCalls::LatestTimestamp(element) => element.encode(),
                MockAggregatorProxyCalls::LatestTimestampValue(element) => element.encode(),
                MockAggregatorProxyCalls::Owner(element) => element.encode(),
                MockAggregatorProxyCalls::PhaseAggregators(element) => element.encode(),
                MockAggregatorProxyCalls::PhaseId(element) => element.encode(),
                MockAggregatorProxyCalls::ProposedAggregator(element) => element.encode(),
                MockAggregatorProxyCalls::ProposedGetRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::ProposedLatestRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::RoundData(element) => element.encode(),
                MockAggregatorProxyCalls::RoundIdAnswers(element) => element.encode(),
                MockAggregatorProxyCalls::RoundTimestamps(element) => element.encode(),
                MockAggregatorProxyCalls::SetLatestAnswer(element) => element.encode(),
                MockAggregatorProxyCalls::SetLatestRound(element) => element.encode(),
                MockAggregatorProxyCalls::SetLatestRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::SetLatestTimestamp(element) => element.encode(),
                MockAggregatorProxyCalls::SetRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::SetRoundIdAnswer(element) => element.encode(),
                MockAggregatorProxyCalls::SetTimestamp(element) => element.encode(),
                MockAggregatorProxyCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockAggregatorProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockAggregatorProxyCalls::AccessController(element) => element.fmt(f),
                MockAggregatorProxyCalls::Aggregator(element) => element.fmt(f),
                MockAggregatorProxyCalls::Decimals(element) => element.fmt(f),
                MockAggregatorProxyCalls::Description(element) => element.fmt(f),
                MockAggregatorProxyCalls::GetAnswer(element) => element.fmt(f),
                MockAggregatorProxyCalls::GetRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::GetTimestamp(element) => element.fmt(f),
                MockAggregatorProxyCalls::LatestAnswer(element) => element.fmt(f),
                MockAggregatorProxyCalls::LatestAnswerValue(element) => element.fmt(f),
                MockAggregatorProxyCalls::LatestRound(element) => element.fmt(f),
                MockAggregatorProxyCalls::LatestRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::LatestRoundDataValue(element) => element.fmt(f),
                MockAggregatorProxyCalls::LatestRoundValue(element) => element.fmt(f),
                MockAggregatorProxyCalls::LatestTimestamp(element) => element.fmt(f),
                MockAggregatorProxyCalls::LatestTimestampValue(element) => element.fmt(f),
                MockAggregatorProxyCalls::Owner(element) => element.fmt(f),
                MockAggregatorProxyCalls::PhaseAggregators(element) => element.fmt(f),
                MockAggregatorProxyCalls::PhaseId(element) => element.fmt(f),
                MockAggregatorProxyCalls::ProposedAggregator(element) => element.fmt(f),
                MockAggregatorProxyCalls::ProposedGetRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::ProposedLatestRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::RoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::RoundIdAnswers(element) => element.fmt(f),
                MockAggregatorProxyCalls::RoundTimestamps(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetLatestAnswer(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetLatestRound(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetLatestRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetLatestTimestamp(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetRoundIdAnswer(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetTimestamp(element) => element.fmt(f),
                MockAggregatorProxyCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AccessControllerCall> for MockAggregatorProxyCalls {
        fn from(var: AccessControllerCall) -> Self {
            MockAggregatorProxyCalls::AccessController(var)
        }
    }
    impl ::std::convert::From<AggregatorCall> for MockAggregatorProxyCalls {
        fn from(var: AggregatorCall) -> Self {
            MockAggregatorProxyCalls::Aggregator(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for MockAggregatorProxyCalls {
        fn from(var: DecimalsCall) -> Self {
            MockAggregatorProxyCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DescriptionCall> for MockAggregatorProxyCalls {
        fn from(var: DescriptionCall) -> Self {
            MockAggregatorProxyCalls::Description(var)
        }
    }
    impl ::std::convert::From<GetAnswerCall> for MockAggregatorProxyCalls {
        fn from(var: GetAnswerCall) -> Self {
            MockAggregatorProxyCalls::GetAnswer(var)
        }
    }
    impl ::std::convert::From<GetRoundDataCall> for MockAggregatorProxyCalls {
        fn from(var: GetRoundDataCall) -> Self {
            MockAggregatorProxyCalls::GetRoundData(var)
        }
    }
    impl ::std::convert::From<GetTimestampCall> for MockAggregatorProxyCalls {
        fn from(var: GetTimestampCall) -> Self {
            MockAggregatorProxyCalls::GetTimestamp(var)
        }
    }
    impl ::std::convert::From<LatestAnswerCall> for MockAggregatorProxyCalls {
        fn from(var: LatestAnswerCall) -> Self {
            MockAggregatorProxyCalls::LatestAnswer(var)
        }
    }
    impl ::std::convert::From<LatestAnswerValueCall> for MockAggregatorProxyCalls {
        fn from(var: LatestAnswerValueCall) -> Self {
            MockAggregatorProxyCalls::LatestAnswerValue(var)
        }
    }
    impl ::std::convert::From<LatestRoundCall> for MockAggregatorProxyCalls {
        fn from(var: LatestRoundCall) -> Self {
            MockAggregatorProxyCalls::LatestRound(var)
        }
    }
    impl ::std::convert::From<LatestRoundDataCall> for MockAggregatorProxyCalls {
        fn from(var: LatestRoundDataCall) -> Self {
            MockAggregatorProxyCalls::LatestRoundData(var)
        }
    }
    impl ::std::convert::From<LatestRoundDataValueCall> for MockAggregatorProxyCalls {
        fn from(var: LatestRoundDataValueCall) -> Self {
            MockAggregatorProxyCalls::LatestRoundDataValue(var)
        }
    }
    impl ::std::convert::From<LatestRoundValueCall> for MockAggregatorProxyCalls {
        fn from(var: LatestRoundValueCall) -> Self {
            MockAggregatorProxyCalls::LatestRoundValue(var)
        }
    }
    impl ::std::convert::From<LatestTimestampCall> for MockAggregatorProxyCalls {
        fn from(var: LatestTimestampCall) -> Self {
            MockAggregatorProxyCalls::LatestTimestamp(var)
        }
    }
    impl ::std::convert::From<LatestTimestampValueCall> for MockAggregatorProxyCalls {
        fn from(var: LatestTimestampValueCall) -> Self {
            MockAggregatorProxyCalls::LatestTimestampValue(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for MockAggregatorProxyCalls {
        fn from(var: OwnerCall) -> Self {
            MockAggregatorProxyCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PhaseAggregatorsCall> for MockAggregatorProxyCalls {
        fn from(var: PhaseAggregatorsCall) -> Self {
            MockAggregatorProxyCalls::PhaseAggregators(var)
        }
    }
    impl ::std::convert::From<PhaseIdCall> for MockAggregatorProxyCalls {
        fn from(var: PhaseIdCall) -> Self {
            MockAggregatorProxyCalls::PhaseId(var)
        }
    }
    impl ::std::convert::From<ProposedAggregatorCall> for MockAggregatorProxyCalls {
        fn from(var: ProposedAggregatorCall) -> Self {
            MockAggregatorProxyCalls::ProposedAggregator(var)
        }
    }
    impl ::std::convert::From<ProposedGetRoundDataCall> for MockAggregatorProxyCalls {
        fn from(var: ProposedGetRoundDataCall) -> Self {
            MockAggregatorProxyCalls::ProposedGetRoundData(var)
        }
    }
    impl ::std::convert::From<ProposedLatestRoundDataCall> for MockAggregatorProxyCalls {
        fn from(var: ProposedLatestRoundDataCall) -> Self {
            MockAggregatorProxyCalls::ProposedLatestRoundData(var)
        }
    }
    impl ::std::convert::From<RoundDataCall> for MockAggregatorProxyCalls {
        fn from(var: RoundDataCall) -> Self {
            MockAggregatorProxyCalls::RoundData(var)
        }
    }
    impl ::std::convert::From<RoundIdAnswersCall> for MockAggregatorProxyCalls {
        fn from(var: RoundIdAnswersCall) -> Self {
            MockAggregatorProxyCalls::RoundIdAnswers(var)
        }
    }
    impl ::std::convert::From<RoundTimestampsCall> for MockAggregatorProxyCalls {
        fn from(var: RoundTimestampsCall) -> Self {
            MockAggregatorProxyCalls::RoundTimestamps(var)
        }
    }
    impl ::std::convert::From<SetLatestAnswerCall> for MockAggregatorProxyCalls {
        fn from(var: SetLatestAnswerCall) -> Self {
            MockAggregatorProxyCalls::SetLatestAnswer(var)
        }
    }
    impl ::std::convert::From<SetLatestRoundCall> for MockAggregatorProxyCalls {
        fn from(var: SetLatestRoundCall) -> Self {
            MockAggregatorProxyCalls::SetLatestRound(var)
        }
    }
    impl ::std::convert::From<SetLatestRoundDataCall> for MockAggregatorProxyCalls {
        fn from(var: SetLatestRoundDataCall) -> Self {
            MockAggregatorProxyCalls::SetLatestRoundData(var)
        }
    }
    impl ::std::convert::From<SetLatestTimestampCall> for MockAggregatorProxyCalls {
        fn from(var: SetLatestTimestampCall) -> Self {
            MockAggregatorProxyCalls::SetLatestTimestamp(var)
        }
    }
    impl ::std::convert::From<SetRoundDataCall> for MockAggregatorProxyCalls {
        fn from(var: SetRoundDataCall) -> Self {
            MockAggregatorProxyCalls::SetRoundData(var)
        }
    }
    impl ::std::convert::From<SetRoundIdAnswerCall> for MockAggregatorProxyCalls {
        fn from(var: SetRoundIdAnswerCall) -> Self {
            MockAggregatorProxyCalls::SetRoundIdAnswer(var)
        }
    }
    impl ::std::convert::From<SetTimestampCall> for MockAggregatorProxyCalls {
        fn from(var: SetTimestampCall) -> Self {
            MockAggregatorProxyCalls::SetTimestamp(var)
        }
    }
    impl ::std::convert::From<VersionCall> for MockAggregatorProxyCalls {
        fn from(var: VersionCall) -> Self {
            MockAggregatorProxyCalls::Version(var)
        }
    }
    #[doc = "Container type for all return fields from the `accessController` function with signature `accessController()` and selector `[188, 67, 203, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AccessControllerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `aggregator` function with signature `aggregator()` and selector `[36, 90, 123, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AggregatorReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `description` function with signature `description()` and selector `[114, 132, 228, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DescriptionReturn(pub String);
    #[doc = "Container type for all return fields from the `getAnswer` function with signature `getAnswer(uint256)` and selector `[181, 171, 88, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAnswerReturn(pub I256);
    #[doc = "Container type for all return fields from the `getRoundData` function with signature `getRoundData(uint80)` and selector `[154, 111, 200, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetRoundDataReturn {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    #[doc = "Container type for all return fields from the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `[182, 51, 98, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetTimestampReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `latestAnswer` function with signature `latestAnswer()` and selector `[80, 210, 91, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestAnswerReturn(pub I256);
    #[doc = "Container type for all return fields from the `latestAnswerValue` function with signature `latestAnswerValue()` and selector `[228, 77, 132, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestAnswerValueReturn(pub I256);
    #[doc = "Container type for all return fields from the `latestRound` function with signature `latestRound()` and selector `[102, 138, 15, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestRoundReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `latestRoundData` function with signature `latestRoundData()` and selector `[254, 175, 150, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestRoundDataReturn {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    #[doc = "Container type for all return fields from the `latestRoundDataValue` function with signature `latestRoundDataValue()` and selector `[234, 0, 150, 153]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestRoundDataValueReturn {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    #[doc = "Container type for all return fields from the `latestRoundValue` function with signature `latestRoundValue()` and selector `[2, 18, 220, 174]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestRoundValueReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `latestTimestamp` function with signature `latestTimestamp()` and selector `[130, 5, 191, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestTimestampReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `latestTimestampValue` function with signature `latestTimestampValue()` and selector `[126, 238, 29, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestTimestampValueReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `phaseAggregators` function with signature `phaseAggregators(uint16)` and selector `[193, 89, 115, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PhaseAggregatorsReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `phaseId` function with signature `phaseId()` and selector `[88, 48, 59, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PhaseIdReturn(pub u16);
    #[doc = "Container type for all return fields from the `proposedAggregator` function with signature `proposedAggregator()` and selector `[232, 196, 190, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ProposedAggregatorReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `proposedGetRoundData` function with signature `proposedGetRoundData(uint80)` and selector `[96, 1, 172, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ProposedGetRoundDataReturn {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    #[doc = "Container type for all return fields from the `proposedLatestRoundData` function with signature `proposedLatestRoundData()` and selector `[143, 107, 77, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ProposedLatestRoundDataReturn {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    #[doc = "Container type for all return fields from the `roundData` function with signature `roundData(uint80)` and selector `[188, 129, 186, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RoundDataReturn {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    #[doc = "Container type for all return fields from the `roundIdAnswers` function with signature `roundIdAnswers(uint256)` and selector `[11, 143, 30, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RoundIdAnswersReturn(pub I256);
    #[doc = "Container type for all return fields from the `roundTimestamps` function with signature `roundTimestamps(uint256)` and selector `[187, 148, 103, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RoundTimestampsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VersionReturn(pub ethers::core::types::U256);
    #[doc = "`RoundData(uint80,int256,uint256,uint256,uint80)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RoundData {
        pub round_id: u128,
        pub answer: I256,
        pub started_at: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
        pub answered_in_round: u128,
    }
}
