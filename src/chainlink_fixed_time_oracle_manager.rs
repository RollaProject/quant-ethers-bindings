pub use chainlinkfixedtimeoraclemanager_mod::*;
#[allow(clippy::too_many_arguments)]
mod chainlinkfixedtimeoraclemanager_mod {
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
    #[doc = "ChainlinkFixedTimeOracleManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CHAINLINKFIXEDTIMEORACLEMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_priceRegistry\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_strikeAssetDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_fallbackPeriodSeconds\",\"type\":\"uint88\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fixedTime\",\"type\":\"uint24\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isValidTime\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FixedTimeUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint88\",\"name\":\"expiryTimestamp\",\"type\":\"uint88\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expiryRoundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"priceSubmitter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isFallback\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceRegistrySubmission\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetOracles\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"chainlinkFixedTimeUpdates\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fallbackPeriodSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidOption\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"searchRoundToSubmit\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistry\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_roundIdAfterExpiry\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryByRound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryFallback\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fixedTime\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isValidTime\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFixedTimeUpdate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"strikeAssetDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CHAINLINKFIXEDTIMEORACLEMANAGER_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x60e034620000cf57601f620025ec38819003918201601f19168301916001600160401b03831184841017620000d457808492606094604052833981010312620000cf578051906001600160a01b0382168203620000cf5760208101519060ff82168203620000cf5760400151906001600160581b0382168203620000cf576200008892620000ea565b6040516124319081620001bb82396080518181816103f80152818161050c0152611c92015260a0518181816104600152610c01015260c0518181816108180152610e030152f35b600080fd5b634e487b7160e01b600052604160045260246000fd5b60008054336001600160a01b0319821681178355604051949594926001600160a01b03928316907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09085a38216156200015257506080526001600160581b031660a05260c052565b62461bcd60e51b815260206004820152603560248201527f50726f76696465724f7261636c654d616e616765723a20696e76616c6964207060448201527f72696365207265676973747279206164647265737300000000000000000000006064820152608490fdfe60806040526004361015610013575b600080fd5b60003560e01c80630a821fea1461018f5780630db015f4146101865780632b6639861461017d5780634ac5365314610174578063535131d71461016b5780636ee3827b14610162578063715018a61461015957806384cc315b1461015057806387e4e00c146101475780638cb4ba231461013e5780638da5cb5b14610135578063a89d490c1461012c578063ab91dcb914610123578063b9bd87961461011a578063c132566114610111578063c6973b9614610108578063cf35bdd0146100ff578063e2411261146100f65763f2fde38b146100ee57600080fd5b61000e611004565b5061000e610f86565b5061000e610f00565b5061000e610e27565b5061000e610dca565b5061000e610cf2565b5061000e610bca565b5061000e610b8d565b5061000e610b3a565b5061000e610934565b5061000e6108da565b5061000e610732565b5061000e61068a565b5061000e61041c565b5061000e6103ac565b5061000e610387565b5061000e6102b7565b5061000e610257565b5061000e6101f8565b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b602435906affffffffffffffffffffff8216820361000e57565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610243610235610198565b61023d6101de565b9061179f565b69ffffffffffffffffffff60405191168152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610299610294610198565b6122d8565b73ffffffffffffffffffffffffffffffffffffffff60405191168152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602073ffffffffffffffffffffffffffffffffffffffff80610307610198565b166000526001825260406000205416604051908152f35b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc606091011261000e5760043573ffffffffffffffffffffffffffffffffffffffff8116810361000e57906024356affffffffffffffffffffff8116810361000e579060443590565b503461000e5761039f6103993661031e565b91611b1c565b005b600091031261000e57565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461000e5761042b3661031e565b9091600473ffffffffffffffffffffffffffffffffffffffff9160009483610458879586541633146110f4565b61049a6104927f00000000000000000000000000000000000000000000000000000000000000006affffffffffffffffffffff85166111ab565b4210156111c3565b6040805173ffffffffffffffffffffffffffffffffffffffff851681526affffffffffffffffffffff8416602082015290810187905260006060820152336080820152600160a08201527f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e0799060c090a17f00000000000000000000000000000000000000000000000000000000000000001691602061055761053e61053e846122d8565b73ffffffffffffffffffffffffffffffffffffffff1690565b604051958680927f313ce5670000000000000000000000000000000000000000000000000000000082525afa93841561067d575b859461064d575b50823b15610649576040517feae65f0800000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90911660048201526affffffffffffffffffffff909116602482015260ff909216604483015260648201939093529182908183816084810103925af1801561063c575b610623575b50604051f35b8061063061063692610c54565b806103a1565b3861061d565b61064461128d565b610618565b8480fd5b61066f91945060203d8111610676575b6106678183610c91565b810190611274565b9238610592565b503d61065d565b61068561128d565b61058b565b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261072f5780547fffffffffffffffffffffffff000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff8216916107013384146110f4565b16825581604051917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b80fd5b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761084e61083e61081461077c61053e61053e610294610198565b604051907ffeaf968c00000000000000000000000000000000000000000000000000000000825260a082600481845afa9182156108bb575b60009261087d575b506004602060ff926107d0600086136112e6565b604051928380927f313ce5670000000000000000000000000000000000000000000000000000000082525afa908115610870575b600091610852575b501690611431565b60ff7f000000000000000000000000000000000000000000000000000000000000000016906114cf565b6040519081529081906020820190565b0390f35b61086a915060203d8111610676576106678183610c91565b3861080c565b61087861128d565b610804565b60ff91925060206108a660049260a03d81116108b4575b61089e8183610c91565b8101906112b1565b5050509050939250506107bc565b503d610894565b6108c361128d565b6107b4565b6004359062ffffff8216820361000e57565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5762ffffff6109166108c8565b166000526003602052602060ff604060002054166040519015158152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761096c610198565b6109746101bb565b73ffffffffffffffffffffffffffffffffffffffff610998816000541633146110f4565b811615610ab65781610a14610a0e61053e6109f47f828d2be040dede7698182e08dfa8bfbd663c879aee772509c4a2bd961d0ed43f9673ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b5473ffffffffffffffffffffffffffffffffffffffff1690565b15612185565b610a1d81612210565b610a8b82610a4b8373ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b9073ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b6040805173ffffffffffffffffffffffffffffffffffffffff928316815292909116602083015290a1005b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602d60248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520697360448201527f207a65726f2061646472657373000000000000000000000000000000000000006064820152fd5b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602073ffffffffffffffffffffffffffffffffffffffff60005416604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020600254604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b67ffffffffffffffff8111610c6857604052565b610c70610c24565b604052565b6020810190811067ffffffffffffffff821117610c6857604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610c6857604052565b604051906080820182811067ffffffffffffffff821117610c6857604052565b503461000e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610d2a610198565b610d326101de565b67ffffffffffffffff9160443583811161000e573660238201121561000e578060040135938411610dbd575b60405190610d9460207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8801160183610c91565b848252366024868301011161000e578460009261039f9660246020940184830137010152611159565b610dc5610c24565b610d5e565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602060405160ff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610e5f6108c8565b6024359081151580920361000e577f1a4d939ae35d2ef6fe079efd40bdae5980befb19ae0806f6ebee6490f1344b459162ffffff604092610eb973ffffffffffffffffffffffffffffffffffffffff6000541633146110f4565b1690816000526003602052826000207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541660ff831617905582519182526020820152a1005b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043560025481101561000e5773ffffffffffffffffffffffffffffffffffffffff60209160026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace015416604051908152f35b503461000e57610f953661031e565b509073ffffffffffffffffffffffffffffffffffffffff809116600052600160205260406000205416151580610fd3575b6020906040519015158152f35b50620151806affffffffffffffffffffff62ffffff921606166000526003602052602060ff60406000205416610fc6565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5761103c610198565b73ffffffffffffffffffffffffffffffffffffffff611060816000541633146110f4565b8116156110705761039f9061238c565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b156110fb57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b6111799169ffffffffffffffffffff611172828461179f565b1691611b1c565b565b507f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b811981116111b7570190565b6111bf61117b565b0190565b156111ca57565b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152605460248201527f436861696e6c696e6b4f7261636c654d616e616765723a205468652066616c6c60448201527f6261636b20707269636520706572696f6420686173206e6f742070617373656460648201527f2073696e6365207468652074696d657374616d700000000000000000000000006084820152fd5b9081602091031261000e575160ff8116810361000e5790565b506040513d6000823e3d90fd5b519069ffffffffffffffffffff8216820361000e57565b908160a091031261000e576112c58161129a565b916020820151916040810151916112e360806060840151930161129a565b90565b156112ed57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603160248201527f436861696e6c696e6b4f7261636c654d616e616765723a204e6f20707269636960448201527f6e67206461746120617661696c61626c650000000000000000000000000000006064820152fd5b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe590601b81106111b7570190565b604d81116113ae575b600a0a90565b6113b661117b565b6113a8565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff048211811515166113ec570290565b6113f461117b565b0290565b8115611402570490565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b90600060405161144081610c75565b52600060405161144f81610c75565b52601b810361147457506114629061153b565b6040519061146f82610c75565b815290565b601b8111156114a0576114629161149561149061149b93611371565b61139f565b906113f8565b61153b565b611462916114bc8261149b93601b106114c2575b601b0361139f565b906113bb565b6114ca61117b565b6114b4565b90601b81036114e357506112e390516115c4565b601b81111561150c57906114bc6115066114ff6112e394611371565b92516115c4565b9161139f565b6114956115256112e39383601b1061152e575b516115c4565b91601b0361139f565b61153661117b565b61151f565b7f80000000000000000000000000000000000000000000000000000000000000008110156115665790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f5175616e744d6174683a206f7574206f6620696e742072616e676500000000006044820152fd5b600081126115cf5790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5175616e744d6174683a206e6567617469766520696e740000000000000000006044820152fd5b9081602091031261000e575190565b1561164357565b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152605460248201527f436861696e6c696e6b4f7261636c654d616e616765723a20546865206c61746560448201527f737420726f756e642074696d657374616d70206973206e6f742061667465722060648201527f746865206578706972792074696d657374616d700000000000000000000000006084820152fd5b156116f457565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603d60248201527f436861696e6c696e6b4f7261636c654d616e616765723a204e6f7420656e6f7560448201527f676820726f756e647320746f2066696e6420726f756e642061667465720000006064820152fd5b69ffffffffffffffffffff9182169116818110611793570390565b61179b61117b565b0390565b916117af61053e61182f946122d8565b9073ffffffffffffffffffffffffffffffffffffffff82169269ffffffffffffffffffff60046040958651907f8205bf6a000000000000000000000000000000000000000000000000000000008252602098828a81868296865afa908115611975575b600091611958575b506affffffffffffffffffffff87161061163c565b8751928380927f668a0f020000000000000000000000000000000000000000000000000000000082525afa90811561194b575b60009161191e575b501660018061189661188782856118828282116116ed565b611778565b69ffffffffffffffffffff1690565b1461191457906118a7918386612000565b926118bc845169ffffffffffffffffffff1690565b9260016118966118876118db8a89015169ffffffffffffffffffff1690565b9761190960606118f78c84015169ffffffffffffffffffff1690565b92015169ffffffffffffffffffff1690565b989598979497611778565b5093505092505090565b61193e9150873d8911611944575b6119368183610c91565b81019061162d565b3861186a565b503d61192c565b61195361128d565b611862565b61196f9150843d8611611944576119368183610c91565b3861181a565b61197d61128d565b611812565b1561198957565b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152604a60248201527f436861696e6c696e6b4f7261636c654d616e616765723a2054686520726f756e60448201527f6420706f73746564206973206e6f74206166746572207468652065787069727960648201527f2074696d657374616d70000000000000000000000000000000000000000000006084820152fd5b67ffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9116600181106111b7570190565b15611a7257565b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152605a60248201527f436861696e6c696e6b4f7261636c654d616e616765723a20457870697279207260448201527f6f756e64207072696f7220746f20746865206f6e6520706f737465642069732060648201527f616674657220746865206578706972792074696d657374616d700000000000006084820152fd5b91611c4283926004611b3061053e866122d8565b73ffffffffffffffffffffffffffffffffffffffff7f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e07981831695869360405198877fb633620c0000000000000000000000000000000000000000000000000000000092838c52611c3d8c602081819f9b81808f859f611bb6918c91019190602083019252565b03915afa908115611e0b575b600091611dee575b50611be46affffffffffffffffffffff8516809211611982565b8c8a67ffffffffffffffff611bfa818916611a33565b604051998a521669ffff00000000000000008816178d890181815290989283918290819060200103915afa908115611de1575b600091611dc4575b501115611a6b565b611e18565b6040805173ffffffffffffffffffffffffffffffffffffffff9c909c168c526affffffffffffffffffffff881660208d01528b0182905260608b01523360808b0152600060a08b01529860c090a17f00000000000000000000000000000000000000000000000000000000000000001693604051928380927f313ce5670000000000000000000000000000000000000000000000000000000082525afa938415611db7575b600094611d98575b5050813b1561000e576040517feae65f0800000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff90941660048501526affffffffffffffffffffff16602484015260ff909116604483015260648201929092529060009082908183816084810103925af18015611d8b575b611d7e5750565b8061063061117992610c54565b611d9361128d565b611d77565b611daf929450803d10610676576106678183610c91565b913880611cef565b611dbf61128d565b611ce7565b611ddb91508b3d8d11611944576119368183610c91565b38611c35565b611de961128d565b611c2d565b611e0591508a3d8c11611944576119368183610c91565b38611bca565b611e1361128d565b611bc2565b6040517fb633620c0000000000000000000000000000000000000000000000000000000081526004810185905293949373ffffffffffffffffffffffffffffffffffffffff91909116929160009190602082602481885afa918215611faf575b8392611f82575b506affffffffffffffffffffff1603611f1b5750506040517f9a6fc8f500000000000000000000000000000000000000000000000000000000815269ffffffffffffffffffff841660048201529060a090829060249082905afa908115611f0e575b600091611eed57509190565b611f05915060a03d81116108b45761089e8183610c91565b50505090509190565b611f1661128d565b611ee1565b6040517f9a6fc8f500000000000000000000000000000000000000000000000000000000815269ffffffffffffffffffff831660048201529194509160a090829060249082905afa918215611f75575b91611eed57509190565b611f7d61128d565b611f6b565b6affffffffffffffffffffff919250611fa89060203d8111611944576119368183610c91565b9190611e7f565b611fb761128d565b611e78565b604051906080820182811067ffffffffffffffff821117611ff3575b60405260006060838281528260208201528260408201520152565b611ffb610c24565b611fd8565b91929061200b611fbc565b5067ffffffffffffffff8083169085169261203261188761202c84876111ab565b60011c90565b9469ffff0000000000000000821669ffffffffffffffffffff87161793602060405180937fb633620c0000000000000000000000000000000000000000000000000000000082528173ffffffffffffffffffffffffffffffffffffffff816120a28b600483019190602083019252565b0392165afa918215612178575b60009261214b575b506affffffffffffffffffffff16101561212f5750506112e3929161210961211c926120f66120e4610cd2565b69ffffffffffffffffffff9098168852565b69ffffffffffffffffffff166020870152565b69ffffffffffffffffffff166040850152565b69ffffffffffffffffffff166060830152565b61211c9295506112e394919350612109906120f66120e4610cd2565b6affffffffffffffffffffff9192506121719060203d8111611944576119368183610c91565b91906120b7565b61218061128d565b6120af565b1561218c57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603360248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520616c60448201527f72656164792073657420666f72206173736574000000000000000000000000006064820152fd5b600254680100000000000000008110156122cb575b600181018060025581101561229c5773ffffffffffffffffffffffffffffffffffffffff9060026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace0191167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b6122d3610c24565b612225565b73ffffffffffffffffffffffffffffffffffffffff80911660005260016020526040600020541680156123085790565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603a60248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520646f60448201527f65736e277420657869737420666f7220746861742061737365740000000000006064820152fd5b6000549073ffffffffffffffffffffffffffffffffffffffff80911691827fffffffffffffffffffffffff0000000000000000000000000000000000000000821617600055167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e06000604051a356fea26469706673582212204e790baff0a32b3f7fcff5e59996986475d4d81ebdadd63b5b339527299cbf0d64736f6c634300080e0033" . parse () . expect ("invalid bytecode")
    });
    pub struct ChainlinkFixedTimeOracleManager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ChainlinkFixedTimeOracleManager<M> {
        fn clone(&self) -> Self {
            ChainlinkFixedTimeOracleManager(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ChainlinkFixedTimeOracleManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ChainlinkFixedTimeOracleManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ChainlinkFixedTimeOracleManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ChainlinkFixedTimeOracleManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                CHAINLINKFIXEDTIMEORACLEMANAGER_ABI.clone(),
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
                CHAINLINKFIXEDTIMEORACLEMANAGER_ABI.clone(),
                CHAINLINKFIXEDTIMEORACLEMANAGER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `chainlinkFixedTimeUpdates` (0x87e4e00c) function"]
        pub fn chainlink_fixed_time_updates(
            &self,
            p0: u32,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([135, 228, 224, 12], p0)
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
            expiry_time: u128,
            p2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([226, 65, 18, 97], (underlying_asset, expiry_time, p2))
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
        #[doc = "Calls the contract's `setFixedTimeUpdate` (0xc6973b96) function"]
        pub fn set_fixed_time_update(
            &self,
            fixed_time: u32,
            is_valid_time: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 151, 59, 150], (fixed_time, is_valid_time))
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
        #[doc = "Gets the contract's `FixedTimeUpdate` event"]
        pub fn fixed_time_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FixedTimeUpdateFilter> {
            self.0.event()
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
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, ChainlinkFixedTimeOracleManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ChainlinkFixedTimeOracleManager<M>
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
    #[ethevent(name = "FixedTimeUpdate", abi = "FixedTimeUpdate(uint24,bool)")]
    pub struct FixedTimeUpdateFilter {
        pub fixed_time: u32,
        pub is_valid_time: bool,
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
    pub enum ChainlinkFixedTimeOracleManagerEvents {
        FixedTimeUpdateFilter(FixedTimeUpdateFilter),
        OracleAddedFilter(OracleAddedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PriceRegistrySubmissionFilter(PriceRegistrySubmissionFilter),
    }
    impl ethers::contract::EthLogDecode for ChainlinkFixedTimeOracleManagerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = FixedTimeUpdateFilter::decode_log(log) {
                return Ok(ChainlinkFixedTimeOracleManagerEvents::FixedTimeUpdateFilter(decoded));
            }
            if let Ok(decoded) = OracleAddedFilter::decode_log(log) {
                return Ok(ChainlinkFixedTimeOracleManagerEvents::OracleAddedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    ChainlinkFixedTimeOracleManagerEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = PriceRegistrySubmissionFilter::decode_log(log) {
                return Ok(
                    ChainlinkFixedTimeOracleManagerEvents::PriceRegistrySubmissionFilter(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ChainlinkFixedTimeOracleManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ChainlinkFixedTimeOracleManagerEvents::FixedTimeUpdateFilter(element) => {
                    element.fmt(f)
                }
                ChainlinkFixedTimeOracleManagerEvents::OracleAddedFilter(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerEvents::OwnershipTransferredFilter(element) => {
                    element.fmt(f)
                }
                ChainlinkFixedTimeOracleManagerEvents::PriceRegistrySubmissionFilter(element) => {
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
    #[doc = "Container type for all input parameters for the `chainlinkFixedTimeUpdates`function with signature `chainlinkFixedTimeUpdates(uint24)` and selector `[135, 228, 224, 12]`"]
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
        name = "chainlinkFixedTimeUpdates",
        abi = "chainlinkFixedTimeUpdates(uint24)"
    )]
    pub struct ChainlinkFixedTimeUpdatesCall(pub u32);
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
        pub expiry_time: u128,
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
    #[doc = "Container type for all input parameters for the `setFixedTimeUpdate`function with signature `setFixedTimeUpdate(uint24,bool)` and selector `[198, 151, 59, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFixedTimeUpdate", abi = "setFixedTimeUpdate(uint24,bool)")]
    pub struct SetFixedTimeUpdateCall {
        pub fixed_time: u32,
        pub is_valid_time: bool,
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
    pub enum ChainlinkFixedTimeOracleManagerCalls {
        AddAssetOracle(AddAssetOracleCall),
        AssetOracles(AssetOraclesCall),
        Assets(AssetsCall),
        ChainlinkFixedTimeUpdates(ChainlinkFixedTimeUpdatesCall),
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
        SetFixedTimeUpdate(SetFixedTimeUpdateCall),
        StrikeAssetDecimals(StrikeAssetDecimalsCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for ChainlinkFixedTimeOracleManagerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddAssetOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::AddAssetOracle(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AssetOraclesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::AssetOracles(decoded));
            }
            if let Ok(decoded) = <AssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::Assets(decoded));
            }
            if let Ok(decoded) =
                <ChainlinkFixedTimeUpdatesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::ChainlinkFixedTimeUpdates(decoded));
            }
            if let Ok(decoded) =
                <FallbackPeriodSecondsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::FallbackPeriodSeconds(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetAssetOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::GetAssetOracle(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetAssetsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::GetAssetsLength(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetCurrentPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::GetCurrentPrice(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IsValidOptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::IsValidOption(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PriceRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::PriceRegistry(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::RenounceOwnership(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SearchRoundToSubmitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::SearchRoundToSubmit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistry(decoded));
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryByRoundCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryByRound(decoded),
                );
            }
            if let Ok(decoded) =
                <SetExpiryPriceInRegistryFallbackCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryFallback(decoded),
                );
            }
            if let Ok(decoded) =
                <SetFixedTimeUpdateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::SetFixedTimeUpdate(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <StrikeAssetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::StrikeAssetDecimals(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ChainlinkFixedTimeOracleManagerCalls::TransferOwnership(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ChainlinkFixedTimeOracleManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ChainlinkFixedTimeOracleManagerCalls::AddAssetOracle(element) => element.encode(),
                ChainlinkFixedTimeOracleManagerCalls::AssetOracles(element) => element.encode(),
                ChainlinkFixedTimeOracleManagerCalls::Assets(element) => element.encode(),
                ChainlinkFixedTimeOracleManagerCalls::ChainlinkFixedTimeUpdates(element) => {
                    element.encode()
                }
                ChainlinkFixedTimeOracleManagerCalls::FallbackPeriodSeconds(element) => {
                    element.encode()
                }
                ChainlinkFixedTimeOracleManagerCalls::GetAssetOracle(element) => element.encode(),
                ChainlinkFixedTimeOracleManagerCalls::GetAssetsLength(element) => element.encode(),
                ChainlinkFixedTimeOracleManagerCalls::GetCurrentPrice(element) => element.encode(),
                ChainlinkFixedTimeOracleManagerCalls::IsValidOption(element) => element.encode(),
                ChainlinkFixedTimeOracleManagerCalls::Owner(element) => element.encode(),
                ChainlinkFixedTimeOracleManagerCalls::PriceRegistry(element) => element.encode(),
                ChainlinkFixedTimeOracleManagerCalls::RenounceOwnership(element) => {
                    element.encode()
                }
                ChainlinkFixedTimeOracleManagerCalls::SearchRoundToSubmit(element) => {
                    element.encode()
                }
                ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistry(element) => {
                    element.encode()
                }
                ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryByRound(element) => {
                    element.encode()
                }
                ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryFallback(element) => {
                    element.encode()
                }
                ChainlinkFixedTimeOracleManagerCalls::SetFixedTimeUpdate(element) => {
                    element.encode()
                }
                ChainlinkFixedTimeOracleManagerCalls::StrikeAssetDecimals(element) => {
                    element.encode()
                }
                ChainlinkFixedTimeOracleManagerCalls::TransferOwnership(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for ChainlinkFixedTimeOracleManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ChainlinkFixedTimeOracleManagerCalls::AddAssetOracle(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::AssetOracles(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::Assets(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::ChainlinkFixedTimeUpdates(element) => {
                    element.fmt(f)
                }
                ChainlinkFixedTimeOracleManagerCalls::FallbackPeriodSeconds(element) => {
                    element.fmt(f)
                }
                ChainlinkFixedTimeOracleManagerCalls::GetAssetOracle(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::GetAssetsLength(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::GetCurrentPrice(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::IsValidOption(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::Owner(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::PriceRegistry(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::RenounceOwnership(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::SearchRoundToSubmit(element) => {
                    element.fmt(f)
                }
                ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistry(element) => {
                    element.fmt(f)
                }
                ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryByRound(element) => {
                    element.fmt(f)
                }
                ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryFallback(element) => {
                    element.fmt(f)
                }
                ChainlinkFixedTimeOracleManagerCalls::SetFixedTimeUpdate(element) => element.fmt(f),
                ChainlinkFixedTimeOracleManagerCalls::StrikeAssetDecimals(element) => {
                    element.fmt(f)
                }
                ChainlinkFixedTimeOracleManagerCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddAssetOracleCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: AddAssetOracleCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::AddAssetOracle(var)
        }
    }
    impl ::std::convert::From<AssetOraclesCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: AssetOraclesCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::AssetOracles(var)
        }
    }
    impl ::std::convert::From<AssetsCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: AssetsCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::Assets(var)
        }
    }
    impl ::std::convert::From<ChainlinkFixedTimeUpdatesCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: ChainlinkFixedTimeUpdatesCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::ChainlinkFixedTimeUpdates(var)
        }
    }
    impl ::std::convert::From<FallbackPeriodSecondsCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: FallbackPeriodSecondsCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::FallbackPeriodSeconds(var)
        }
    }
    impl ::std::convert::From<GetAssetOracleCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: GetAssetOracleCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::GetAssetOracle(var)
        }
    }
    impl ::std::convert::From<GetAssetsLengthCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: GetAssetsLengthCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::GetAssetsLength(var)
        }
    }
    impl ::std::convert::From<GetCurrentPriceCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: GetCurrentPriceCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::GetCurrentPrice(var)
        }
    }
    impl ::std::convert::From<IsValidOptionCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: IsValidOptionCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::IsValidOption(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: OwnerCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PriceRegistryCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: PriceRegistryCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::PriceRegistry(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SearchRoundToSubmitCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: SearchRoundToSubmitCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::SearchRoundToSubmit(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: SetExpiryPriceInRegistryCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistry(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryByRoundCall>
        for ChainlinkFixedTimeOracleManagerCalls
    {
        fn from(var: SetExpiryPriceInRegistryByRoundCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryByRound(var)
        }
    }
    impl ::std::convert::From<SetExpiryPriceInRegistryFallbackCall>
        for ChainlinkFixedTimeOracleManagerCalls
    {
        fn from(var: SetExpiryPriceInRegistryFallbackCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::SetExpiryPriceInRegistryFallback(var)
        }
    }
    impl ::std::convert::From<SetFixedTimeUpdateCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: SetFixedTimeUpdateCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::SetFixedTimeUpdate(var)
        }
    }
    impl ::std::convert::From<StrikeAssetDecimalsCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: StrikeAssetDecimalsCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::StrikeAssetDecimals(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ChainlinkFixedTimeOracleManagerCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ChainlinkFixedTimeOracleManagerCalls::TransferOwnership(var)
        }
    }
}
