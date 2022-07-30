pub use quant_math_tester::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod quant_math_tester {
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
    #[doc = "QuantMathTester was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static QUANTMATHTESTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"addTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"divTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"fromScaledUintTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"a\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"fromUnscaledIntTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isEqualTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isGreaterThanOrEqualTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isGreaterThanTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isLessThanOrEqualTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isLessThanTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"maxTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"minTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"mulTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"subTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"roundDown\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"toScaledUintTest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static QUANTMATHTESTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001657610e7a908161001c8239f35b600080fdfe6040608081526004908136101561001557600080fd5b600091823560e01c908163186c1a0614610a4557816323fe1cfb146109d65781632d5a6b3f146108c457816333293dc01461084d5781633366567c14610789578163465371971461061f57816366829be9146105ce57816377714493146104d0578163a07fdf1b1461047e578163bb27b976146102e7578163c334a50b14610296578163dc9ebe0b14610177578163e2052e0d14610111575063ffd908f2146100bd57600080fd5b3461010d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010d576020906100f736610ae5565b61010036610b24565b9051905113159051908152f35b5080fd5b9050346101735760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610173576101629083602094610152610a96565b5261015b610a96565b5235610b5e565b908161016c610a96565b5251908152f35b8280fd5b82843461029357817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610293576101b036610ae5565b6101b936610b24565b90826101c3610a96565b52826101cd610a96565b5251905191808312837f800000000000000000000000000000000000000000000000000000000000000001831281151661026757837f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0183131661023b57506020935003908161016c610a96565b806011867f4e487b71000000000000000000000000000000000000000000000000000000006024945252fd5b6024826011887f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b80fd5b50503461010d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010d576020906102d236610ae5565b6102db36610b24565b90519051149051908152f35b9050346101735760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101735761032136610ae5565b926024359060443580159081150361010d57601b8303610353575050505061034b60209251610e04565b905b51908152f35b601b83949693116000146103b45750601b831061023b575060209350906103a87fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe56103a16103ae9451610e04565b9201610cd4565b90610d11565b9061034d565b9280601b9392931061045257601b039082936103fd575b906103e26103dc6103e89351610e04565b91610cd4565b90610d42565b908219821161023b575060209350019061034d565b6104078151610e04565b61041083610cd4565b9081156104265706156103cb57600193506103cb565b60248560128a7f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b6024836011887f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b50503461010d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010d576020906104ba36610ae5565b6104c336610b24565b9051905112159051908152f35b82843461029357817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610293578235906024359080610510610a96565b528061051a610a96565b5280610524610a96565b52601b82036105505750506020925061053c90610d7b565b610544610a96565b908152905b5190518152f35b601b8211156105ae57601b821061023b57506020935061059d916103e27fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe56105989301610cd4565b610d7b565b6105a5610a96565b90815290610549565b81601b1061023b57506020935061059d916103a861059892601b03610cd4565b50503461010d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010d5760209061060a36610ae5565b61061336610b24565b90519051129051908152f35b83833461010d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010d5761065836610ae5565b9161066236610b24565b928161066c610a96565b5281610676610a96565b526106818151610b5e565b5083511561075d576106939051610b5e565b9251801561075d577f8000000000000000000000000000000000000000000000000000000000000000937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82148582141661045257059281841290847f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0383138215166104525784900382121661073157602083838161016c610a96565b806011857f4e487b71000000000000000000000000000000000000000000000000000000006024945252fd5b6024826012877f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b82843461029357817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610293576107c236610ae5565b6107cb36610b24565b90826107d5610a96565b52826107df610a96565b5251905191808212827f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03841381151661026757827f80000000000000000000000000000000000000000000000000000000000000000384121661023b57506020935001908161016c610a96565b50503461010d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010d5760209161088936610ae5565b9061089336610b24565b908061089d610a96565b52806108a7610a96565b52508151815112156108bc5750905190518152f35b905090610549565b83833461010d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010d576b033b2e3c9fd0803ce800000061094261090d36610ae5565b61091636610b24565b9085610920610a96565b528561092a610a96565b526109388151835190610bf0565b5051905190610bf0565b0591808312837f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0382138115166109aa57837f80000000000000000000000000000000000000000000000000000000000000000382121661073157602083838161016c610a96565b6024826011877f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b50503461010d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010d57602091610a1236610ae5565b90610a1c36610b24565b9080610a26610a96565b5280610a30610a96565b52508151815113156108bc5750905190518152f35b50503461010d57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010d57602090610a8136610ae5565b610a8a36610b24565b90519051139051908152f35b604051906020820182811067ffffffffffffffff821117610ab657604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc6020910112610b1f57610b17610a96565b906004358252565b600080fd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdc6020910112610b1f57610b56610a96565b906024358252565b6000600174279d346de4781f921dd7a89933d54d1f72927c701483118284131616610bc35760017fffffffffffffffffffffffd862cb921b87e06de2285766cc2ab2e08d6d838fec83128284121616610bc357506b033b2e3c9fd0803ce80000000290565b807f4e487b7100000000000000000000000000000000000000000000000000000000602492526011600452fd5b60008082138184137f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff828216868204861116610c7a57838612917f800000000000000000000000000000000000000000000000000000000000000093838786058912911616610ca757868587129405861290841616610c7a578590058412911616610bc357500290565b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b604d8111610ce257600a0a90565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04821181151516610ce2570290565b8115610d4c570490565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b7f8000000000000000000000000000000000000000000000000000000000000000811015610da65790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f5175616e744d6174683a206f7574206f6620696e742072616e676500000000006044820152fd5b60008112610e0f5790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5175616e744d6174683a206e6567617469766520696e740000000000000000006044820152fdfea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
        });
    pub struct QuantMathTester<M>(ethers::contract::Contract<M>);
    impl<M> Clone for QuantMathTester<M> {
        fn clone(&self) -> Self {
            QuantMathTester(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for QuantMathTester<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for QuantMathTester<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(QuantMathTester))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> QuantMathTester<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), QUANTMATHTESTER_ABI.clone(), client)
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
                QUANTMATHTESTER_ABI.clone(),
                QUANTMATHTESTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addTest` (0x3366567c) function"]
        pub fn add_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, FixedPointInt> {
            self.0
                .method_hash([51, 102, 86, 124], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `divTest` (0x46537197) function"]
        pub fn div_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, FixedPointInt> {
            self.0
                .method_hash([70, 83, 113, 151], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fromScaledUintTest` (0x77714493) function"]
        pub fn from_scaled_uint_test(
            &self,
            a: ethers::core::types::U256,
            decimals: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, FixedPointInt> {
            self.0
                .method_hash([119, 113, 68, 147], (a, decimals))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fromUnscaledIntTest` (0xe2052e0d) function"]
        pub fn from_unscaled_int_test(
            &self,
            a: I256,
        ) -> ethers::contract::builders::ContractCall<M, FixedPointInt> {
            self.0
                .method_hash([226, 5, 46, 13], a)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isEqualTest` (0xc334a50b) function"]
        pub fn is_equal_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([195, 52, 165, 11], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isGreaterThanOrEqualTest` (0xffd908f2) function"]
        pub fn is_greater_than_or_equal_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([255, 217, 8, 242], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isGreaterThanTest` (0x66829be9) function"]
        pub fn is_greater_than_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 130, 155, 233], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isLessThanOrEqualTest` (0xa07fdf1b) function"]
        pub fn is_less_than_or_equal_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([160, 127, 223, 27], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isLessThanTest` (0x186c1a06) function"]
        pub fn is_less_than_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([24, 108, 26, 6], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxTest` (0x33293dc0) function"]
        pub fn max_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, FixedPointInt> {
            self.0
                .method_hash([51, 41, 61, 192], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minTest` (0x23fe1cfb) function"]
        pub fn min_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, FixedPointInt> {
            self.0
                .method_hash([35, 254, 28, 251], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mulTest` (0x2d5a6b3f) function"]
        pub fn mul_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, FixedPointInt> {
            self.0
                .method_hash([45, 90, 107, 63], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `subTest` (0xdc9ebe0b) function"]
        pub fn sub_test(
            &self,
            a: FixedPointInt,
            b: FixedPointInt,
        ) -> ethers::contract::builders::ContractCall<M, FixedPointInt> {
            self.0
                .method_hash([220, 158, 190, 11], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `toScaledUintTest` (0xbb27b976) function"]
        pub fn to_scaled_uint_test(
            &self,
            a: FixedPointInt,
            decimals: ethers::core::types::U256,
            round_down: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([187, 39, 185, 118], (a, decimals, round_down))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for QuantMathTester<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `addTest` function with signature `addTest((int256),(int256))` and selector `[51, 102, 86, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addTest", abi = "addTest((int256),(int256))")]
    pub struct AddTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `divTest` function with signature `divTest((int256),(int256))` and selector `[70, 83, 113, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "divTest", abi = "divTest((int256),(int256))")]
    pub struct DivTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `fromScaledUintTest` function with signature `fromScaledUintTest(uint256,uint256)` and selector `[119, 113, 68, 147]`"]
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
        name = "fromScaledUintTest",
        abi = "fromScaledUintTest(uint256,uint256)"
    )]
    pub struct FromScaledUintTestCall {
        pub a: ethers::core::types::U256,
        pub decimals: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fromUnscaledIntTest` function with signature `fromUnscaledIntTest(int256)` and selector `[226, 5, 46, 13]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fromUnscaledIntTest", abi = "fromUnscaledIntTest(int256)")]
    pub struct FromUnscaledIntTestCall {
        pub a: I256,
    }
    #[doc = "Container type for all input parameters for the `isEqualTest` function with signature `isEqualTest((int256),(int256))` and selector `[195, 52, 165, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isEqualTest", abi = "isEqualTest((int256),(int256))")]
    pub struct IsEqualTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `isGreaterThanOrEqualTest` function with signature `isGreaterThanOrEqualTest((int256),(int256))` and selector `[255, 217, 8, 242]`"]
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
        name = "isGreaterThanOrEqualTest",
        abi = "isGreaterThanOrEqualTest((int256),(int256))"
    )]
    pub struct IsGreaterThanOrEqualTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `isGreaterThanTest` function with signature `isGreaterThanTest((int256),(int256))` and selector `[102, 130, 155, 233]`"]
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
        name = "isGreaterThanTest",
        abi = "isGreaterThanTest((int256),(int256))"
    )]
    pub struct IsGreaterThanTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `isLessThanOrEqualTest` function with signature `isLessThanOrEqualTest((int256),(int256))` and selector `[160, 127, 223, 27]`"]
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
        name = "isLessThanOrEqualTest",
        abi = "isLessThanOrEqualTest((int256),(int256))"
    )]
    pub struct IsLessThanOrEqualTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `isLessThanTest` function with signature `isLessThanTest((int256),(int256))` and selector `[24, 108, 26, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isLessThanTest", abi = "isLessThanTest((int256),(int256))")]
    pub struct IsLessThanTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `maxTest` function with signature `maxTest((int256),(int256))` and selector `[51, 41, 61, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxTest", abi = "maxTest((int256),(int256))")]
    pub struct MaxTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `minTest` function with signature `minTest((int256),(int256))` and selector `[35, 254, 28, 251]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "minTest", abi = "minTest((int256),(int256))")]
    pub struct MinTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `mulTest` function with signature `mulTest((int256),(int256))` and selector `[45, 90, 107, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mulTest", abi = "mulTest((int256),(int256))")]
    pub struct MulTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `subTest` function with signature `subTest((int256),(int256))` and selector `[220, 158, 190, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "subTest", abi = "subTest((int256),(int256))")]
    pub struct SubTestCall {
        pub a: FixedPointInt,
        pub b: FixedPointInt,
    }
    #[doc = "Container type for all input parameters for the `toScaledUintTest` function with signature `toScaledUintTest((int256),uint256,bool)` and selector `[187, 39, 185, 118]`"]
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
        name = "toScaledUintTest",
        abi = "toScaledUintTest((int256),uint256,bool)"
    )]
    pub struct ToScaledUintTestCall {
        pub a: FixedPointInt,
        pub decimals: ethers::core::types::U256,
        pub round_down: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum QuantMathTesterCalls {
        AddTest(AddTestCall),
        DivTest(DivTestCall),
        FromScaledUintTest(FromScaledUintTestCall),
        FromUnscaledIntTest(FromUnscaledIntTestCall),
        IsEqualTest(IsEqualTestCall),
        IsGreaterThanOrEqualTest(IsGreaterThanOrEqualTestCall),
        IsGreaterThanTest(IsGreaterThanTestCall),
        IsLessThanOrEqualTest(IsLessThanOrEqualTestCall),
        IsLessThanTest(IsLessThanTestCall),
        MaxTest(MaxTestCall),
        MinTest(MinTestCall),
        MulTest(MulTestCall),
        SubTest(SubTestCall),
        ToScaledUintTest(ToScaledUintTestCall),
    }
    impl ethers::core::abi::AbiDecode for QuantMathTesterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::AddTest(decoded));
            }
            if let Ok(decoded) =
                <DivTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::DivTest(decoded));
            }
            if let Ok(decoded) =
                <FromScaledUintTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::FromScaledUintTest(decoded));
            }
            if let Ok(decoded) =
                <FromUnscaledIntTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::FromUnscaledIntTest(decoded));
            }
            if let Ok(decoded) =
                <IsEqualTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::IsEqualTest(decoded));
            }
            if let Ok(decoded) =
                <IsGreaterThanOrEqualTestCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(QuantMathTesterCalls::IsGreaterThanOrEqualTest(decoded));
            }
            if let Ok(decoded) =
                <IsGreaterThanTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::IsGreaterThanTest(decoded));
            }
            if let Ok(decoded) =
                <IsLessThanOrEqualTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::IsLessThanOrEqualTest(decoded));
            }
            if let Ok(decoded) =
                <IsLessThanTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::IsLessThanTest(decoded));
            }
            if let Ok(decoded) =
                <MaxTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::MaxTest(decoded));
            }
            if let Ok(decoded) =
                <MinTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::MinTest(decoded));
            }
            if let Ok(decoded) =
                <MulTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::MulTest(decoded));
            }
            if let Ok(decoded) =
                <SubTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::SubTest(decoded));
            }
            if let Ok(decoded) =
                <ToScaledUintTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(QuantMathTesterCalls::ToScaledUintTest(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for QuantMathTesterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                QuantMathTesterCalls::AddTest(element) => element.encode(),
                QuantMathTesterCalls::DivTest(element) => element.encode(),
                QuantMathTesterCalls::FromScaledUintTest(element) => element.encode(),
                QuantMathTesterCalls::FromUnscaledIntTest(element) => element.encode(),
                QuantMathTesterCalls::IsEqualTest(element) => element.encode(),
                QuantMathTesterCalls::IsGreaterThanOrEqualTest(element) => element.encode(),
                QuantMathTesterCalls::IsGreaterThanTest(element) => element.encode(),
                QuantMathTesterCalls::IsLessThanOrEqualTest(element) => element.encode(),
                QuantMathTesterCalls::IsLessThanTest(element) => element.encode(),
                QuantMathTesterCalls::MaxTest(element) => element.encode(),
                QuantMathTesterCalls::MinTest(element) => element.encode(),
                QuantMathTesterCalls::MulTest(element) => element.encode(),
                QuantMathTesterCalls::SubTest(element) => element.encode(),
                QuantMathTesterCalls::ToScaledUintTest(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for QuantMathTesterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                QuantMathTesterCalls::AddTest(element) => element.fmt(f),
                QuantMathTesterCalls::DivTest(element) => element.fmt(f),
                QuantMathTesterCalls::FromScaledUintTest(element) => element.fmt(f),
                QuantMathTesterCalls::FromUnscaledIntTest(element) => element.fmt(f),
                QuantMathTesterCalls::IsEqualTest(element) => element.fmt(f),
                QuantMathTesterCalls::IsGreaterThanOrEqualTest(element) => element.fmt(f),
                QuantMathTesterCalls::IsGreaterThanTest(element) => element.fmt(f),
                QuantMathTesterCalls::IsLessThanOrEqualTest(element) => element.fmt(f),
                QuantMathTesterCalls::IsLessThanTest(element) => element.fmt(f),
                QuantMathTesterCalls::MaxTest(element) => element.fmt(f),
                QuantMathTesterCalls::MinTest(element) => element.fmt(f),
                QuantMathTesterCalls::MulTest(element) => element.fmt(f),
                QuantMathTesterCalls::SubTest(element) => element.fmt(f),
                QuantMathTesterCalls::ToScaledUintTest(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddTestCall> for QuantMathTesterCalls {
        fn from(var: AddTestCall) -> Self {
            QuantMathTesterCalls::AddTest(var)
        }
    }
    impl ::std::convert::From<DivTestCall> for QuantMathTesterCalls {
        fn from(var: DivTestCall) -> Self {
            QuantMathTesterCalls::DivTest(var)
        }
    }
    impl ::std::convert::From<FromScaledUintTestCall> for QuantMathTesterCalls {
        fn from(var: FromScaledUintTestCall) -> Self {
            QuantMathTesterCalls::FromScaledUintTest(var)
        }
    }
    impl ::std::convert::From<FromUnscaledIntTestCall> for QuantMathTesterCalls {
        fn from(var: FromUnscaledIntTestCall) -> Self {
            QuantMathTesterCalls::FromUnscaledIntTest(var)
        }
    }
    impl ::std::convert::From<IsEqualTestCall> for QuantMathTesterCalls {
        fn from(var: IsEqualTestCall) -> Self {
            QuantMathTesterCalls::IsEqualTest(var)
        }
    }
    impl ::std::convert::From<IsGreaterThanOrEqualTestCall> for QuantMathTesterCalls {
        fn from(var: IsGreaterThanOrEqualTestCall) -> Self {
            QuantMathTesterCalls::IsGreaterThanOrEqualTest(var)
        }
    }
    impl ::std::convert::From<IsGreaterThanTestCall> for QuantMathTesterCalls {
        fn from(var: IsGreaterThanTestCall) -> Self {
            QuantMathTesterCalls::IsGreaterThanTest(var)
        }
    }
    impl ::std::convert::From<IsLessThanOrEqualTestCall> for QuantMathTesterCalls {
        fn from(var: IsLessThanOrEqualTestCall) -> Self {
            QuantMathTesterCalls::IsLessThanOrEqualTest(var)
        }
    }
    impl ::std::convert::From<IsLessThanTestCall> for QuantMathTesterCalls {
        fn from(var: IsLessThanTestCall) -> Self {
            QuantMathTesterCalls::IsLessThanTest(var)
        }
    }
    impl ::std::convert::From<MaxTestCall> for QuantMathTesterCalls {
        fn from(var: MaxTestCall) -> Self {
            QuantMathTesterCalls::MaxTest(var)
        }
    }
    impl ::std::convert::From<MinTestCall> for QuantMathTesterCalls {
        fn from(var: MinTestCall) -> Self {
            QuantMathTesterCalls::MinTest(var)
        }
    }
    impl ::std::convert::From<MulTestCall> for QuantMathTesterCalls {
        fn from(var: MulTestCall) -> Self {
            QuantMathTesterCalls::MulTest(var)
        }
    }
    impl ::std::convert::From<SubTestCall> for QuantMathTesterCalls {
        fn from(var: SubTestCall) -> Self {
            QuantMathTesterCalls::SubTest(var)
        }
    }
    impl ::std::convert::From<ToScaledUintTestCall> for QuantMathTesterCalls {
        fn from(var: ToScaledUintTestCall) -> Self {
            QuantMathTesterCalls::ToScaledUintTest(var)
        }
    }
    #[doc = "Container type for all return fields from the `addTest` function with signature `addTest((int256),(int256))` and selector `[51, 102, 86, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddTestReturn(pub (I256,));
    #[doc = "Container type for all return fields from the `divTest` function with signature `divTest((int256),(int256))` and selector `[70, 83, 113, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DivTestReturn(pub (I256,));
    #[doc = "Container type for all return fields from the `fromScaledUintTest` function with signature `fromScaledUintTest(uint256,uint256)` and selector `[119, 113, 68, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FromScaledUintTestReturn(pub (I256,));
    #[doc = "Container type for all return fields from the `fromUnscaledIntTest` function with signature `fromUnscaledIntTest(int256)` and selector `[226, 5, 46, 13]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FromUnscaledIntTestReturn(pub (I256,));
    #[doc = "Container type for all return fields from the `isEqualTest` function with signature `isEqualTest((int256),(int256))` and selector `[195, 52, 165, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsEqualTestReturn(pub bool);
    #[doc = "Container type for all return fields from the `isGreaterThanOrEqualTest` function with signature `isGreaterThanOrEqualTest((int256),(int256))` and selector `[255, 217, 8, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsGreaterThanOrEqualTestReturn(pub bool);
    #[doc = "Container type for all return fields from the `isGreaterThanTest` function with signature `isGreaterThanTest((int256),(int256))` and selector `[102, 130, 155, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsGreaterThanTestReturn(pub bool);
    #[doc = "Container type for all return fields from the `isLessThanOrEqualTest` function with signature `isLessThanOrEqualTest((int256),(int256))` and selector `[160, 127, 223, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsLessThanOrEqualTestReturn(pub bool);
    #[doc = "Container type for all return fields from the `isLessThanTest` function with signature `isLessThanTest((int256),(int256))` and selector `[24, 108, 26, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsLessThanTestReturn(pub bool);
    #[doc = "Container type for all return fields from the `maxTest` function with signature `maxTest((int256),(int256))` and selector `[51, 41, 61, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MaxTestReturn(pub (I256,));
    #[doc = "Container type for all return fields from the `minTest` function with signature `minTest((int256),(int256))` and selector `[35, 254, 28, 251]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MinTestReturn(pub (I256,));
    #[doc = "Container type for all return fields from the `mulTest` function with signature `mulTest((int256),(int256))` and selector `[45, 90, 107, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MulTestReturn(pub (I256,));
    #[doc = "Container type for all return fields from the `subTest` function with signature `subTest((int256),(int256))` and selector `[220, 158, 190, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SubTestReturn(pub (I256,));
    #[doc = "Container type for all return fields from the `toScaledUintTest` function with signature `toScaledUintTest((int256),uint256,bool)` and selector `[187, 39, 185, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ToScaledUintTestReturn(pub ethers::core::types::U256);
    #[doc = "`FixedPointInt(int256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FixedPointInt {
        pub value: I256,
    }
}
