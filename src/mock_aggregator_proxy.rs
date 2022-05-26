pub use mockaggregatorproxy_mod::*;
#[allow(clippy::too_many_arguments)]
mod mockaggregatorproxy_mod {
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
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"current\",\"type\":\"int256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AnswerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"startedBy\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRound\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"accessController\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"aggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_aggregator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"confirmAggregator\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestAnswerValue\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRound\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundDataValue\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundValue\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestTimestampValue\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"phaseAggregators\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"phaseId\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_aggregator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposeAggregator\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"proposedAggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"proposedGetRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"proposedLatestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roundIdAnswers\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roundTimestamps\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_accessController\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setController\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_latestAnswer\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLatestAnswer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_latestRound\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLatestRound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct MockAggregatorProxy.RoundData\",\"name\":\"_latestRoundData\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLatestRoundData\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_latestTimestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLatestTimestamp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct MockAggregatorProxy.RoundData\",\"name\":\"_roundData\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRoundData\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_answer\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRoundIdAnswer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_round\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_timestamp\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTimestamp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKAGGREGATORPROXY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001657610c42908161001c8239f35b600080fdfe60806040526004361015610013575b600080fd5b60003560e01c80630212dcae1461025e57806304ea97b0146102825780630b8f1e5514610216578063245a7bfc146101ce578063313ce56714610279578063369be1621461027057806350d25bcd146101d757806354fd4d50146101ce57806358303b10146101ce5780636001ac5314610267578063668a0f021461025e5780637284e4161461025557806379ba50971461024c5780637eee1d95146102435780638205bf6a146102435780638da5cb5b146101ce5780638f6b4d911461023a57806392eefe9b146101bc5780639a6fc8f5146102315780639b440660146102285780639f51c5021461021f578063a928c096146101bc578063b5ab58dc14610216578063b633620c1461020d578063bb9467291461020d578063bc43cbaf146101ce578063bc81bae814610204578063c1597304146101fb578063d160c7a9146101f2578063df4a00eb146101e9578063df61c3e2146101e0578063e44d846d146101d7578063e8c4be30146101ce578063ea009699146101c5578063f2fde38b146101bc578063f8a2abd3146101bc5763feaf968c146101b457600080fd5b61000e610b1b565b5061000e61066e565b5061000e610b1b565b5061000e61034b565b5061000e61040c565b5061000e610ae3565b5061000e610a59565b5061000e610a0e565b5061000e6109c6565b5061000e61092a565b5061000e6108df565b5061000e610300565b5061000e6108a7565b5061000e6107c4565b5061000e6106bf565b5061000e610613565b5061000e6105d6565b5061000e6105a4565b5061000e6104c7565b5061000e61028b565b5061000e61045d565b5061000e6103c1565b5061000e610386565b5061000e6102c8565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020600a54604051908152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57600435600855005b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043560005260016020526020604060002054604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602060405160008152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602060405160088152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043560005260016020526024356040600020556000604051f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020600854604051908152f35b69ffffffffffffffffffff81160361000e57565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610498600435610449565b604051806104c381906000608060a08401938281528260208201528260408201528260608201520152565b0390f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576104ff610bbf565b6003815260207f2e2e2e000000000000000000000000000000000000000000000000000000000081830152604090815192818492835281519182828501526000915b83831061058c575050817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe092601f921161057f575b01168101030190f35b6000858286010152610576565b81830181015187840187015286945091820191610541565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57005b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020600954604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57604051806104c381906000608060a08401938281528260208201528260408201528260608201520152565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043573ffffffffffffffffffffffffffffffffffffffff81160361000e57005b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576004356106fb81610449565b69ffffffffffffffffffff80911660005260026020526040600020906104c36080610724610bec565b938381541693848652600182015495866020820152600283015491826040830152600460038501549485606085015201541693849101526040519586958693608093969591929660a086019769ffffffffffffffffffff809516875260208701526040860152606085015216910152565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc60a091011261000e57600490565b503461000e576108a56107d636610795565b600460808235926107e684610449565b69ffffffffffffffffffff8416600052600260205261084460406000209461080d81610449565b859069ffffffffffffffffffff167fffffffffffffffffffffffffffffffffffffffffffff00000000000000000000825416179055565b60208101356001850155604081013560028501556060810135600385015501359161086e83610449565b019069ffffffffffffffffffff167fffffffffffffffffffffffffffffffffffffffffffff00000000000000000000825416179055565b005b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57600435600955005b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043560005260006020526020604060002054604051908152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043561096681610449565b69ffffffffffffffffffff908116600090815260026020818152604092839020805460018201549382015460038301546004909301548651928816835293820194909452938401929092526060830191909152909116608082015260a090f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043561ffff81160361000e57602060405160008152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043560005260006020526024356040600020556000604051f35b503461000e57610a6836610795565b803590610a7482610449565b608069ffffffffffffffffffff91827fffffffffffffffffffffffffffffffffffffffffffff0000000000000000000094168460035416176003556020810135600455604081013560055560608101356006550135610ad281610449565b169060075416176007556000604051f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57600435600a55005b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576003546004546005546006546007546040805169ffffffffffffffffffff96871681526020810195909552840192909252606083015291909116608082015260a090f35b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b604051906040820182811067ffffffffffffffff821117610bdf57604052565b610be7610b8f565b604052565b6040519060a0820182811067ffffffffffffffff821117610bdf5760405256fea26469706673582212208b85dbbb92f0fe2d20889add1ccf9d49ba75171bc4a9028be2bddd5e255a4af164736f6c634300080e0033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `acceptOwnership` (0x79ba5097) function"]
        pub fn accept_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `confirmAggregator` (0xa928c096) function"]
        pub fn confirm_aggregator(
            &self,
            aggregator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 40, 192, 150], aggregator)
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
        #[doc = "Calls the contract's `proposeAggregator` (0xf8a2abd3) function"]
        pub fn propose_aggregator(
            &self,
            aggregator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 162, 171, 211], aggregator)
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
        #[doc = "Calls the contract's `setController` (0x92eefe9b) function"]
        pub fn set_controller(
            &self,
            access_controller: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 238, 254, 155], access_controller)
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
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], to)
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
    #[doc = "Container type for all input parameters for the `acceptOwnership`function with signature `acceptOwnership()` and selector `[121, 186, 80, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    #[doc = "Container type for all input parameters for the `accessController`function with signature `accessController()` and selector `[188, 67, 203, 175]`"]
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
    #[doc = "Container type for all input parameters for the `aggregator`function with signature `aggregator()` and selector `[36, 90, 123, 252]`"]
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
    #[doc = "Container type for all input parameters for the `confirmAggregator`function with signature `confirmAggregator(address)` and selector `[169, 40, 192, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "confirmAggregator", abi = "confirmAggregator(address)")]
    pub struct ConfirmAggregatorCall {
        pub aggregator: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `description`function with signature `description()` and selector `[114, 132, 228, 22]`"]
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
    #[doc = "Container type for all input parameters for the `getAnswer`function with signature `getAnswer(uint256)` and selector `[181, 171, 88, 220]`"]
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
    #[doc = "Container type for all input parameters for the `getRoundData`function with signature `getRoundData(uint80)` and selector `[154, 111, 200, 245]`"]
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
    #[doc = "Container type for all input parameters for the `getTimestamp`function with signature `getTimestamp(uint256)` and selector `[182, 51, 98, 12]`"]
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
    #[doc = "Container type for all input parameters for the `latestAnswer`function with signature `latestAnswer()` and selector `[80, 210, 91, 205]`"]
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
    #[doc = "Container type for all input parameters for the `latestAnswerValue`function with signature `latestAnswerValue()` and selector `[228, 77, 132, 109]`"]
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
    #[doc = "Container type for all input parameters for the `latestRound`function with signature `latestRound()` and selector `[102, 138, 15, 2]`"]
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
    #[doc = "Container type for all input parameters for the `latestRoundData`function with signature `latestRoundData()` and selector `[254, 175, 150, 140]`"]
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
    #[doc = "Container type for all input parameters for the `latestRoundDataValue`function with signature `latestRoundDataValue()` and selector `[234, 0, 150, 153]`"]
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
    #[doc = "Container type for all input parameters for the `latestRoundValue`function with signature `latestRoundValue()` and selector `[2, 18, 220, 174]`"]
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
    #[doc = "Container type for all input parameters for the `latestTimestamp`function with signature `latestTimestamp()` and selector `[130, 5, 191, 106]`"]
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
    #[doc = "Container type for all input parameters for the `latestTimestampValue`function with signature `latestTimestampValue()` and selector `[126, 238, 29, 149]`"]
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
    #[doc = "Container type for all input parameters for the `phaseAggregators`function with signature `phaseAggregators(uint16)` and selector `[193, 89, 115, 4]`"]
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
    #[doc = "Container type for all input parameters for the `phaseId`function with signature `phaseId()` and selector `[88, 48, 59, 16]`"]
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
    #[doc = "Container type for all input parameters for the `proposeAggregator`function with signature `proposeAggregator(address)` and selector `[248, 162, 171, 211]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "proposeAggregator", abi = "proposeAggregator(address)")]
    pub struct ProposeAggregatorCall {
        pub aggregator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `proposedAggregator`function with signature `proposedAggregator()` and selector `[232, 196, 190, 48]`"]
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
    #[doc = "Container type for all input parameters for the `proposedGetRoundData`function with signature `proposedGetRoundData(uint80)` and selector `[96, 1, 172, 83]`"]
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
    #[doc = "Container type for all input parameters for the `proposedLatestRoundData`function with signature `proposedLatestRoundData()` and selector `[143, 107, 77, 145]`"]
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
    #[doc = "Container type for all input parameters for the `roundData`function with signature `roundData(uint80)` and selector `[188, 129, 186, 232]`"]
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
    #[doc = "Container type for all input parameters for the `roundIdAnswers`function with signature `roundIdAnswers(uint256)` and selector `[11, 143, 30, 85]`"]
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
    #[doc = "Container type for all input parameters for the `roundTimestamps`function with signature `roundTimestamps(uint256)` and selector `[187, 148, 103, 41]`"]
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
    #[doc = "Container type for all input parameters for the `setController`function with signature `setController(address)` and selector `[146, 238, 254, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setController", abi = "setController(address)")]
    pub struct SetControllerCall {
        pub access_controller: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setLatestAnswer`function with signature `setLatestAnswer(int256)` and selector `[4, 234, 151, 176]`"]
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
    #[doc = "Container type for all input parameters for the `setLatestRound`function with signature `setLatestRound(uint256)` and selector `[223, 97, 195, 226]`"]
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
    #[doc = "Container type for all input parameters for the `setLatestRoundData`function with signature `setLatestRoundData((uint80,int256,uint256,uint256,uint80))` and selector `[223, 74, 0, 235]`"]
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
    #[doc = "Container type for all input parameters for the `setLatestTimestamp`function with signature `setLatestTimestamp(uint256)` and selector `[159, 81, 197, 2]`"]
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
    #[doc = "Container type for all input parameters for the `setRoundData`function with signature `setRoundData((uint80,int256,uint256,uint256,uint80))` and selector `[155, 68, 6, 96]`"]
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
    #[doc = "Container type for all input parameters for the `setRoundIdAnswer`function with signature `setRoundIdAnswer(uint256,int256)` and selector `[54, 155, 225, 98]`"]
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
    #[doc = "Container type for all input parameters for the `setTimestamp`function with signature `setTimestamp(uint256,uint256)` and selector `[209, 96, 199, 169]`"]
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
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `version`function with signature `version()` and selector `[84, 253, 77, 80]`"]
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
        AcceptOwnership(AcceptOwnershipCall),
        AccessController(AccessControllerCall),
        Aggregator(AggregatorCall),
        ConfirmAggregator(ConfirmAggregatorCall),
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
        ProposeAggregator(ProposeAggregatorCall),
        ProposedAggregator(ProposedAggregatorCall),
        ProposedGetRoundData(ProposedGetRoundDataCall),
        ProposedLatestRoundData(ProposedLatestRoundDataCall),
        RoundData(RoundDataCall),
        RoundIdAnswers(RoundIdAnswersCall),
        RoundTimestamps(RoundTimestampsCall),
        SetController(SetControllerCall),
        SetLatestAnswer(SetLatestAnswerCall),
        SetLatestRound(SetLatestRoundCall),
        SetLatestRoundData(SetLatestRoundDataCall),
        SetLatestTimestamp(SetLatestTimestampCall),
        SetRoundData(SetRoundDataCall),
        SetRoundIdAnswer(SetRoundIdAnswerCall),
        SetTimestamp(SetTimestampCall),
        TransferOwnership(TransferOwnershipCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for MockAggregatorProxyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::AcceptOwnership(decoded));
            }
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
                <ConfirmAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::ConfirmAggregator(decoded));
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
                <ProposeAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::ProposeAggregator(decoded));
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
                <SetControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::SetController(decoded));
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
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockAggregatorProxyCalls::TransferOwnership(decoded));
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
                MockAggregatorProxyCalls::AcceptOwnership(element) => element.encode(),
                MockAggregatorProxyCalls::AccessController(element) => element.encode(),
                MockAggregatorProxyCalls::Aggregator(element) => element.encode(),
                MockAggregatorProxyCalls::ConfirmAggregator(element) => element.encode(),
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
                MockAggregatorProxyCalls::ProposeAggregator(element) => element.encode(),
                MockAggregatorProxyCalls::ProposedAggregator(element) => element.encode(),
                MockAggregatorProxyCalls::ProposedGetRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::ProposedLatestRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::RoundData(element) => element.encode(),
                MockAggregatorProxyCalls::RoundIdAnswers(element) => element.encode(),
                MockAggregatorProxyCalls::RoundTimestamps(element) => element.encode(),
                MockAggregatorProxyCalls::SetController(element) => element.encode(),
                MockAggregatorProxyCalls::SetLatestAnswer(element) => element.encode(),
                MockAggregatorProxyCalls::SetLatestRound(element) => element.encode(),
                MockAggregatorProxyCalls::SetLatestRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::SetLatestTimestamp(element) => element.encode(),
                MockAggregatorProxyCalls::SetRoundData(element) => element.encode(),
                MockAggregatorProxyCalls::SetRoundIdAnswer(element) => element.encode(),
                MockAggregatorProxyCalls::SetTimestamp(element) => element.encode(),
                MockAggregatorProxyCalls::TransferOwnership(element) => element.encode(),
                MockAggregatorProxyCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockAggregatorProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockAggregatorProxyCalls::AcceptOwnership(element) => element.fmt(f),
                MockAggregatorProxyCalls::AccessController(element) => element.fmt(f),
                MockAggregatorProxyCalls::Aggregator(element) => element.fmt(f),
                MockAggregatorProxyCalls::ConfirmAggregator(element) => element.fmt(f),
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
                MockAggregatorProxyCalls::ProposeAggregator(element) => element.fmt(f),
                MockAggregatorProxyCalls::ProposedAggregator(element) => element.fmt(f),
                MockAggregatorProxyCalls::ProposedGetRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::ProposedLatestRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::RoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::RoundIdAnswers(element) => element.fmt(f),
                MockAggregatorProxyCalls::RoundTimestamps(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetController(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetLatestAnswer(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetLatestRound(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetLatestRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetLatestTimestamp(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetRoundData(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetRoundIdAnswer(element) => element.fmt(f),
                MockAggregatorProxyCalls::SetTimestamp(element) => element.fmt(f),
                MockAggregatorProxyCalls::TransferOwnership(element) => element.fmt(f),
                MockAggregatorProxyCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptOwnershipCall> for MockAggregatorProxyCalls {
        fn from(var: AcceptOwnershipCall) -> Self {
            MockAggregatorProxyCalls::AcceptOwnership(var)
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
    impl ::std::convert::From<ConfirmAggregatorCall> for MockAggregatorProxyCalls {
        fn from(var: ConfirmAggregatorCall) -> Self {
            MockAggregatorProxyCalls::ConfirmAggregator(var)
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
    impl ::std::convert::From<ProposeAggregatorCall> for MockAggregatorProxyCalls {
        fn from(var: ProposeAggregatorCall) -> Self {
            MockAggregatorProxyCalls::ProposeAggregator(var)
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
    impl ::std::convert::From<SetControllerCall> for MockAggregatorProxyCalls {
        fn from(var: SetControllerCall) -> Self {
            MockAggregatorProxyCalls::SetController(var)
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
    impl ::std::convert::From<TransferOwnershipCall> for MockAggregatorProxyCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            MockAggregatorProxyCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<VersionCall> for MockAggregatorProxyCalls {
        fn from(var: VersionCall) -> Self {
            MockAggregatorProxyCalls::Version(var)
        }
    }
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
