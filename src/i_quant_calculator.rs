pub use iquantcalculator_mod::*;
#[allow(clippy::too_many_arguments)]
mod iquantcalculator_mod {
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
    #[doc = "IQuantCalculator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IQUANTCALCULATOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetsRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_collateralTokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calculateClaimableCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"returnableCollateral\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToClaim\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qTokenToMint\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenForCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCollateralRequirement\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"collateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getExercisePayout\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isSettled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"payoutToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"payoutAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qTokenShort\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenLong\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amountToNeutralize\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNeutralizationPayout\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"collateralType\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralOwed\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"optionsDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"optionsFactory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"strikeAssetDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IQuantCalculator<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IQuantCalculator<M> {
        fn clone(&self) -> Self {
            IQuantCalculator(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IQuantCalculator<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IQuantCalculator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IQuantCalculator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IQuantCalculator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IQUANTCALCULATOR_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `assetsRegistry` (0x911303b4) function"]
        pub fn assets_registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([145, 19, 3, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculateClaimableCollateral` (0x2a2d7d94) function"]
        pub fn calculate_claimable_collateral(
            &self,
            collateral_token_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::Address,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([42, 45, 125, 148], (collateral_token_id, amount, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCollateralRequirement` (0x1df82e59) function"]
        pub fn get_collateral_requirement(
            &self,
            q_token_to_mint: ethers::core::types::Address,
            q_token_for_collateral: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [29, 248, 46, 89],
                    (q_token_to_mint, q_token_for_collateral, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getExercisePayout` (0x57284c2a) function"]
        pub fn get_exercise_payout(
            &self,
            q_token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ethers::core::types::Address,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([87, 40, 76, 42], (q_token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNeutralizationPayout` (0x099ca872) function"]
        pub fn get_neutralization_payout(
            &self,
            q_token_short: ethers::core::types::Address,
            q_token_long: ethers::core::types::Address,
            amount_to_neutralize: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [9, 156, 168, 114],
                    (q_token_short, q_token_long, amount_to_neutralize),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `optionsDecimals` (0x8493cec1) function"]
        pub fn options_decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([132, 147, 206, 193], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `optionsFactory` (0xe66ef2c4) function"]
        pub fn options_factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([230, 110, 242, 196], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `priceRegistry` (0x535131d7) function"]
        pub fn price_registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([83, 81, 49, 215], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `strikeAssetDecimals` (0xc1325661) function"]
        pub fn strike_asset_decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([193, 50, 86, 97], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IQuantCalculator<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `assetsRegistry`function with signature `assetsRegistry()` and selector `[145, 19, 3, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assetsRegistry", abi = "assetsRegistry()")]
    pub struct AssetsRegistryCall;
    #[doc = "Container type for all input parameters for the `calculateClaimableCollateral`function with signature `calculateClaimableCollateral(uint256,uint256,address)` and selector `[42, 45, 125, 148]`"]
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
        name = "calculateClaimableCollateral",
        abi = "calculateClaimableCollateral(uint256,uint256,address)"
    )]
    pub struct CalculateClaimableCollateralCall {
        pub collateral_token_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCollateralRequirement`function with signature `getCollateralRequirement(address,address,uint256)` and selector `[29, 248, 46, 89]`"]
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
        name = "getCollateralRequirement",
        abi = "getCollateralRequirement(address,address,uint256)"
    )]
    pub struct GetCollateralRequirementCall {
        pub q_token_to_mint: ethers::core::types::Address,
        pub q_token_for_collateral: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getExercisePayout`function with signature `getExercisePayout(address,uint256)` and selector `[87, 40, 76, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getExercisePayout", abi = "getExercisePayout(address,uint256)")]
    pub struct GetExercisePayoutCall {
        pub q_token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getNeutralizationPayout`function with signature `getNeutralizationPayout(address,address,uint256)` and selector `[9, 156, 168, 114]`"]
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
        name = "getNeutralizationPayout",
        abi = "getNeutralizationPayout(address,address,uint256)"
    )]
    pub struct GetNeutralizationPayoutCall {
        pub q_token_short: ethers::core::types::Address,
        pub q_token_long: ethers::core::types::Address,
        pub amount_to_neutralize: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `optionsDecimals`function with signature `optionsDecimals()` and selector `[132, 147, 206, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "optionsDecimals", abi = "optionsDecimals()")]
    pub struct OptionsDecimalsCall;
    #[doc = "Container type for all input parameters for the `optionsFactory`function with signature `optionsFactory()` and selector `[230, 110, 242, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "optionsFactory", abi = "optionsFactory()")]
    pub struct OptionsFactoryCall;
    #[doc = "Container type for all input parameters for the `priceRegistry`function with signature `priceRegistry()` and selector `[83, 81, 49, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceRegistry", abi = "priceRegistry()")]
    pub struct PriceRegistryCall;
    #[doc = "Container type for all input parameters for the `strikeAssetDecimals`function with signature `strikeAssetDecimals()` and selector `[193, 50, 86, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "strikeAssetDecimals", abi = "strikeAssetDecimals()")]
    pub struct StrikeAssetDecimalsCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IQuantCalculatorCalls {
        AssetsRegistry(AssetsRegistryCall),
        CalculateClaimableCollateral(CalculateClaimableCollateralCall),
        GetCollateralRequirement(GetCollateralRequirementCall),
        GetExercisePayout(GetExercisePayoutCall),
        GetNeutralizationPayout(GetNeutralizationPayoutCall),
        OptionsDecimals(OptionsDecimalsCall),
        OptionsFactory(OptionsFactoryCall),
        PriceRegistry(PriceRegistryCall),
        StrikeAssetDecimals(StrikeAssetDecimalsCall),
    }
    impl ethers::core::abi::AbiDecode for IQuantCalculatorCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssetsRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuantCalculatorCalls::AssetsRegistry(decoded));
            }
            if let Ok(decoded) =
                <CalculateClaimableCollateralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IQuantCalculatorCalls::CalculateClaimableCollateral(decoded));
            }
            if let Ok(decoded) =
                <GetCollateralRequirementCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IQuantCalculatorCalls::GetCollateralRequirement(decoded));
            }
            if let Ok(decoded) =
                <GetExercisePayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuantCalculatorCalls::GetExercisePayout(decoded));
            }
            if let Ok(decoded) =
                <GetNeutralizationPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuantCalculatorCalls::GetNeutralizationPayout(decoded));
            }
            if let Ok(decoded) =
                <OptionsDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuantCalculatorCalls::OptionsDecimals(decoded));
            }
            if let Ok(decoded) =
                <OptionsFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuantCalculatorCalls::OptionsFactory(decoded));
            }
            if let Ok(decoded) =
                <PriceRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuantCalculatorCalls::PriceRegistry(decoded));
            }
            if let Ok(decoded) =
                <StrikeAssetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuantCalculatorCalls::StrikeAssetDecimals(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IQuantCalculatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IQuantCalculatorCalls::AssetsRegistry(element) => element.encode(),
                IQuantCalculatorCalls::CalculateClaimableCollateral(element) => element.encode(),
                IQuantCalculatorCalls::GetCollateralRequirement(element) => element.encode(),
                IQuantCalculatorCalls::GetExercisePayout(element) => element.encode(),
                IQuantCalculatorCalls::GetNeutralizationPayout(element) => element.encode(),
                IQuantCalculatorCalls::OptionsDecimals(element) => element.encode(),
                IQuantCalculatorCalls::OptionsFactory(element) => element.encode(),
                IQuantCalculatorCalls::PriceRegistry(element) => element.encode(),
                IQuantCalculatorCalls::StrikeAssetDecimals(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IQuantCalculatorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IQuantCalculatorCalls::AssetsRegistry(element) => element.fmt(f),
                IQuantCalculatorCalls::CalculateClaimableCollateral(element) => element.fmt(f),
                IQuantCalculatorCalls::GetCollateralRequirement(element) => element.fmt(f),
                IQuantCalculatorCalls::GetExercisePayout(element) => element.fmt(f),
                IQuantCalculatorCalls::GetNeutralizationPayout(element) => element.fmt(f),
                IQuantCalculatorCalls::OptionsDecimals(element) => element.fmt(f),
                IQuantCalculatorCalls::OptionsFactory(element) => element.fmt(f),
                IQuantCalculatorCalls::PriceRegistry(element) => element.fmt(f),
                IQuantCalculatorCalls::StrikeAssetDecimals(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssetsRegistryCall> for IQuantCalculatorCalls {
        fn from(var: AssetsRegistryCall) -> Self {
            IQuantCalculatorCalls::AssetsRegistry(var)
        }
    }
    impl ::std::convert::From<CalculateClaimableCollateralCall> for IQuantCalculatorCalls {
        fn from(var: CalculateClaimableCollateralCall) -> Self {
            IQuantCalculatorCalls::CalculateClaimableCollateral(var)
        }
    }
    impl ::std::convert::From<GetCollateralRequirementCall> for IQuantCalculatorCalls {
        fn from(var: GetCollateralRequirementCall) -> Self {
            IQuantCalculatorCalls::GetCollateralRequirement(var)
        }
    }
    impl ::std::convert::From<GetExercisePayoutCall> for IQuantCalculatorCalls {
        fn from(var: GetExercisePayoutCall) -> Self {
            IQuantCalculatorCalls::GetExercisePayout(var)
        }
    }
    impl ::std::convert::From<GetNeutralizationPayoutCall> for IQuantCalculatorCalls {
        fn from(var: GetNeutralizationPayoutCall) -> Self {
            IQuantCalculatorCalls::GetNeutralizationPayout(var)
        }
    }
    impl ::std::convert::From<OptionsDecimalsCall> for IQuantCalculatorCalls {
        fn from(var: OptionsDecimalsCall) -> Self {
            IQuantCalculatorCalls::OptionsDecimals(var)
        }
    }
    impl ::std::convert::From<OptionsFactoryCall> for IQuantCalculatorCalls {
        fn from(var: OptionsFactoryCall) -> Self {
            IQuantCalculatorCalls::OptionsFactory(var)
        }
    }
    impl ::std::convert::From<PriceRegistryCall> for IQuantCalculatorCalls {
        fn from(var: PriceRegistryCall) -> Self {
            IQuantCalculatorCalls::PriceRegistry(var)
        }
    }
    impl ::std::convert::From<StrikeAssetDecimalsCall> for IQuantCalculatorCalls {
        fn from(var: StrikeAssetDecimalsCall) -> Self {
            IQuantCalculatorCalls::StrikeAssetDecimals(var)
        }
    }
}
