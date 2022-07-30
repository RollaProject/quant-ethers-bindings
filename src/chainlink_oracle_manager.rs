pub use chainlink_oracle_manager::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod chainlink_oracle_manager {
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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_priceRegistry\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_strikeAssetDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_fallbackPeriodSeconds\",\"type\":\"uint88\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint88\",\"name\":\"expiryTimestamp\",\"type\":\"uint88\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expiryRoundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"priceSubmitter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isFallback\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceRegistrySubmission\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetOracles\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fallbackPeriodSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidOption\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"searchRoundToSubmit\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistry\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_roundIdAfterExpiry\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryByRound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryFallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"strikeAssetDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CHAINLINKORACLEMANAGER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60e03461017857601f6200201238819003918201601f19168301916001600160401b0383118484101761017d578084926060946040528339810103126101785780516001600160a01b0380821691908282036101785760208401519360ff8516850361017857604001516001600160581b0381169390849003610178576000543360018060a01b0319821617600055604051923391167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0600085a315610110575060805260a05260c052604051611e7e9081620001948239608051818181610d0901528181610f380152611874015260a0518181816104890152610c79015260c05181818161031d01526108fb0152f35b62461bcd60e51b815260206004820152603560248201527f50726f76696465724f7261636c654d616e616765723a20696e76616c6964207060448201527f72696365207265676973747279206164647265737300000000000000000000006064820152608490fd5b600080fd5b634e487b7160e01b600052604160045260246000fdfe6040608081526004908136101561001557600080fd5b600091823560e01c80630a821fea146110365780630db015f414610fda5780632b66398614610f755780634ac5365314610f5c578063535131d714610eed5780636ee3827b14610c4a578063715018a614610baa57806384cc315b146107d95780638cb4ba231461053a5780638da5cb5b146104e9578063a89d490c146104ac578063ab91dcb914610453578063b9bd879614610341578063c1325661146102e5578063cf35bdd01461025d578063e2411261146102185763f2fde38b146100dc57600080fd5b346102145760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102145761011361108d565b61011b611df2565b73ffffffffffffffffffffffffffffffffffffffff809116918215610191575060005492827fffffffffffffffffffffffff00000000000000000000000000000000000000008516176000555192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0600084a3f35b60849060208551917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b8280fd5b505034610259576020918161022c366110ca565b50509173ffffffffffffffffffffffffffffffffffffffff80931681526001855220541615159051908152f35b5080fd5b50346102145760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102145735916002548310156102e2575073ffffffffffffffffffffffffffffffffffffffff60209260026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace0154169051908152f35b80fd5b50503461025957817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610259576020905160ff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50346102145760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102145761037961108d565b6103816110b0565b60443567ffffffffffffffff9384821161044f573660238301121561044f5781810135948511610423575085908551946103e360207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601876111ae565b80865236602482840101116102145761042095816024602094018483013701015269ffffffffffffffffffff61041982846113f9565b16916116e7565b51f35b8660416024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b8680fd5b50503461025957817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261025957602090517f00000000000000000000000000000000000000000000000000000000000000008152f35b50503461025957817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610259576020906002549051908152f35b50503461025957817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102595773ffffffffffffffffffffffffffffffffffffffff60209254169051908152f35b503461021457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102145761057161108d565b906024359073ffffffffffffffffffffffffffffffffffffffff928383168093036107d45761059e611df2565b821561075157831692838652600160205284862054166106ce5760025490680100000000000000008210156106a257600182018060025582101561067457509183917f828d2be040dede7698182e08dfa8bfbd663c879aee772509c4a2bd961d0ed43f9360026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace017fffffffffffffffffffffffff00000000000000000000000000000000000000009083828254161790558287526001602052818488209182541617905582519182526020820152a151f35b6032907f4e487b71000000000000000000000000000000000000000000000000000000006000525260246000fd5b8560416024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b60849060208551917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603360248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520616c60448201527f72656164792073657420666f72206173736574000000000000000000000000006064820152fd5b60848260208751917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602d60248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520697360448201527f207a65726f2061646472657373000000000000000000000000000000000000006064820152fd5b600080fd5b5091346102e257602092837ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102595773ffffffffffffffffffffffffffffffffffffffff61083161082c61108d565b611d3e565b16918351927ffeaf968c00000000000000000000000000000000000000000000000000000000845260a0848481845afa938415610ae9578294610b75575b5081841315610af3578583918651928380927f313ce5670000000000000000000000000000000000000000000000000000000082525afa908115610ae9579060ff918391610abc575b5016928185516108c781611176565b528185516108d481611176565b52601b93808503610a1d57506108e990611307565b8451906108f582611176565b8152915b7f000000000000000000000000000000000000000000000000000000000000000060ff169380850361093b57505050610933915051611390565b905b51908152f35b808511156109bc578410610990575050906109847fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe561097d61098a9451611390565b920161128f565b9061129d565b90610935565b9060116024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b92908484959395106109f15750916109e66109ec926109df61098a969551611390565b920361128f565b906112ce565b6111ef565b8460116024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b84811115610aa357848110610a7757610a63916109e67fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe5610a5e930161128f565b611307565b845190610a6f82611176565b8152916108f9565b6024836011867f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b808510610a7757610a6391610984610a5e92870361128f565b610adc9150873d8911610ae2575b610ad481836111ae565b81019061122a565b386108b8565b503d610aca565b85513d84823e3d90fd5b608483878751917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603160248201527f436861696e6c696e6b4f7261636c654d616e616765723a204e6f20707269636960448201527f6e67206461746120617661696c61626c650000000000000000000000000000006064820152fd5b610b9791945060a03d8111610ba3575b610b8f81836111ae565b81019061125a565b5050509050923861086f565b503d610b85565b50503461025957817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261025957610be2611df2565b600073ffffffffffffffffffffffffffffffffffffffff8154927fffffffffffffffffffffffff0000000000000000000000000000000000000000841683555192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b5082903461025957610c5b366110ca565b9091610c65611df2565b6affffffffffffffffffffff831691610c9e7f0000000000000000000000000000000000000000000000000000000000000000846111ef565b4210610e445784907f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e07960c0895173ffffffffffffffffffffffffffffffffffffffff9687871682526020820152838b820152896060820152336080820152600160a0820152a16020847f00000000000000000000000000000000000000000000000000000000000000001694610d3385611d3e565b168951978880927f313ce5670000000000000000000000000000000000000000000000000000000082525afa958615610e3a578796610e1a575b50833b1561044f578694939291610df086928a51988997889687957feae65f0800000000000000000000000000000000000000000000000000000000875286019094939260ff906affffffffffffffffffffff60609473ffffffffffffffffffffffffffffffffffffffff60808601991685521660208401521660408201520152565b03925af18015610e1057610e0357509051f35b610e0c90611133565b9051f35b83513d84823e3d90fd5b610e3391965060203d8111610ae257610ad481836111ae565b9488610d6d565b88513d89823e3d90fd5b60a48560208951917f08c379a0000000000000000000000000000000000000000000000000000000008352820152605460248201527f436861696e6c696e6b4f7261636c654d616e616765723a205468652066616c6c60448201527f6261636b20707269636520706572696f6420686173206e6f742070617373656460648201527f2073696e6365207468652074696d657374616d700000000000000000000000006084820152fd5b50503461025957817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610259576020905173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b50503461025957610420610f6f366110ca565b916116e7565b5050346102595760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610259576020918173ffffffffffffffffffffffffffffffffffffffff9182610fc961108d565b168152600185522054169051908152f35b5050346102595760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102595760209073ffffffffffffffffffffffffffffffffffffffff61102e61082c61108d565b915191168152f35b50503461025957807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102595760209069ffffffffffffffffffff61102e61107f61108d565b6110876110b0565b906113f9565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036107d457565b602435906affffffffffffffffffffff821682036107d457565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc60609101126107d45760043573ffffffffffffffffffffffffffffffffffffffff811681036107d457906024356affffffffffffffffffffff811681036107d4579060443590565b67ffffffffffffffff811161114757604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6020810190811067ffffffffffffffff82111761114757604052565b6080810190811067ffffffffffffffff82111761114757604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761114757604052565b811981116111fb570190565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b908160209103126107d4575160ff811681036107d45790565b519069ffffffffffffffffffff821682036107d457565b908160a09103126107d45761126e81611243565b9160208201519160408101519161128c608060608401519301611243565b90565b604d81116111fb57600a0a90565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff048211811515166111fb570290565b81156112d8570490565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b7f80000000000000000000000000000000000000000000000000000000000000008110156113325790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f5175616e744d6174683a206f7574206f6620696e742072616e676500000000006044820152fd5b6000811261139b5790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5175616e744d6174683a206e6567617469766520696e740000000000000000006044820152fd5b61141773ffffffffffffffffffffffffffffffffffffffff91611d3e565b169160409081517f8205bf6a000000000000000000000000000000000000000000000000000000008152602093600491858184818a5afa908115611607576000916116ba575b506affffffffffffffffffffff82161015611612578351957f668a0f0200000000000000000000000000000000000000000000000000000000875285878481845afa968715611607576000976115d8575b5069ffffffffffffffffffff96871693600193848681811115611556578a80919894989793975b169116908181106115285703891660011461151c57906114f6918484611bdf565b80518782015187830151606090930151908a1692918a16918a16908a90819081166114d5565b50955050505092505090565b6011877f4e487b71000000000000000000000000000000000000000000000000000000006000525260246000fd5b6084838b8b51917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603d60248201527f436861696e6c696e6b4f7261636c654d616e616765723a204e6f7420656e6f7560448201527f676820726f756e647320746f2066696e6420726f756e642061667465720000006064820152fd5b90968682813d8311611600575b6115ef81836111ae565b810103126102e257505195386114ae565b503d6115e5565b85513d6000823e3d90fd5b60a482868651917f08c379a0000000000000000000000000000000000000000000000000000000008352820152605460248201527f436861696e6c696e6b4f7261636c654d616e616765723a20546865206c61746560448201527f737420726f756e642074696d657374616d70206973206e6f742061667465722060648201527f746865206578706972792074696d657374616d700000000000000000000000006084820152fd5b908682813d83116116e0575b6116d081836111ae565b810103126102e25750513861145d565b503d6116c6565b909173ffffffffffffffffffffffffffffffffffffffff928361170984611d3e565b166040938451937fb633620c000000000000000000000000000000000000000000000000000000009687865260049582878201526020986024938a8386818a5afa928315611ad157600093611bb0575b506affffffffffffffffffffff881680931115611b095767ffffffffffffffff8082169160018310611adc5769ffff00000000000000007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff91169201161790895190815281898201528a8186818a5afa908115611ad157908391600091611aa1575b50116119fa5760a0895180957f9a6fc8f5000000000000000000000000000000000000000000000000000000008252838b83015281895afa9384156119ef57918893918b936000966119a3575b507f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e0799160c0918c5191858a16835286830152878d8301526060820152336080820152600060a0820152a17f000000000000000000000000000000000000000000000000000000000000000016948851928380927f313ce5670000000000000000000000000000000000000000000000000000000082525afa97881561199857600098611979575b5050823b156107d45760009461195186928851998a97889687957feae65f0800000000000000000000000000000000000000000000000000000000875286019094939260ff906affffffffffffffffffffff60609473ffffffffffffffffffffffffffffffffffffffff60808601991685521660208401521660408201520152565b03925af190811561196f57506119645750565b61196d90611133565b565b513d6000823e3d90fd5b611990929850803d10610ae257610ad481836111ae565b9538806118cf565b87513d6000823e3d90fd5b60c0919650916119e17f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e0799360a03d8111610ba357610b8f81836111ae565b505050905096915091611828565b89513d6000823e3d90fd5b60a488605a868d8d51937f08c379a00000000000000000000000000000000000000000000000000000000085528401528201527f436861696e6c696e6b4f7261636c654d616e616765723a20457870697279207260448201527f6f756e64207072696f7220746f20746865206f6e6520706f737465642069732060648201527f616674657220746865206578706972792074696d657374616d700000000000006084820152fd5b91508b82813d8311611aca575b611ab881836111ae565b810103126102e25750829051386117db565b503d611aae565b8a513d6000823e3d90fd5b8660118c7f4e487b7100000000000000000000000000000000000000000000000000000000600052526000fd5b60a489604a878e8e51937f08c379a00000000000000000000000000000000000000000000000000000000085528401528201527f436861696e6c696e6b4f7261636c654d616e616765723a2054686520726f756e60448201527f6420706f73746564206973206e6f74206166746572207468652065787069727960648201527f2074696d657374616d70000000000000000000000000000000000000000000006084820152fd5b90928b82813d8311611bd8575b611bc781836111ae565b810103126102e25750519138611759565b503d611bbd565b929091926040908151611bf181611192565b600080825280606060209382858201528287820152015267ffffffffffffffff8086169088169569ffffffffffffffffffff9283611c2f848a6111ef565b60011c169873ffffffffffffffffffffffffffffffffffffffff8669ffff000000000000000085168c179860248b51809481937fb633620c0000000000000000000000000000000000000000000000000000000083528d6004840152165afa928315611d33578093611cf6575b50506affffffffffffffffffffff161015611cd0575050835196611cbf88611192565b168652850152830152606082015290565b9192939750959450835196611ce488611192565b87521690850152830152606082015290565b909192508682813d8311611d2c575b611d0f81836111ae565b810103126102e2575051906affffffffffffffffffffff38611c9c565b503d611d05565b8951903d90823e3d90fd5b73ffffffffffffffffffffffffffffffffffffffff8091166000526001602052604060002054168015611d6e5790565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603a60248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520646f60448201527f65736e277420657869737420666f7220746861742061737365740000000000006064820152fd5b73ffffffffffffffffffffffffffffffffffffffff600054163303611e1357565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fdfea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
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
    #[doc = "Container type for all input parameters for the `addAssetOracle` function with signature `addAssetOracle(address,address)` and selector `[140, 180, 186, 35]`"]
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
    #[doc = "Container type for all input parameters for the `assetOracles` function with signature `assetOracles(address)` and selector `[43, 102, 57, 134]`"]
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
    #[doc = "Container type for all input parameters for the `assets` function with signature `assets(uint256)` and selector `[207, 53, 189, 208]`"]
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
    #[doc = "Container type for all input parameters for the `fallbackPeriodSeconds` function with signature `fallbackPeriodSeconds()` and selector `[171, 145, 220, 185]`"]
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
    #[doc = "Container type for all input parameters for the `getAssetOracle` function with signature `getAssetOracle(address)` and selector `[13, 176, 21, 244]`"]
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
    #[doc = "Container type for all input parameters for the `getAssetsLength` function with signature `getAssetsLength()` and selector `[168, 157, 73, 12]`"]
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
    #[doc = "Container type for all input parameters for the `getCurrentPrice` function with signature `getCurrentPrice(address)` and selector `[132, 204, 49, 91]`"]
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
    #[doc = "Container type for all input parameters for the `isValidOption` function with signature `isValidOption(address,uint88,uint256)` and selector `[226, 65, 18, 97]`"]
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
    #[doc = "Container type for all input parameters for the `priceRegistry` function with signature `priceRegistry()` and selector `[83, 81, 49, 215]`"]
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
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
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
    #[doc = "Container type for all input parameters for the `searchRoundToSubmit` function with signature `searchRoundToSubmit(address,uint88)` and selector `[10, 130, 31, 234]`"]
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
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistry` function with signature `setExpiryPriceInRegistry(address,uint88,bytes)` and selector `[185, 189, 135, 150]`"]
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
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistryByRound` function with signature `setExpiryPriceInRegistryByRound(address,uint88,uint256)` and selector `[74, 197, 54, 83]`"]
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
    #[doc = "Container type for all input parameters for the `setExpiryPriceInRegistryFallback` function with signature `setExpiryPriceInRegistryFallback(address,uint88,uint256)` and selector `[110, 227, 130, 123]`"]
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
    #[doc = "Container type for all input parameters for the `strikeAssetDecimals` function with signature `strikeAssetDecimals()` and selector `[193, 50, 86, 97]`"]
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
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
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
    #[doc = "Container type for all return fields from the `assetOracles` function with signature `assetOracles(address)` and selector `[43, 102, 57, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssetOraclesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `assets` function with signature `assets(uint256)` and selector `[207, 53, 189, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssetsReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `fallbackPeriodSeconds` function with signature `fallbackPeriodSeconds()` and selector `[171, 145, 220, 185]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FallbackPeriodSecondsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getAssetOracle` function with signature `getAssetOracle(address)` and selector `[13, 176, 21, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAssetOracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAssetsLength` function with signature `getAssetsLength()` and selector `[168, 157, 73, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAssetsLengthReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getCurrentPrice` function with signature `getCurrentPrice(address)` and selector `[132, 204, 49, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCurrentPriceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isValidOption` function with signature `isValidOption(address,uint88,uint256)` and selector `[226, 65, 18, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsValidOptionReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `priceRegistry` function with signature `priceRegistry()` and selector `[83, 81, 49, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PriceRegistryReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `searchRoundToSubmit` function with signature `searchRoundToSubmit(address,uint88)` and selector `[10, 130, 31, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SearchRoundToSubmitReturn(pub u128);
    #[doc = "Container type for all return fields from the `strikeAssetDecimals` function with signature `strikeAssetDecimals()` and selector `[193, 50, 86, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct StrikeAssetDecimalsReturn(pub u8);
}
