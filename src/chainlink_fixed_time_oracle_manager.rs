pub use chainlink_fixed_time_oracle_manager::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod chainlink_fixed_time_oracle_manager {
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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_priceRegistry\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_strikeAssetDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_fallbackPeriodSeconds\",\"type\":\"uint88\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fixedTime\",\"type\":\"uint24\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isValidTime\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FixedTimeUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OracleAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint88\",\"name\":\"expiryTimestamp\",\"type\":\"uint88\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expiryRoundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"priceSubmitter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isFallback\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PriceRegistrySubmission\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetOracles\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"\",\"type\":\"uint24\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"chainlinkFixedTimeUpdates\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fallbackPeriodSeconds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTime\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidOption\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceRegistry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"searchRoundToSubmit\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistry\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_roundIdAfterExpiry\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryByRound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint88\",\"name\":\"_expiryTimestamp\",\"type\":\"uint88\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpiryPriceInRegistryFallback\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint24\",\"name\":\"fixedTime\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isValidTime\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFixedTimeUpdate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"strikeAssetDecimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CHAINLINKFIXEDTIMEORACLEMANAGER_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x60e0346200017f57601f6200228938819003918201601f19168301916001600160401b0383118484101762000184578084926060946040528339810103126200017f5780516001600160a01b0380821691908282036200017f5760208401519360ff851685036200017f57604001516001600160581b03811693908490036200017f576000543360018060a01b0319821617600055604051923391167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0600085a31562000117575060805260a05260c0526040516120ee90816200019b8239608051818181610e700152818161109f0152611a19015260a05181818161059a0152610de0015260c05181818161042e0152610a620152f35b62461bcd60e51b815260206004820152603560248201527f50726f76696465724f7261636c654d616e616765723a20696e76616c6964207060448201527f72696365207265676973747279206164647265737300000000000000000000006064820152608490fd5b600080fd5b634e487b7160e01b600052604160045260246000fdfe6040608081526004908136101561001557600080fd5b600091823560e01c80630a821fea1461119d5780630db015f4146111415780632b663986146110dc5780634ac53653146110c3578063535131d7146110545780636ee3827b14610db1578063715018a614610d1157806384cc315b1461094057806387e4e00c146108ea5780638cb4ba231461064b5780638da5cb5b146105fa578063a89d490c146105bd578063ab91dcb914610564578063b9bd879614610452578063c1325661146103f6578063c6973b9614610336578063cf35bdd0146102ae578063e24112611461022e5763f2fde38b146100f257600080fd5b3461022a5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261022a576101296111f4565b610131612062565b73ffffffffffffffffffffffffffffffffffffffff8091169182156101a7575060005492827fffffffffffffffffffffffff00000000000000000000000000000000000000008516176000555192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0600084a3f35b60849060208551917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b8280fd5b5050346102aa579060209161024236611231565b50929073ffffffffffffffffffffffffffffffffffffffff80911682526001855282822054161515928361027b575b5050519015158152f35b620151806affffffffffffffffffffff9091160662ffffff168152600384528190205460ff1691503880610271565b5080fd5b503461022a5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261022a573591600254831015610333575073ffffffffffffffffffffffffffffffffffffffff60209260026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace0154169051908152f35b80fd5b5050346102aa57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa5761036e61129a565b602435908115158092036103f2577f1a4d939ae35d2ef6fe079efd40bdae5980befb19ae0806f6ebee6490f1344b459162ffffff84926103ac612062565b169081865260036020528286207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0081541660ff831617905582519182526020820152a151f35b8380fd5b5050346102aa57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa576020905160ff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461022a5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261022a5761048a6111f4565b610492611217565b60443567ffffffffffffffff9384821161056057366023830112156105605781810135948511610534575085908551946104f460207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8401160187611327565b808652366024828401011161022a5761053195816024602094018483013701015269ffffffffffffffffffff61052a8284611572565b1691611860565b51f35b8660416024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b8680fd5b5050346102aa57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa57602090517f00000000000000000000000000000000000000000000000000000000000000008152f35b5050346102aa57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa576020906002549051908152f35b5050346102aa57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa5773ffffffffffffffffffffffffffffffffffffffff60209254169051908152f35b503461022a57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261022a576106826111f4565b906024359073ffffffffffffffffffffffffffffffffffffffff928383168093036108e5576106af612062565b821561086257831692838652600160205284862054166107df5760025490680100000000000000008210156107b357600182018060025582101561078557509183917f828d2be040dede7698182e08dfa8bfbd663c879aee772509c4a2bd961d0ed43f9360026000527f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace017fffffffffffffffffffffffff00000000000000000000000000000000000000009083828254161790558287526001602052818488209182541617905582519182526020820152a151f35b6032907f4e487b71000000000000000000000000000000000000000000000000000000006000525260246000fd5b8560416024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b60849060208551917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603360248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520616c60448201527f72656164792073657420666f72206173736574000000000000000000000000006064820152fd5b60848260208751917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602d60248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520697360448201527f207a65726f2061646472657373000000000000000000000000000000000000006064820152fd5b600080fd5b5050346102aa5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa5760ff8160209362ffffff61092d61129a565b1681526003855220541690519015158152f35b50913461033357602092837ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa5773ffffffffffffffffffffffffffffffffffffffff6109986109936111f4565b611fae565b16918351927ffeaf968c00000000000000000000000000000000000000000000000000000000845260a0848481845afa938415610c50578294610cdc575b5081841315610c5a578583918651928380927f313ce5670000000000000000000000000000000000000000000000000000000082525afa908115610c50579060ff918391610c23575b501692818551610a2e816112ef565b52818551610a3b816112ef565b52601b93808503610b845750610a5090611480565b845190610a5c826112ef565b8152915b7f000000000000000000000000000000000000000000000000000000000000000060ff1693808503610aa257505050610a9a915051611509565b905b51908152f35b80851115610b23578410610af757505090610aeb7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe5610ae4610af19451611509565b9201611408565b90611416565b90610a9c565b9060116024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b9290848495939510610b58575091610b4d610b5392610b46610af1969551611509565b9203611408565b90611447565b611368565b8460116024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b84811115610c0a57848110610bde57610bca91610b4d7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe5610bc59301611408565b611480565b845190610bd6826112ef565b815291610a60565b6024836011867f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b808510610bde57610bca91610aeb610bc5928703611408565b610c439150873d8911610c49575b610c3b8183611327565b8101906113a3565b38610a1f565b503d610c31565b85513d84823e3d90fd5b608483878751917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603160248201527f436861696e6c696e6b4f7261636c654d616e616765723a204e6f20707269636960448201527f6e67206461746120617661696c61626c650000000000000000000000000000006064820152fd5b610cfe91945060a03d8111610d0a575b610cf68183611327565b8101906113d3565b505050905092386109d6565b503d610cec565b5050346102aa57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa57610d49612062565b600073ffffffffffffffffffffffffffffffffffffffff8154927fffffffffffffffffffffffff0000000000000000000000000000000000000000841683555192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b508290346102aa57610dc236611231565b9091610dcc612062565b6affffffffffffffffffffff831691610e057f000000000000000000000000000000000000000000000000000000000000000084611368565b4210610fab5784907f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e07960c0895173ffffffffffffffffffffffffffffffffffffffff9687871682526020820152838b820152896060820152336080820152600160a0820152a16020847f00000000000000000000000000000000000000000000000000000000000000001694610e9a85611fae565b168951978880927f313ce5670000000000000000000000000000000000000000000000000000000082525afa958615610fa1578796610f81575b50833b15610560578694939291610f5786928a51988997889687957feae65f0800000000000000000000000000000000000000000000000000000000875286019094939260ff906affffffffffffffffffffff60609473ffffffffffffffffffffffffffffffffffffffff60808601991685521660208401521660408201520152565b03925af18015610f7757610f6a57509051f35b610f73906112ac565b9051f35b83513d84823e3d90fd5b610f9a91965060203d8111610c4957610c3b8183611327565b9488610ed4565b88513d89823e3d90fd5b60a48560208951917f08c379a0000000000000000000000000000000000000000000000000000000008352820152605460248201527f436861696e6c696e6b4f7261636c654d616e616765723a205468652066616c6c60448201527f6261636b20707269636520706572696f6420686173206e6f742070617373656460648201527f2073696e6365207468652074696d657374616d700000000000000000000000006084820152fd5b5050346102aa57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa576020905173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b5050346102aa576105316110d636611231565b91611860565b5050346102aa5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa576020918173ffffffffffffffffffffffffffffffffffffffff91826111306111f4565b168152600185522054169051908152f35b5050346102aa5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa5760209073ffffffffffffffffffffffffffffffffffffffff6111956109936111f4565b915191168152f35b5050346102aa57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102aa5760209069ffffffffffffffffffff6111956111e66111f4565b6111ee611217565b90611572565b6004359073ffffffffffffffffffffffffffffffffffffffff821682036108e557565b602435906affffffffffffffffffffff821682036108e557565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc60609101126108e55760043573ffffffffffffffffffffffffffffffffffffffff811681036108e557906024356affffffffffffffffffffff811681036108e5579060443590565b6004359062ffffff821682036108e557565b67ffffffffffffffff81116112c057604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6020810190811067ffffffffffffffff8211176112c057604052565b6080810190811067ffffffffffffffff8211176112c057604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176112c057604052565b81198111611374570190565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b908160209103126108e5575160ff811681036108e55790565b519069ffffffffffffffffffff821682036108e557565b908160a09103126108e5576113e7816113bc565b916020820151916040810151916114056080606084015193016113bc565b90565b604d811161137457600a0a90565b807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04821181151516611374570290565b8115611451570490565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b7f80000000000000000000000000000000000000000000000000000000000000008110156114ab5790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f5175616e744d6174683a206f7574206f6620696e742072616e676500000000006044820152fd5b600081126115145790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601760248201527f5175616e744d6174683a206e6567617469766520696e740000000000000000006044820152fd5b61159073ffffffffffffffffffffffffffffffffffffffff91611fae565b169160409081517f8205bf6a000000000000000000000000000000000000000000000000000000008152602093600491858184818a5afa90811561178057600091611833575b506affffffffffffffffffffff8216101561178b578351957f668a0f0200000000000000000000000000000000000000000000000000000000875285878481845afa96871561178057600097611751575b5069ffffffffffffffffffff968716936001938486818111156116cf578a80919894989793975b169116908181106116a157038916600114611695579061166f918484611e4f565b80518782015187830151606090930151908a1692918a16918a16908a908190811661164e565b50955050505092505090565b6011877f4e487b71000000000000000000000000000000000000000000000000000000006000525260246000fd5b6084838b8b51917f08c379a0000000000000000000000000000000000000000000000000000000008352820152603d60248201527f436861696e6c696e6b4f7261636c654d616e616765723a204e6f7420656e6f7560448201527f676820726f756e647320746f2066696e6420726f756e642061667465720000006064820152fd5b90968682813d8311611779575b6117688183611327565b810103126103335750519538611627565b503d61175e565b85513d6000823e3d90fd5b60a482868651917f08c379a0000000000000000000000000000000000000000000000000000000008352820152605460248201527f436861696e6c696e6b4f7261636c654d616e616765723a20546865206c61746560448201527f737420726f756e642074696d657374616d70206973206e6f742061667465722060648201527f746865206578706972792074696d657374616d700000000000000000000000006084820152fd5b908682813d8311611859575b6118498183611327565b81010312610333575051386115d6565b503d61183f565b9073ffffffffffffffffffffffffffffffffffffffff908161188184611fae565b166040938451937fb633620c000000000000000000000000000000000000000000000000000000009081865260049588878201526020986024908a8383818a5afa928315611e4457600093611e15575b506affffffffffffffffffffff881680931115611d6e5767ffffffffffffffff8082169060018210611d41577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff69ffff0000000000000000841692011617948a51818152868b8201528c8185818c5afa908115611d3657908591600091611d06575b5011611c5f576000908b51908152868b8201528c8185818c5afa908115611c55579085918391611c24575b5003611b7657505060a0895180927f9a6fc8f5000000000000000000000000000000000000000000000000000000008252868b83015281895afa908115611b6b5760c08b938a95937f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e07993600091611b48575b50965b8c5191858a16835286830152878d8301526060820152336080820152600060a0820152a17f000000000000000000000000000000000000000000000000000000000000000016948851928380927f313ce5670000000000000000000000000000000000000000000000000000000082525afa978815611b3d57600098611b1e575b5050823b156108e557600094611af686928851998a97889687957feae65f0800000000000000000000000000000000000000000000000000000000875286019094939260ff906affffffffffffffffffffff60609473ffffffffffffffffffffffffffffffffffffffff60808601991685521660208401521660408201520152565b03925af1908115611b145750611b095750565b611b12906112ac565b565b513d6000823e3d90fd5b611b35929850803d10610c4957610c3b8183611327565b953880611a74565b87513d6000823e3d90fd5b611b60915060a03d8111610d0a57610cf68183611327565b5050509050386119f0565b89513d6000823e3d90fd5b9190945060a08a5180927f9a6fc8f500000000000000000000000000000000000000000000000000000000825269ffffffffffffffffffff88168c830152818a5afa918215611c19578b938a95937f6b52c44ea4f324cde3f56343ad632136aa0a3f87fd083e4fe64b4fed29f5e0799360c09391611bf6575b50966119f3565b611c0e915060a03d8111610d0a57610cf68183611327565b505050905038611bef565b8a51903d90823e3d90fd5b8092508e8092503d8311611c4e575b611c3d8183611327565b810103126102aa578490513861197e565b503d611c33565b8c513d84823e3d90fd5b60a48a605a858f8f51937f08c379a00000000000000000000000000000000000000000000000000000000085528401528201527f436861696e6c696e6b4f7261636c654d616e616765723a20457870697279207260448201527f6f756e64207072696f7220746f20746865206f6e6520706f737465642069732060648201527f616674657220746865206578706972792074696d657374616d700000000000006084820152fd5b91508d82813d8311611d2f575b611d1d8183611327565b81010312610333575084905138611953565b503d611d13565b8c513d6000823e3d90fd5b8360118c7f4e487b7100000000000000000000000000000000000000000000000000000000600052526000fd5b60a489604a848e8e51937f08c379a00000000000000000000000000000000000000000000000000000000085528401528201527f436861696e6c696e6b4f7261636c654d616e616765723a2054686520726f756e60448201527f6420706f73746564206973206e6f74206166746572207468652065787069727960648201527f2074696d657374616d70000000000000000000000000000000000000000000006084820152fd5b90928b82813d8311611e3d575b611e2c8183611327565b8101031261033357505191386118d1565b503d611e22565b8a513d6000823e3d90fd5b929091926040908151611e618161130b565b600080825280606060209382858201528287820152015267ffffffffffffffff8086169088169569ffffffffffffffffffff9283611e9f848a611368565b60011c169873ffffffffffffffffffffffffffffffffffffffff8669ffff000000000000000085168c179860248b51809481937fb633620c0000000000000000000000000000000000000000000000000000000083528d6004840152165afa928315611fa3578093611f66575b50506affffffffffffffffffffff161015611f40575050835196611f2f8861130b565b168652850152830152606082015290565b9192939750959450835196611f548861130b565b87521690850152830152606082015290565b909192508682813d8311611f9c575b611f7f8183611327565b81010312610333575051906affffffffffffffffffffff38611f0c565b503d611f75565b8951903d90823e3d90fd5b73ffffffffffffffffffffffffffffffffffffffff8091166000526001602052604060002054168015611fde5790565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603a60248201527f50726f76696465724f7261636c654d616e616765723a204f7261636c6520646f60448201527f65736e277420657869737420666f7220746861742061737365740000000000006064820152fd5b73ffffffffffffffffffffffffffffffffffffffff60005416330361208357565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fdfea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
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
    #[doc = "Container type for all input parameters for the `chainlinkFixedTimeUpdates` function with signature `chainlinkFixedTimeUpdates(uint24)` and selector `[135, 228, 224, 12]`"]
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
        pub expiry_time: u128,
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
    #[doc = "Container type for all input parameters for the `setFixedTimeUpdate` function with signature `setFixedTimeUpdate(uint24,bool)` and selector `[198, 151, 59, 150]`"]
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
    #[doc = "Container type for all return fields from the `chainlinkFixedTimeUpdates` function with signature `chainlinkFixedTimeUpdates(uint24)` and selector `[135, 228, 224, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ChainlinkFixedTimeUpdatesReturn(pub bool);
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
