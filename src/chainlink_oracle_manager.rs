pub use chainlinkoraclemanager_mod::*;
#[allow(clippy::too_many_arguments)]
mod chainlinkoraclemanager_mod {
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
    #[doc = "ChainlinkOracleManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CHAINLINKORACLEMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_priceRegistry\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_strikeAssetDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_fallbackPeriodSeconds\",\"type\":\"uint88\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint88\",\"name\":\"expiryTimestamp\",\"type\":\"uint88\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expiryRoundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"priceSubmitter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isFallback\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceRegistrySubmission\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetOracles\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fallbackPeriodSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidOption\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"searchRoundToSubmit\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistry\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_roundIdAfterExpiry\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryByRound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryFallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"strikeAssetDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CHAINLINKORACLEMANAGER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60e034620000cf57601f6200234438819003918201601f19168301916001600160401b03831184841017620000d457808492606094604052833981010312620000cf578051906001600160a01b0382168203620000cf5760208101519060ff82168203620000cf5760400151906001600160581b0382168203620000cf576200008892620000ea565b6040516121899081620001bb82396080518181816103d0015281816104e40152611aee015260a0518181816104380152610b6d015260c0518181816107f00152610d6f0152f35b600080fd5b634e487b7160e01b600052604160045260246000fd5b60008054336001600160a01b0319821681178355604051949594926001600160a01b03928316907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09085a38216156200015257506080526001600160581b031660a05260c052565b62461bcd60e51b815260206004820152603560248201527f50726f76696465724f7261636c654d616e616765723a20696e76616c6964207060448201527f72696365207265676973747279206164647265737300000000000000000000006064820152608490fdfe60806040526004361015610013575b600080fd5b60003560e01c80630a821fea146101675780630db015f41461015e5780632b663986146101555780634ac536531461014c578063535131d7146101435780636ee3827b1461013a578063715018a61461013157806384cc315b146101285780638cb4ba231461011f5780638da5cb5b14610116578063a89d490c1461010d578063ab91dcb914610104578063b9bd8796146100fb578063c1325661146100f2578063cf35bdd0146100e9578063e2411261146100e05763f2fde38b146100d857600080fd5b61000e610e5c565b5061000e610e19565b5061000e610d93565b5061000e610d36565b5061000e610c5e565b5061000e610b36565b5061000e610af9565b5061000e610aa6565b5061000e6108a0565b5061000e61070a565b5061000e610662565b5061000e6103f4565b5061000e610384565b5061000e61035f565b5061000e61028f565b5061000e61022f565b5061000e6101d0565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b602435906affffffffffffffffffffff8216820361000e57565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061021b61020d610170565b6102156101b6565b906115f7565b69ffffffffffffffffffff60405191168152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602061027161026c610170565b612030565b73ffffffffffffffffffffffffffffffffffffffff60405191168152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602073ffffffffffffffffffffffffffffffffffffffff806102df610170565b166000526001825260406000205416604051908152f35b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc606091011261000e5760043573ffffffffffffffffffffffffffffffffffffffff8116810361000e57906024356affffffffffffffffffffff8116810361000e579060443590565b503461000e57610377610371366102f6565b91611974565b005b600091031261000e57565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461000e57610403366102f6565b9091600473ffffffffffffffffffffffffffffffffffffffff916000948361043087958654163314610f6e565b61047261046a7f00000000000000000000000000000000000000000000000000000000000000006affffffffffffffffffffff8516611003565b42101561101b565b6040805173ffffffffffffffffffffffffffffffffffffffff851681526affffffffffffffffffffff8416602082015290810187905260006060820152336080820152600160a08201527f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e0799060c090a17f00000000000000000000000000000000000000000000000000000000000000001691602061052f61051661051684612030565b73ffffffffffffffffffffffffffffffffffffffff1690565b604051958680927f313ce5670000000000000000000000000000000000000000000000000000000082525afa938415610655575b8594610625575b50823b15610621576040517feae65f0800000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90911660048201526affffffffffffffffffffff909116602482015260ff909216604483015260648201939093529182908183816084810103925af18015610614575b6105fb575b50604051f35b8061060861060e92610bc0565b80610379565b386105f5565b61061c6110e5565b6105f0565b8480fd5b61064791945060203d811161064e575b61063f8183610bfd565b8101906110cc565b923861056a565b503d610635565b61065d6110e5565b610563565b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126107075780547fffffffffffffffffffffffff000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff8216916106d9338414610f6e565b16825581604051917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b80fd5b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576108266108166107ec61075461051661051661026c610170565b604051907ffeaf968c00000000000000000000000000000000000000000000000000000000825260a082600481845afa918215610893575b600092610855575b506004602060ff926107a86000861361113e565b604051928380927f313ce5670000000000000000000000000000000000000000000000000000000082525afa908115610848575b60009161082a575b501690611289565b60ff7f00000000000000000000000000000000000000000000000000000000000000001690611327565b6040519081529081906020820190565b0390f35b610842915060203d811161064e5761063f8183610bfd565b386107e4565b6108506110e5565b6107dc565b60ff919250602061087e60049260a03d811161088c575b6108768183610bfd565b810190611109565b505050905093925050610794565b503d61086c565b61089b6110e5565b61078c565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576108d8610170565b6108e0610193565b73ffffffffffffffffffffffffffffffffffffffff61090481600054163314610f6e565b811615610a22578161098061097a6105166109607f828d2be040dede7698182e08dfa8bfbd663c879aee772509c4a2bd961d0ed43f9673ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b5473ffffffffffffffffffffffffffffffffffffffff1690565b15611edd565b61098981611f68565b6109f7826109b78373ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b9073ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b6040805173ffffffffffffffffffffffffffffffffffffffff928316815292909116602083015290a1005b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602d60248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520697360448201527f207a65726f2061646472657373000000000000000000000000000000000000006064820152fd5b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602073ffffffffffffffffffffffffffffffffffffffff60005416604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020600254604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b67ffffffffffffffff8111610bd457604052565b610bdc610b90565b604052565b6020810190811067ffffffffffffffff821117610bd457604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610bd457604052565b604051906080820182811067ffffffffffffffff821117610bd457604052565b503461000e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610c96610170565b610c9e6101b6565b67ffffffffffffffff9160443583811161000e573660238201121561000e578060040135938411610d29575b60405190610d0060207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8801160183610bfd565b848252366024868301011161000e57846000926103779660246020940184830137010152610f4c565b610d31610b90565b610cca565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602060405160ff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043560025481101561000e5773ffffffffffffffffffffffffffffffffffffffff60209160026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace015416604051908152f35b503461000e576020610e2a366102f6565b505073ffffffffffffffffffffffffffffffffffffffff80911660005260018252604060002054161515604051908152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610e94610170565b73ffffffffffffffffffffffffffffffffffffffff610eb881600054163314610f6e565b811615610ec857610377906120e4565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b610f6c9169ffffffffffffffffffff610f6582846115f7565b1691611974565b565b15610f7557565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b507f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b8119811161100f570190565b611017610fd3565b0190565b1561102257565b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152605460248201527f436861696e6c696e6b4f7261636c654d616e616765723a205468652066616c6c60448201527f6261636b20707269636520706572696f6420686173206e6f742070617373656460648201527f2073696e6365207468652074696d657374616d700000000000000000000000006084820152fd5b9081602091031261000e575160ff8116810361000e5790565b506040513d6000823e3d90fd5b519069ffffffffffffffffffff8216820361000e57565b908160a091031261000e5761111d816110f2565b9160208201519160408101519161113b6080606084015193016110f2565b90565b1561114557565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603160248201527f436861696e6c696e6b4f7261636c654d616e616765723a204e6f20707269636960448201527f6e67206461746120617661696c61626c650000000000000000000000000000006064820152fd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe590601b811061100f570190565b604d8111611206575b600a0a90565b61120e610fd3565b611200565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04821181151516611244570290565b61124c610fd3565b0290565b811561125a570490565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b90600060405161129881610be1565b5260006040516112a781610be1565b52601b81036112cc57506112ba90611393565b604051906112c782610be1565b815290565b601b8111156112f8576112ba916112ed6112e86112f3936111c9565b6111f7565b90611250565b611393565b6112ba91611314826112f393601b1061131a575b601b036111f7565b90611213565b611322610fd3565b61130c565b90601b810361133b575061113b905161141c565b601b811115611364579061131461135e61135761113b946111c9565b925161141c565b916111f7565b6112ed61137d61113b9383601b10611386575b5161141c565b91601b036111f7565b61138e610fd3565b611377565b7f80000000000000000000000000000000000000000000000000000000000000008110156113be5790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f5175616e744d6174683a206f7574206f6620696e742072616e676500000000006044820152fd5b600081126114275790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5175616e744d6174683a206e6567617469766520696e740000000000000000006044820152fd5b9081602091031261000e575190565b1561149b57565b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152605460248201527f436861696e6c696e6b4f7261636c654d616e616765723a20546865206c61746560448201527f737420726f756e642074696d657374616d70206973206e6f742061667465722060648201527f746865206578706972792074696d657374616d700000000000000000000000006084820152fd5b1561154c57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603d60248201527f436861696e6c696e6b4f7261636c654d616e616765723a204e6f7420656e6f7560448201527f676820726f756e647320746f2066696e6420726f756e642061667465720000006064820152fd5b69ffffffffffffffffffff91821691168181106115eb570390565b6115f3610fd3565b0390565b9161160761051661168794612030565b9073ffffffffffffffffffffffffffffffffffffffff82169269ffffffffffffffffffff60046040958651907f8205bf6a000000000000000000000000000000000000000000000000000000008252602098828a81868296865afa9081156117cd575b6000916117b0575b506affffffffffffffffffffff871610611494565b8751928380927f668a0f020000000000000000000000000000000000000000000000000000000082525afa9081156117a3575b600091611776575b50166001806116ee6116df82856116da828211611545565b6115d0565b69ffffffffffffffffffff1690565b1461176c57906116ff918386611d58565b92611714845169ffffffffffffffffffff1690565b9260016116ee6116df6117338a89015169ffffffffffffffffffff1690565b97611761606061174f8c84015169ffffffffffffffffffff1690565b92015169ffffffffffffffffffff1690565b9895989794976115d0565b5093505092505090565b6117969150873d891161179c575b61178e8183610bfd565b810190611485565b386116c2565b503d611784565b6117ab6110e5565b6116ba565b6117c79150843d861161179c5761178e8183610bfd565b38611672565b6117d56110e5565b61166a565b156117e157565b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152604a60248201527f436861696e6c696e6b4f7261636c654d616e616765723a2054686520726f756e60448201527f6420706f73746564206973206e6f74206166746572207468652065787069727960648201527f2074696d657374616d70000000000000000000000000000000000000000000006084820152fd5b67ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff91166001811061100f570190565b156118ca57565b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152605a60248201527f436861696e6c696e6b4f7261636c654d616e616765723a20457870697279207260448201527f6f756e64207072696f7220746f20746865206f6e6520706f737465642069732060648201527f616674657220746865206578706972792074696d657374616d700000000000006084820152fd5b9190829161198461051684612030565b91600473ffffffffffffffffffffffffffffffffffffffff92838516937f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e079611a9e604051977fb633620c0000000000000000000000000000000000000000000000000000000090818a52611a9960209a878c828d8180611a0d8f9e869f83019190602083019252565b03915afa918215611c69575b600092611c4a575b50611a3b6affffffffffffffffffffff8c168093116117da565b67ffffffffffffffff69ffff0000000000000000611a5a82841661188b565b92169116179360405190815288818d8180611a7d8f8b9083019190602083019252565b03915afa908115611c3d575b600091611c20575b5011156118c3565b611c76565b6040805173ffffffffffffffffffffffffffffffffffffffff9c909c168c526affffffffffffffffffffff881660208d01528b0182905260608b01523360808b0152600060a08b01529860c090a17f00000000000000000000000000000000000000000000000000000000000000001693604051928380927f313ce5670000000000000000000000000000000000000000000000000000000082525afa938415611c13575b600094611bf4575b5050813b1561000e576040517feae65f0800000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90941660048501526affffffffffffffffffffff16602484015260ff909116604483015260648201929092529060009082908183816084810103925af18015611be7575b611bda5750565b80610608610f6c92610bc0565b611bef6110e5565b611bd3565b611c0b929450803d1061064e5761063f8183610bfd565b913880611b4b565b611c1b6110e5565b611b43565b611c379150893d8b1161179c5761178e8183610bfd565b38611a91565b611c456110e5565b611a89565b611c62919250893d8b1161179c5761178e8183610bfd565b9038611a21565b611c716110e5565b611a19565b60a073ffffffffffffffffffffffffffffffffffffffff919392936024604051809481937f9a6fc8f500000000000000000000000000000000000000000000000000000000835269ffffffffffffffffffff89166004840152165afa908115611d07575b600091611ce657509190565b611cfe915060a03d811161088c576108768183610bfd565b50505090509190565b611d0f6110e5565b611cda565b604051906080820182811067ffffffffffffffff821117611d4b575b60405260006060838281528260208201528260408201520152565b611d53610b90565b611d30565b919290611d63611d14565b5067ffffffffffffffff80831690851692611d8a6116df611d848487611003565b60011c90565b9469ffff0000000000000000821669ffffffffffffffffffff87161793602060405180937fb633620c0000000000000000000000000000000000000000000000000000000082528173ffffffffffffffffffffffffffffffffffffffff81611dfa8b600483019190602083019252565b0392165afa918215611ed0575b600092611ea3575b506affffffffffffffffffffff161015611e8757505061113b9291611e61611e7492611e4e611e3c610c3e565b69ffffffffffffffffffff9098168852565b69ffffffffffffffffffff166020870152565b69ffffffffffffffffffff166040850152565b69ffffffffffffffffffff166060830152565b611e7492955061113b94919350611e6190611e4e611e3c610c3e565b6affffffffffffffffffffff919250611ec99060203d811161179c5761178e8183610bfd565b9190611e0f565b611ed86110e5565b611e07565b15611ee457565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603360248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520616c60448201527f72656164792073657420666f72206173736574000000000000000000000000006064820152fd5b60025468010000000000000000811015612023575b6001810180600255811015611ff45773ffffffffffffffffffffffffffffffffffffffff9060026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace0191167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b61202b610b90565b611f7d565b73ffffffffffffffffffffffffffffffffffffffff80911660005260016020526040600020541680156120605790565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603a60248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520646f60448201527f65736e277420657869737420666f7220746861742061737365740000000000006064820152fd5b6000549073ffffffffffffffffffffffffffffffffffffffff80911691827fffffffffffffffffffffffff0000000000000000000000000000000000000000821617600055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e06000604051a356fea2646970667358221220332e8129ce36ae1cf9c33b272aa5a480b61e70929356fa2687536dca778797fc64736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ChainlinkOracleManager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ChainlinkOracleManager<M> {
        fn clone(&self) -> Self {
            ChainlinkOracleManager(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ChainlinkOracleManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ChainlinkOracleManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ChainlinkOracleManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ChainlinkOracleManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                CHAINLINKORACLEMANAGER_ABI.clone(),
                client,
            )
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
                CHAINLINKORACLEMANAGER_ABI.clone(),
                CHAINLINKORACLEMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addAssetOracle` (0x8cb4ba23) function"]
        pub fn add_asset_oracle(
            &self,
            asset: ethers::core::types::Address,
            oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 180, 186, 35], (asset, oracle))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetOracles` (0x2b663986) function"]
        pub fn asset_oracles(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([43, 102, 57, 134], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assets` (0xcf35bdd0) function"]
        pub fn assets(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([207, 53, 189, 208], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fallbackPeriodSeconds` (0xab91dcb9) function"]
        pub fn fallback_period_seconds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([171, 145, 220, 185], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetOracle` (0x0db015f4) function"]
        pub fn get_asset_oracle(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 176, 21, 244], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetsLength` (0xa89d490c) function"]
        pub fn get_assets_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([168, 157, 73, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentPrice` (0x84cc315b) function"]
        pub fn get_current_price(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([132, 204, 49, 91], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isValidOption` (0xe2411261) function"]
        pub fn is_valid_option(
            &self,
            underlying_asset: ethers::core::types::Address,
            p1: u128,
            p2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([226, 65, 18, 97], (underlying_asset, p1, p2))
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
        #[doc = "Calls the contract's `priceRegistry` (0x535131d7) function"]
        pub fn price_registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([83, 81, 49, 215], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `searchRoundToSubmit` (0x0a821fea) function"]
        pub fn search_round_to_submit(
            &self,
            asset: ethers::core::types::Address,
            expiry_timestamp: u128,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([10, 130, 31, 234], (asset, expiry_timestamp))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpiryPriceInRegistry` (0xb9bd8796) function"]
        pub fn set_expiry_price_in_registry(
            &self,
            asset: ethers::core::types::Address,
            expiry_timestamp: u128,
            p2: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 189, 135, 150], (asset, expiry_timestamp, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpiryPriceInRegistryByRound` (0x4ac53653) function"]
        pub fn set_expiry_price_in_registry_by_round(
            &self,
            asset: ethers::core::types::Address,
            expiry_timestamp: u128,
            round_id_after_expiry: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [74, 197, 54, 83],
                    (asset, expiry_timestamp, round_id_after_expiry),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpiryPriceInRegistryFallback` (0x6ee3827b) function"]
        pub fn set_expiry_price_in_registry_fallback(
            &self,
            asset: ethers::core::types::Address,
            expiry_timestamp: u128,
            price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 227, 130, 123], (asset, expiry_timestamp, price))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `strikeAssetDecimals` (0xc1325661) function"]
        pub fn strike_asset_decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([193, 50, 86, 97], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OracleAdded` event"]
        pub fn oracle_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OracleAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PriceRegistrySubmission` event"]
        pub fn price_registry_submission_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PriceRegistrySubmissionFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ChainlinkOracleManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ChainlinkOracleManager<M>
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
    #[ethevent(name = "OracleAdded", abi = "OracleAdded(address,address)")]
    pub struct OracleAddedFilter {
        pub asset: ethers::core::types::Address,
        pub oracle: ethers::core::types::Address,
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
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
        name = "PriceRegistrySubmission",
        abi = "PriceRegistrySubmission(address,uint88,uint256,uint256,address,bool)"
    )]
    pub struct PriceRegistrySubmissionFilter {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
        pub price: ethers::core::types::U256,
        pub expiry_round_id: ethers::core::types::U256,
        pub price_submitter: ethers::core::types::Address,
        pub is_fallback: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ChainlinkOracleManagerEvents {
        OracleAddedFilter(OracleAddedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PriceRegistrySubmissionFilter(PriceRegistrySubmissionFilter),
    }
    impl ethers::contract::EthLogDecode for ChainlinkOracleManagerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = OracleAddedFilter::decode_log(log) {
                return Ok(ChainlinkOracleManagerEvents::OracleAddedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ChainlinkOracleManagerEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PriceRegistrySubmissionFilter::decode_log(log) {
                return Ok(ChainlinkOracleManagerEvents::PriceRegistrySubmissionFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ChainlinkOracleManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ChainlinkOracleManagerEvents::OracleAddedFilter(element) => element.fmt(f),
                ChainlinkOracleManagerEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                ChainlinkOracleManagerEvents::PriceRegistrySubmissionFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addAssetOracle`function with signature `addAssetOracle(address,address)` and selector `[140, 180, 186, 35]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addAssetOracle", abi = "addAssetOracle(address,address)")]
    pub struct AddAssetOracleCall {
        pub asset: ethers::core::types::Address,
        pub oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `assetOracles`function with signature `assetOracles(address)` and selector `[43, 102, 57, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assetOracles", abi = "assetOracles(address)")]
    pub struct AssetOraclesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `assets`function with signature `assets(uint256)` and selector `[207, 53, 189, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assets", abi = "assets(uint256)")]
    pub struct AssetsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `fallbackPeriodSeconds`function with signature `fallbackPeriodSeconds()` and selector `[171, 145, 220, 185]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fallbackPeriodSeconds", abi = "fallbackPeriodSeconds()")]
    pub struct FallbackPeriodSecondsCall;
    #[doc = "Container type for all input parameters for the `getAssetOracle`function with signature `getAssetOracle(address)` and selector `[13, 176, 21, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetOracle", abi = "getAssetOracle(address)")]
    pub struct GetAssetOracleCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAssetsLength`function with signature `getAssetsLength()` and selector `[168, 157, 73, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetsLength", abi = "getAssetsLength()")]
    pub struct GetAssetsLengthCall;
    #[doc = "Container type for all input parameters for the `getCurrentPrice`function with signature `getCurrentPrice(address)` and selector `[132, 204, 49, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCurrentPrice", abi = "getCurrentPrice(address)")]
    pub struct GetCurrentPriceCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isValidOption`function with signature `isValidOption(address,uint88,uint256)` and selector `[226, 65, 18, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isValidOption", abi = "isValidOption(address,uint88,uint256)")]
    pub struct IsValidOptionCall {
        pub underlying_asset: ethers::core::types::Address,
        pub p1: u128,
        pub p2: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `renounceOwnership`function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `searchRoundToSubmit`function with signature `searchRoundToSubmit(address,uint88)` and selector `[10, 130, 31, 234]`"]
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
        name = "searchRoundToSubmit",
        abi = "searchRoundToSubmit(address,uint88)"
    )]
    pub struct SearchRoundToSubmitCall {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
    }
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistry`function with signature `setExpiryPriceInRegistry(address,uint88,bytes)` and selector `[185, 189, 135, 150]`"]
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
        name = "setExpiryPriceInRegistry",
        abi = "setExpiryPriceInRegistry(address,uint88,bytes)"
    )]
    pub struct SetExpiryPriceInRegistryCall {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
        pub p2: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistryByRound`function with signature `setExpiryPriceInRegistryByRound(address,uint88,uint256)` and selector `[74, 197, 54, 83]`"]
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
        name = "setExpiryPriceInRegistryByRound",
        abi = "setExpiryPriceInRegistryByRound(address,uint88,uint256)"
    )]
    pub struct SetExpiryPriceInRegistryByRoundCall {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
        pub round_id_after_expiry: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistryFallback`function with signature `setExpiryPriceInRegistryFallback(address,uint88,uint256)` and selector `[110, 227, 130, 123]`"]
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
        name = "setExpiryPriceInRegistryFallback",
        abi = "setExpiryPriceInRegistryFallback(address,uint88,uint256)"
    )]
    pub struct SetExpiryPriceInRegistryFallbackCall {
        pub asset: ethers::core::types::Address,
        pub expiry_timestamp: u128,
        pub price: ethers::core::types::U256,
    }
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
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ChainlinkOracleManagerCalls {
        AddAssetOracle(AddAssetOracleCall),
        AssetOracles(AssetOraclesCall),
        Assets(AssetsCall),
        FallbackPeriodSeconds(FallbackPeriodSecondsCall),
        GetAssetOracle(GetAssetOracleCall),
        GetAssetsLength(GetAssetsLengthCall),
        GetCurrentPrice(GetCurrentPriceCall),
        IsValidOption(IsValidOptionCall),
        Owner(OwnerCall),
        PriceRegistry(PriceRegistryCall),
        RenounceOwnership(RenounceOwnershipCall),
        SearchRoundToSubmit(SearchRoundToSubmitCall),
        SetExpiryPriceInRegistry(SetExpiryPriceInRegistryCall),
        SetExpiryPriceInRegistryByRound(SetExpiryPriceInRegistryByRoundCall),
        SetExpiryPriceInRegistryFallback(SetExpiryPriceInRegistryFallbackCall),
        StrikeAssetDecimals(StrikeAssetDecimalsCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for ChainlinkOracleManagerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddAssetOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::AddAssetOracle(decoded));
            }
            if let Ok(decoded) =
                <AssetOraclesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::AssetOracles(decoded));
            }
            if let Ok(decoded) = <AssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::Assets(decoded));
            }
            if let Ok(decoded) =
                <FallbackPeriodSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::FallbackPeriodSeconds(decoded));
            }
            if let Ok(decoded) =
                <GetAssetOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::GetAssetOracle(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::GetAssetsLength(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::GetCurrentPrice(decoded));
            }
            if let Ok(decoded) =
                <IsValidOptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::IsValidOption(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PriceRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::PriceRegistry(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SearchRoundToSubmitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::SearchRoundToSubmit(decoded));
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ChainlinkOracleManagerCalls::SetExpiryPriceInRegistry(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryByRoundCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ChainlinkOracleManagerCalls::SetExpiryPriceInRegistryByRound(decoded));
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryFallbackCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ChainlinkOracleManagerCalls::SetExpiryPriceInRegistryFallback(decoded));
            }
            if let Ok(decoded) =
                <StrikeAssetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::StrikeAssetDecimals(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkOracleManagerCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ChainlinkOracleManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ChainlinkOracleManagerCalls::AddAssetOracle(element) => element.encode(),
                ChainlinkOracleManagerCalls::AssetOracles(element) => element.encode(),
                ChainlinkOracleManagerCalls::Assets(element) => element.encode(),
                ChainlinkOracleManagerCalls::FallbackPeriodSeconds(element) => element.encode(),
                ChainlinkOracleManagerCalls::GetAssetOracle(element) => element.encode(),
                ChainlinkOracleManagerCalls::GetAssetsLength(element) => element.encode(),
                ChainlinkOracleManagerCalls::GetCurrentPrice(element) => element.encode(),
                ChainlinkOracleManagerCalls::IsValidOption(element) => element.encode(),
                ChainlinkOracleManagerCalls::Owner(element) => element.encode(),
                ChainlinkOracleManagerCalls::PriceRegistry(element) => element.encode(),
                ChainlinkOracleManagerCalls::RenounceOwnership(element) => element.encode(),
                ChainlinkOracleManagerCalls::SearchRoundToSubmit(element) => element.encode(),
                ChainlinkOracleManagerCalls::SetExpiryPriceInRegistry(element) => element.encode(),
                ChainlinkOracleManagerCalls::SetExpiryPriceInRegistryByRound(element) => {
                    element.encode()
                }
                ChainlinkOracleManagerCalls::SetExpiryPriceInRegistryFallback(element) => {
                    element.encode()
                }
                ChainlinkOracleManagerCalls::StrikeAssetDecimals(element) => element.encode(),
                ChainlinkOracleManagerCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ChainlinkOracleManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ChainlinkOracleManagerCalls::AddAssetOracle(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::AssetOracles(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::Assets(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::FallbackPeriodSeconds(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::GetAssetOracle(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::GetAssetsLength(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::GetCurrentPrice(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::IsValidOption(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::Owner(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::PriceRegistry(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::RenounceOwnership(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::SearchRoundToSubmit(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::SetExpiryPriceInRegistry(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::SetExpiryPriceInRegistryByRound(element) => {
                    element.fmt(f)
                }
                ChainlinkOracleManagerCalls::SetExpiryPriceInRegistryFallback(element) => {
                    element.fmt(f)
                }
                ChainlinkOracleManagerCalls::StrikeAssetDecimals(element) => element.fmt(f),
                ChainlinkOracleManagerCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddAssetOracleCall> for ChainlinkOracleManagerCalls {
        fn from(var: AddAssetOracleCall) -> Self {
            ChainlinkOracleManagerCalls::AddAssetOracle(var)
        }
    }
    impl ::std::convert::From<AssetOraclesCall> for ChainlinkOracleManagerCalls {
        fn from(var: AssetOraclesCall) -> Self {
            ChainlinkOracleManagerCalls::AssetOracles(var)
        }
    }
    impl ::std::convert::From<AssetsCall> for ChainlinkOracleManagerCalls {
        fn from(var: AssetsCall) -> Self {
            ChainlinkOracleManagerCalls::Assets(var)
        }
    }
    impl ::std::convert::From<FallbackPeriodSecondsCall> for ChainlinkOracleManagerCalls {
        fn from(var: FallbackPeriodSecondsCall) -> Self {
            ChainlinkOracleManagerCalls::FallbackPeriodSeconds(var)
        }
    }
    impl ::std::convert::From<GetAssetOracleCall> for ChainlinkOracleManagerCalls {
        fn from(var: GetAssetOracleCall) -> Self {
            ChainlinkOracleManagerCalls::GetAssetOracle(var)
        }
    }
    impl ::std::convert::From<GetAssetsLengthCall> for ChainlinkOracleManagerCalls {
        fn from(var: GetAssetsLengthCall) -> Self {
            ChainlinkOracleManagerCalls::GetAssetsLength(var)
        }
    }
    impl ::std::convert::From<GetCurrentPriceCall> for ChainlinkOracleManagerCalls {
        fn from(var: GetCurrentPriceCall) -> Self {
            ChainlinkOracleManagerCalls::GetCurrentPrice(var)
        }
    }
    impl ::std::convert::From<IsValidOptionCall> for ChainlinkOracleManagerCalls {
        fn from(var: IsValidOptionCall) -> Self {
            ChainlinkOracleManagerCalls::IsValidOption(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ChainlinkOracleManagerCalls {
        fn from(var: OwnerCall) -> Self {
            ChainlinkOracleManagerCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PriceRegistryCall> for ChainlinkOracleManagerCalls {
        fn from(var: PriceRegistryCall) -> Self {
            ChainlinkOracleManagerCalls::PriceRegistry(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ChainlinkOracleManagerCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ChainlinkOracleManagerCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SearchRoundToSubmitCall> for ChainlinkOracleManagerCalls {
        fn from(var: SearchRoundToSubmitCall) -> Self {
            ChainlinkOracleManagerCalls::SearchRoundToSubmit(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryCall> for ChainlinkOracleManagerCalls {
        fn from(var: SetExpiryPriceInRegistryCall) -> Self {
            ChainlinkOracleManagerCalls::SetExpiryPriceInRegistry(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryByRoundCall> for ChainlinkOracleManagerCalls {
        fn from(var: SetExpiryPriceInRegistryByRoundCall) -> Self {
            ChainlinkOracleManagerCalls::SetExpiryPriceInRegistryByRound(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryFallbackCall> for ChainlinkOracleManagerCalls {
        fn from(var: SetExpiryPriceInRegistryFallbackCall) -> Self {
            ChainlinkOracleManagerCalls::SetExpiryPriceInRegistryFallback(var)
        }
    }
    impl ::std::convert::From<StrikeAssetDecimalsCall> for ChainlinkOracleManagerCalls {
        fn from(var: StrikeAssetDecimalsCall) -> Self {
            ChainlinkOracleManagerCalls::StrikeAssetDecimals(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ChainlinkOracleManagerCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ChainlinkOracleManagerCalls::TransferOwnership(var)
        }
    }
}
