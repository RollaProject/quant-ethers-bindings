pub use quantmathtester_mod::*;
#[allow(clippy::too_many_arguments)]
mod quantmathtester_mod {
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
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"addTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"divTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"fromScaledUintTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"a\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"fromUnscaledIntTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isEqualTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isGreaterThanOrEqualTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isGreaterThanTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isLessThanOrEqualTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"isLessThanTest\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"maxTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"minTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"mulTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"subTest\",\"outputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct QuantMath.FixedPointInt\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"roundDown\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"toScaledUintTest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static QUANTMATHTESTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461001657610e16908161001c8239f35b600080fdfe60806040526004361015610013575b600080fd5b60003560e01c8063186c1a061461012b57806323fe1cfb146101225780632d5a6b3f1461011957806333293dc0146101105780633366567c1461010757806346537197146100fe57806366829be9146100f557806377714493146100ec578063a07fdf1b146100e3578063bb27b976146100da578063c334a50b146100d1578063dc9ebe0b146100c8578063e2052e0d146100bf5763ffd908f2146100b757600080fd5b61000e6109e7565b5061000e610994565b5061000e6108b5565b5061000e610864565b5061000e61074f565b5061000e6106fd565b5061000e610615565b5061000e6105c4565b5061000e6104a8565b5061000e6103c9565b5061000e61035b565b5061000e6102be565b5061000e610248565b5061000e6101f7565b604051906020820182811067ffffffffffffffff82111761015457604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc602091011261000e576101b5610134565b906004358252565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdc602091011261000e576101ef610134565b906024358252565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061023236610183565b61023b366101bd565b9051905113604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061028336610183565b61028c366101bd565b6000610296610134565b5260006102a1610134565b528151815113156102b757505b60405190518152f35b90506102ae565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206103496b033b2e3c9fd0803ce800000061034361030c36610183565b610315366101bd565b906000610320610134565b52600061032b610134565b526103398151835190610ae8565b5051905190610ae8565b05610d68565b80610352610134565b52604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061039636610183565b61039f366101bd565b60006103a9610134565b5260006103b4610134565b528151815112156102b7575060405190518152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061040436610183565b61040d366101bd565b906000610418610134565b526000610423610134565b525190519060008112817f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03831381151661049b575b817f80000000000000000000000000000000000000000000000000000000000000000383121661048e575b0180610352610134565b610496610a39565b610484565b6104a3610a39565b610459565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761059961057e6104e736610183565b6105236104f3366101bd565b9160006104fe610134565b526000610509610134565b526105148151610a69565b508251156105b7575b51610a69565b90519081156105aa575b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82147f800000000000000000000000000000000000000000000000000000000000000082141661059d5705610d68565b610586610134565b8190526040519081529081906020820190565b0390f35b6105a5610a39565b610343565b6105b2610c24565b61052d565b6105bf610c24565b61051d565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206105ff36610183565b610608366101bd565b9051905112604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576105996004356024356000610658610134565b526000610663610134565b52600061066e610134565b52601b8103610692575061057e90610c6a565b604051905181529081906020820190565b601b8111156106ce576106be916106b36106ae6106b993610b9d565b610bd7565b90610c54565b610c6a565b6106c6610134565b908152610681565b6106be916106ea826106b993601b106106f0575b601b03610bd7565b90610bf3565b6106f8610a39565b6106e2565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061073836610183565b610741366101bd565b905190511215604051908152f35b503461000e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761078836610183565b6024356044359182159283150361000e57610599926000601b84036107c45750506107b4915051610cff565b6040519081529081906020820190565b601b8411156107f35750506106ea6107e86107e16107ee94610b9d565b9251610cff565b91610bd7565b6107b4565b836107ee94601b10610857575b601b039091610820575b6106b36107e861081b939451610cff565b610cf3565b61082a8351610cff565b61083382610bd7565b90811561084a575b061561080a576001915061080a565b610852610c24565b61083b565b61085f610a39565b610800565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061089f36610183565b6108a8366101bd565b9051905114604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206108f036610183565b6108f9366101bd565b906000610904610134565b52600061090f610134565b525190519060008212827f8000000000000000000000000000000000000000000000000000000000000000018212811516610987575b827f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0182131661097a575b0380610352610134565b610982610a39565b610970565b61098f610a39565b610945565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760006109ce610134565b5260006109d9610134565b526020610349600435610a69565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610a2236610183565b610a2b366101bd565b905190511315604051908152f35b507f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b6b033b2e3c9fd0803ce800000090600174279d346de4781f921dd7a89933d54d1f72927c70148211600083131616610adb575b60017fffffffffffffffffffffffd862cb921b87e06de2285766cc2ab2e08d6d838fec8212600083121616610acf570290565b610ad7610a39565b0290565b610ae3610a39565b610a9c565b600082136000821390837f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff818104851183851616610b90575b60008212927f800000000000000000000000000000000000000000000000000000000000000094848787058512911616610b83575b82600087129505861290851616610b76575b058312911616610acf570290565b610b7e610a39565b610b68565b610b8b610a39565b610b56565b610b98610a39565b610b21565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe590601b8110610bcb570190565b610bd3610a39565b0190565b604d8111610be6575b600a0a90565b610bee610a39565b610be0565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04821181151516610acf570290565b507f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b8115610c5e570490565b610c66610c24565b0490565b7f8000000000000000000000000000000000000000000000000000000000000000811015610c955790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f5175616e744d6174683a206f7574206f6620696e742072616e676500000000006044820152fd5b81198111610bcb570190565b60008112610d0a5790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5175616e744d6174683a206e6567617469766520696e740000000000000000006044820152fd5b60008112817f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03600013811516610dd3575b817f80000000000000000000000000000000000000000000000000000000000000000360001216610dc85790565b610dd0610a39565b90565b610ddb610a39565b610d9a56fea26469706673582212205a820afb84957f90002b341e58b51416632051b160f18c1a071eb6b65ee03b7a64736f6c634300080e0033" . parse () . expect ("invalid bytecode")
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
    #[doc = "Container type for all input parameters for the `addTest`function with signature `addTest((int256),(int256))` and selector `[51, 102, 86, 124]`"]
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
    #[doc = "Container type for all input parameters for the `divTest`function with signature `divTest((int256),(int256))` and selector `[70, 83, 113, 151]`"]
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
    #[doc = "Container type for all input parameters for the `fromScaledUintTest`function with signature `fromScaledUintTest(uint256,uint256)` and selector `[119, 113, 68, 147]`"]
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
    #[doc = "Container type for all input parameters for the `fromUnscaledIntTest`function with signature `fromUnscaledIntTest(int256)` and selector `[226, 5, 46, 13]`"]
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
    #[doc = "Container type for all input parameters for the `isEqualTest`function with signature `isEqualTest((int256),(int256))` and selector `[195, 52, 165, 11]`"]
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
    #[doc = "Container type for all input parameters for the `isGreaterThanOrEqualTest`function with signature `isGreaterThanOrEqualTest((int256),(int256))` and selector `[255, 217, 8, 242]`"]
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
    #[doc = "Container type for all input parameters for the `isGreaterThanTest`function with signature `isGreaterThanTest((int256),(int256))` and selector `[102, 130, 155, 233]`"]
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
    #[doc = "Container type for all input parameters for the `isLessThanOrEqualTest`function with signature `isLessThanOrEqualTest((int256),(int256))` and selector `[160, 127, 223, 27]`"]
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
    #[doc = "Container type for all input parameters for the `isLessThanTest`function with signature `isLessThanTest((int256),(int256))` and selector `[24, 108, 26, 6]`"]
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
    #[doc = "Container type for all input parameters for the `maxTest`function with signature `maxTest((int256),(int256))` and selector `[51, 41, 61, 192]`"]
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
    #[doc = "Container type for all input parameters for the `minTest`function with signature `minTest((int256),(int256))` and selector `[35, 254, 28, 251]`"]
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
    #[doc = "Container type for all input parameters for the `mulTest`function with signature `mulTest((int256),(int256))` and selector `[45, 90, 107, 63]`"]
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
    #[doc = "Container type for all input parameters for the `subTest`function with signature `subTest((int256),(int256))` and selector `[220, 158, 190, 11]`"]
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
    #[doc = "Container type for all input parameters for the `toScaledUintTest`function with signature `toScaledUintTest((int256),uint256,bool)` and selector `[187, 39, 185, 118]`"]
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
