pub use collateral_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod collateral_token {
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
    #[doc = "CollateralToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COLLATERALTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_version\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"uri_\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"qTokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qTokenAsCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralTokenCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferBatch\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferSingle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"URI\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"owners\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfBatch\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenOwner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralTokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnCollateralToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qTokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createOptionCollateralToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenAsCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createSpreadCollateralToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenAsCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getCollateralTokenId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"idToInfo\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"qTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qTokenAsCollateral\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenOwner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"metaSetApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralTokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintCollateralToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeBatchTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"optionsFactory_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOptionsFactory\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"uri\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"uri_\",\"type\":\"string\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COLLATERALTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101406040818152346200030357620025c2803803809162000022828662000308565b8439820190606083830312620003035782516001600160401b0390818111620003035783620000539186016200032c565b9360209081810151838111620003035785620000719183016200032c565b948482015184811162000303576200008a92016200032c565b9481815191012093818151910120918460e052610100958387524660a052845193838501907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f978883528787015260608601524660808601523060a086015260a0855260c085019480861084871117620002ed578587525190206080523060c05261012095865260028054336001600160a01b0319821681179092556000956001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0908790a38051918211620002d9576005938454916001928381811c91168015620002ce575b86821014620002ba57601f811162000271575b508491601f85116001146200020a579394508492919083620001fe575b50501b916000199060031b1c19161790555b519061220d9283620003b5843960805183611e8a015260a05183611f56015260c05183611e5b015260e05183611ed901525182611eff01525181611eb60152f35b015192503880620001ab565b86815285812093958591601f198316915b888383106200025657505050106200023c575b505050811b019055620001bd565b015160001960f88460031b161c191690553880806200022e565b8587015188559096019594850194879350908101906200021b565b868352858320601f8601881c810191878710620002af575b601f01881c019084905b828110620002a35750506200018e565b84815501849062000293565b909150819062000289565b634e487b7160e01b83526022600452602483fd5b90607f16906200017b565b634e487b7160e01b84526041600452602484fd5b634e487b7160e01b600052604160045260246000fd5b600080fd5b601f909101601f19168101906001600160401b03821190821017620002ed57604052565b9080601f8301121562000303578151906001600160401b038211620002ed576040519260209162000367601f8501601f191684018662000308565b838552828483010111620003035782906000905b838383106200039b575050116200039157505090565b6000918301015290565b819350828193920101518282880101520183916200037b56fe6080604081815260048036101561001557600080fd5b600092833560e01c908162fdd58e146118135750806301ffc9a7146117245780630e89341c146115b35780631c82a559146113f95780632eb2c2d6146111305780633e029179146110ab57806347a95d48146110285780634e1273f414610e725780636ed1ff6d14610e1f578063715018a614610d7e5780637ecebe0014610d1e5780638da5cb5b14610ccb578063a22cb46514610bf3578063b4e60a3214610835578063d94a7e53146107ce578063e1ad922114610619578063e37f5ac7146104d5578063e985e9c51461045c57838163f242432a1461023b575063f2fde38b1461010057600080fd5b346102375760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102375761013761187a565b61013f611ade565b73ffffffffffffffffffffffffffffffffffffffff8091169182156101b4575060025492827fffffffffffffffffffffffff00000000000000000000000000000000000000008516176002555192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08484a3f35b60849060208551917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b8280fd5b919050346104585760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104585761027561187a565b9061027e6118a2565b9160643560443560843567ffffffffffffffff8111610454576102a49036908601611965565b969073ffffffffffffffffffffffffffffffffffffffff809516908133148015610435575b6102d290611f7c565b8183526020958387528a842085855287528a84206102f1878254611fe1565b90558816978884528387528a842085855287528a842061031287825461201b565b905588838c7fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f628151918983528a8c8401523392a43b610361575050505050505061035e9150151561209e565b51f35b906103be8697989493928b519a8b97889687957ff23a6e61000000000000000000000000000000000000000000000000000000009d8e8852339088015260248701526044860152606485015260a0608485015260a484019161205f565b03925af1801561042b5761035e937fffffffff000000000000000000000000000000000000000000000000000000009287926103fe575b5050161461209e565b61041d9250803d10610424575b6104158183611a4f565b810190612027565b38806103f5565b503d61040b565b84513d87823e3d90fd5b50818352600160209081528a842033855290528983205460ff166102c9565b8680fd5b5080fd5b50503461045857807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104585760ff8160209361049a61187a565b6104a26118a2565b73ffffffffffffffffffffffffffffffffffffffff91821683526001875283832091168252855220549151911615158152f35b5034610237576104e436611993565b6104ef939293611ade565b84519060209283830183811067ffffffffffffffff8211176105ed57908891885281845273ffffffffffffffffffffffffffffffffffffffff871696878352828652888320828452865288832061054785825461201b565b905587838a5184815286898201527fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f628c3392a43b6105905750505050505061035e90151561209e565b916103be96918594938951988995869485937ff23a6e61000000000000000000000000000000000000000000000000000000009b8c865233908601528560248601526044850152606484015260a0608484015260a48301906118c5565b6024896041887f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b5091903461045857807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104585761065261187a565b9261065b6118a2565b92610664611ade565b61066e8486611a90565b9373ffffffffffffffffffffffffffffffffffffffff8080921696169286841461072557506020957f7b88fa12e0eaf025be34548ca7c53f68b4ac70c4d36664baf147c0d4226335f39285928351906106c6826119e8565b868252600181868c8501958787528c815260038e5220935116927fffffffffffffffffffffffff00000000000000000000000000000000000000009384825416178155019251169082541617905581519081528587820152a251908152f35b60a49060208651917f08c379a0000000000000000000000000000000000000000000000000000000008352820152604960248201527f436f6c6c61746572616c546f6b656e3a2043616e206f6e6c792063726561746560448201527f206120636f6c6c61746572616c20746f6b656e2077697468206469666665726560648201527f6e7420746f6b656e7300000000000000000000000000000000000000000000006084820152fd5b50346102375760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102375791819235815260036020522073ffffffffffffffffffffffffffffffffffffffff6001818354169201541682519182526020820152f35b5034610237576101007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102375761086e61187a565b6108766118a2565b906044358015159081810361045457606435946084359360a4359560ff87168703610bef5773ffffffffffffffffffffffffffffffffffffffff80921696878b526020968488528a8c20548a03610b9357428110610b11578a90848251948b8b8701947f8733d126a676f1e83270eccfbe576f65af55d3ff784c4dc4884be48932f47c818652870152169a8b606086015288608086015260a085015260c084015260c0835260e083019267ffffffffffffffff9381811085821117610ae5578c5251902090610943611e44565b928b5192898401947f190100000000000000000000000000000000000000000000000000000000000086526022850152604284015260428352608083019083821090821117610ab957926109b2926109aa928b96958e5260e4359260c43592519020611d82565b919091611b5d565b1603610a37577f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31939291610a2e91868a5284528789206001815401905560018452878920878a5284528789209060ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0083541691151516179055565b8551908152a351f35b608490848851917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602260248201527f436f6c6c61746572616c546f6b656e3a20696e76616c6964207369676e61747560448201527f72650000000000000000000000000000000000000000000000000000000000006064820152fd5b60248d6041887f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b60248e6041897f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b608485898d51917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602160248201527f436f6c6c61746572616c546f6b656e3a206578706972656420646561646c696e60448201527f65000000000000000000000000000000000000000000000000000000000000006064820152fd5b606485898d51917f08c379a0000000000000000000000000000000000000000000000000000000008352820152601e60248201527f436f6c6c61746572616c546f6b656e3a20696e76616c6964206e6f6e636500006044820152fd5b8980fd5b50503461045857807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261045857610c2b61187a565b60243580151590818103610cc757610c9990338652600160205273ffffffffffffffffffffffffffffffffffffffff8587209416938487526020528486209060ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0083541691151516179055565b82519081527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c3160203392a351f35b8480fd5b50503461045857817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104585760209073ffffffffffffffffffffffffffffffffffffffff600254169051908152f35b50346102375760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261023757602092829173ffffffffffffffffffffffffffffffffffffffff610d7061187a565b168252845220549051908152f35b50503461045857817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261045857610db6611ade565b8173ffffffffffffffffffffffffffffffffffffffff600254927fffffffffffffffffffffffff000000000000000000000000000000000000000084166002555192167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b50503461045857807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261045857602090610e6b610e5d61187a565b610e656118a2565b90611a90565b9051908152f35b5082903461045857827ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104585767ffffffffffffffff91813583811161045857610ec39036908401611934565b91909360249081359081116102375791848692610ee4899536908401611934565b949092610ef2868914612103565b610efb886121e8565b93610f0888519586611a4f565b888552610f14896121e8565b966020997fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08b88019901368a37875b818110610f8c575050505050505083519485948186019282875251809352850193925b828110610f7557505050500390f35b835185528695509381019392810192600101610f66565b610f9d8183889e9b9a9c9d9e612168565b3573ffffffffffffffffffffffffffffffffffffffff81168091036110245788528789528b8820610fcf828587612168565b35895289528b8820548a51821015610ff957600582901b8b018a0152979a99989697600101610f43565b87896032887f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b8880fd5b5050346104585760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104585773ffffffffffffffffffffffffffffffffffffffff61107661187a565b61107e611ade565b167fffffffffffffffffffffffff0000000000000000000000000000000000000000600654161760065551f35b505034610458578173ffffffffffffffffffffffffffffffffffffffff6110d136611993565b9290916110dc611ade565b1691828452836020528484208285526020528484206110fc828254611fe1565b9055845191825260208201527fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62843392a451f35b5034610237577ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc9060a0823601126113f55761116a61187a565b906111736118a2565b67ffffffffffffffff906044358281116113f1576111949036908501611934565b929095606435828111610bef576111ae9036908701611934565b926084359081116113ed576111c69036908801611965565b9190936111d4818814612103565b73ffffffffffffffffffffffffffffffffffffffff988916956112033388148d8f82156113ce575b5050611f7c565b8c878d8d839d85169d8e945b8d8083106113665761124e92507f4a39dc06d4c0dbc64b70af90fd698a233a518aa5d07e595d983b8c0526c8f7fb9391508451948086528501916121a7565b918083036020820152806112643394898b6121a7565b0390a43b61128057505050505050505061035e9150151561209e565b8a98979851998a9788977fbc197c81000000000000000000000000000000000000000000000000000000009b8c8a5233908a015260248901526044880160a0905260a48801906112cf926121a7565b90848783030160648801526112e3926121a7565b918483030160848501526112f69261205f565b0381875a94602095f19182156113595761035e927fffffffff0000000000000000000000000000000000000000000000000000000091869161133b575b50161461209e565b611353915060203d8111610424576104158183611a4f565b38611333565b50505051903d90823e3d90fd5b9561137b836001986113bf9698979598612168565b3590611388878b8d612168565b3595835260209083825284842083855282528484206113a8888254611fe1565b90558352828152838320918352522091825461201b565b905501878d8f8d938f9161120f565b60ff92508a815260016020528181203382526020522054168d8f6111fc565b8a80fd5b8780fd5b8380fd5b50903461023757602092837ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126115b05761143461187a565b9073ffffffffffffffffffffffffffffffffffffffff938460065416330361152e5750827f7b88fa12e0eaf025be34548ca7c53f68b4ac70c4d36664baf147c0d4226335f3918151878101907fffffffffffffffffffffffffffffffffffffffff0000000000000000000000008660601b168252826034820152602881526114bb81611a33565b51902095808351956114cc876119e8565b16948581528881019183835288845260038a52600181868620935116927fffffffffffffffffffffffff00000000000000000000000000000000000000009384825416178155019251169082541617905581519081528587820152a251908152f35b608490868551917f08c379a0000000000000000000000000000000000000000000000000000000008352820152602d60248201527f436f6c6c61746572616c546f6b656e3a2063616c6c6572206973206e6f74204f60448201527f7074696f6e73466163746f7279000000000000000000000000000000000000006064820152fd5b80fd5b5091903461045857602090817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261023757805192806005549060019082821c92828116801561171a575b87851081146116ee578899509688969785829a5291826000146116a957505060011461164d575b505050611649929161163a910385611a4f565b519282849384528301906118c5565b0390f35b9190869350600583527f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db05b828410611691575050508201018161163a611649611627565b8054848a018601528895508794909301928101611678565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00168782015293151560051b8601909301935084925061163a91506116499050611627565b60248360228c7f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b93607f1693611600565b50346102375760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102375735907fffffffff00000000000000000000000000000000000000000000000000000000821680920361023757602092507f01ffc9a70000000000000000000000000000000000000000000000000000000082149182156117e9575b82156117bf575b50519015158152f35b7f0e89341c00000000000000000000000000000000000000000000000000000000149150386117b6565b7fd9b67a2600000000000000000000000000000000000000000000000000000000811492506117af565b8490843461023757807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102375760209273ffffffffffffffffffffffffffffffffffffffff61186461187a565b1681528084528181206024358252845220548152f35b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361189d57565b600080fd5b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361189d57565b91908251928382526000905b84821061191c5750601f84602094957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0931161190f575b0116010190565b6000858286010152611908565b906020908180828501015190828601015201906118d1565b9181601f8401121561189d5782359167ffffffffffffffff831161189d576020808501948460051b01011161189d57565b9181601f8401121561189d5782359167ffffffffffffffff831161189d576020838186019501011161189d57565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc606091011261189d5760043573ffffffffffffffffffffffffffffffffffffffff8116810361189d57906024359060443590565b6040810190811067ffffffffffffffff821117611a0457604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6060810190811067ffffffffffffffff821117611a0457604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117611a0457604052565b6040519060208201927fffffffffffffffffffffffffffffffffffffffff000000000000000000000000809260601b16845260601b16603482015260288152611ad881611a33565b51902090565b73ffffffffffffffffffffffffffffffffffffffff600254163303611aff57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b6005811015611d535780611b6e5750565b60018103611bd45760646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152fd5b60028103611c3a5760646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152fd5b60038103611cc65760846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c60448201527f75650000000000000000000000000000000000000000000000000000000000006064820152fd5b600414611ccf57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c60448201527f75650000000000000000000000000000000000000000000000000000000000006064820152fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311611e385760ff16601b81141580611e2d575b611e21579160809493916020936040519384528484015260408301526060820152600093849182805260015afa15611e1457815173ffffffffffffffffffffffffffffffffffffffff811615611e0e579190565b50600190565b50604051903d90823e3d90fd5b50505050600090600490565b50601c811415611dba565b50505050600090600390565b73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016301480611f53575b15611eac577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f000000000000000000000000000000000000000000000000000000000000000082527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a0815260c0810181811067ffffffffffffffff821117611a045760405251902090565b507f00000000000000000000000000000000000000000000000000000000000000004614611e83565b15611f8357565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f4e4f545f415554484f52495a45440000000000000000000000000000000000006044820152fd5b818110611fec570390565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b81198111611fec570190565b9081602091031261189d57517fffffffff000000000000000000000000000000000000000000000000000000008116810361189d5790565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0938186528686013760008582860101520116010190565b156120a557565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601060248201527f554e534146455f524543495049454e54000000000000000000000000000000006044820152fd5b1561210a57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600f60248201527f4c454e4754485f4d49534d4154434800000000000000000000000000000000006044820152fd5b91908110156121785760051b0190565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b90918281527f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831161189d5760209260051b80928483013701016000815290565b67ffffffffffffffff8111611a045760051b6020019056fea164736f6c634300080f000a" . parse () . expect ("invalid bytecode")
        });
    pub struct CollateralToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CollateralToken<M> {
        fn clone(&self) -> Self {
            CollateralToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CollateralToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CollateralToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CollateralToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CollateralToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COLLATERALTOKEN_ABI.clone(), client)
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
                COLLATERALTOKEN_ABI.clone(),
                COLLATERALTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `balanceOf` (0x00fdd58e) function"]
        pub fn balance_of(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOfBatch` (0x4e1273f4) function"]
        pub fn balance_of_batch(
            &self,
            owners: ::std::vec::Vec<ethers::core::types::Address>,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([78, 18, 115, 244], (owners, ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnCollateralToken` (0x3e029179) function"]
        pub fn burn_collateral_token(
            &self,
            c_token_owner: ethers::core::types::Address,
            collateral_token_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [62, 2, 145, 121],
                    (c_token_owner, collateral_token_id, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createOptionCollateralToken` (0x1c82a559) function"]
        pub fn create_option_collateral_token(
            &self,
            q_token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 130, 165, 89], q_token_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createSpreadCollateralToken` (0xe1ad9221) function"]
        pub fn create_spread_collateral_token(
            &self,
            q_token_address: ethers::core::types::Address,
            q_token_as_collateral: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [225, 173, 146, 33],
                    (q_token_address, q_token_as_collateral),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCollateralTokenId` (0x6ed1ff6d) function"]
        pub fn get_collateral_token_id(
            &self,
            q_token: ethers::core::types::Address,
            q_token_as_collateral: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([110, 209, 255, 109], (q_token, q_token_as_collateral))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `idToInfo` (0xd94a7e53) function"]
        pub fn id_to_info(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, ethers::core::types::Address),
        > {
            self.0
                .method_hash([217, 74, 126, 83], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `metaSetApprovalForAll` (0xb4e60a32) function"]
        pub fn meta_set_approval_for_all(
            &self,
            c_token_owner: ethers::core::types::Address,
            operator: ethers::core::types::Address,
            approved: bool,
            nonce: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [180, 230, 10, 50],
                    (c_token_owner, operator, approved, nonce, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintCollateralToken` (0xe37f5ac7) function"]
        pub fn mint_collateral_token(
            &self,
            recipient: ethers::core::types::Address,
            collateral_token_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 127, 90, 199],
                    (recipient, collateral_token_id, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
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
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeBatchTransferFrom` (0x2eb2c2d6) function"]
        pub fn safe_batch_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            ids: ::std::vec::Vec<ethers::core::types::U256>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 178, 194, 214], (from, to, ids, amounts, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xf242432a) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 66, 67, 42], (from, to, id, amount, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOptionsFactory` (0x47a95d48) function"]
        pub fn set_options_factory(
            &self,
            options_factory: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 169, 93, 72], options_factory)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        #[doc = "Calls the contract's `uri` (0x0e89341c) function"]
        pub fn uri(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([14, 137, 52, 28], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CollateralTokenCreated` event"]
        pub fn collateral_token_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollateralTokenCreatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferBatch` event"]
        pub fn transfer_batch_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferBatchFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferSingle` event"]
        pub fn transfer_single_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferSingleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `URI` event"]
        pub fn uri_filter(&self) -> ethers::contract::builders::Event<M, UriFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CollateralTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CollateralToken<M> {
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
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
        name = "CollateralTokenCreated",
        abi = "CollateralTokenCreated(address,address,uint256)"
    )]
    pub struct CollateralTokenCreatedFilter {
        #[ethevent(indexed)]
        pub q_token_address: ethers::core::types::Address,
        pub q_token_as_collateral: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
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
        name = "TransferBatch",
        abi = "TransferBatch(address,address,address,uint256[],uint256[])"
    )]
    pub struct TransferBatchFilter {
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub ids: Vec<ethers::core::types::U256>,
        pub amounts: Vec<ethers::core::types::U256>,
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
        name = "TransferSingle",
        abi = "TransferSingle(address,address,address,uint256,uint256)"
    )]
    pub struct TransferSingleFilter {
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "URI", abi = "URI(string,uint256)")]
    pub struct UriFilter {
        pub value: String,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CollateralTokenEvents {
        ApprovalForAllFilter(ApprovalForAllFilter),
        CollateralTokenCreatedFilter(CollateralTokenCreatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferBatchFilter(TransferBatchFilter),
        TransferSingleFilter(TransferSingleFilter),
        UriFilter(UriFilter),
    }
    impl ethers::contract::EthLogDecode for CollateralTokenEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(CollateralTokenEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = CollateralTokenCreatedFilter::decode_log(log) {
                return Ok(CollateralTokenEvents::CollateralTokenCreatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(CollateralTokenEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TransferBatchFilter::decode_log(log) {
                return Ok(CollateralTokenEvents::TransferBatchFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(CollateralTokenEvents::TransferSingleFilter(decoded));
            }
            if let Ok(decoded) = UriFilter::decode_log(log) {
                return Ok(CollateralTokenEvents::UriFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CollateralTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CollateralTokenEvents::ApprovalForAllFilter(element) => element.fmt(f),
                CollateralTokenEvents::CollateralTokenCreatedFilter(element) => element.fmt(f),
                CollateralTokenEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                CollateralTokenEvents::TransferBatchFilter(element) => element.fmt(f),
                CollateralTokenEvents::TransferSingleFilter(element) => element.fmt(f),
                CollateralTokenEvents::UriFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `[0, 253, 213, 142]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `[78, 18, 115, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOfBatch", abi = "balanceOfBatch(address[],uint256[])")]
    pub struct BalanceOfBatchCall {
        pub owners: ::std::vec::Vec<ethers::core::types::Address>,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `burnCollateralToken` function with signature `burnCollateralToken(address,uint256,uint256)` and selector `[62, 2, 145, 121]`"]
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
        name = "burnCollateralToken",
        abi = "burnCollateralToken(address,uint256,uint256)"
    )]
    pub struct BurnCollateralTokenCall {
        pub c_token_owner: ethers::core::types::Address,
        pub collateral_token_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `createOptionCollateralToken` function with signature `createOptionCollateralToken(address)` and selector `[28, 130, 165, 89]`"]
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
        name = "createOptionCollateralToken",
        abi = "createOptionCollateralToken(address)"
    )]
    pub struct CreateOptionCollateralTokenCall {
        pub q_token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `createSpreadCollateralToken` function with signature `createSpreadCollateralToken(address,address)` and selector `[225, 173, 146, 33]`"]
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
        name = "createSpreadCollateralToken",
        abi = "createSpreadCollateralToken(address,address)"
    )]
    pub struct CreateSpreadCollateralTokenCall {
        pub q_token_address: ethers::core::types::Address,
        pub q_token_as_collateral: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCollateralTokenId` function with signature `getCollateralTokenId(address,address)` and selector `[110, 209, 255, 109]`"]
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
        name = "getCollateralTokenId",
        abi = "getCollateralTokenId(address,address)"
    )]
    pub struct GetCollateralTokenIdCall {
        pub q_token: ethers::core::types::Address,
        pub q_token_as_collateral: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `idToInfo` function with signature `idToInfo(uint256)` and selector `[217, 74, 126, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "idToInfo", abi = "idToInfo(uint256)")]
    pub struct IdToInfoCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `metaSetApprovalForAll` function with signature `metaSetApprovalForAll(address,address,bool,uint256,uint256,uint8,bytes32,bytes32)` and selector `[180, 230, 10, 50]`"]
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
        name = "metaSetApprovalForAll",
        abi = "metaSetApprovalForAll(address,address,bool,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct MetaSetApprovalForAllCall {
        pub c_token_owner: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
        pub approved: bool,
        pub nonce: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `mintCollateralToken` function with signature `mintCollateralToken(address,uint256,uint256)` and selector `[227, 127, 90, 199]`"]
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
        name = "mintCollateralToken",
        abi = "mintCollateralToken(address,uint256,uint256)"
    )]
    pub struct MintCollateralTokenCall {
        pub recipient: ethers::core::types::Address,
        pub collateral_token_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `safeBatchTransferFrom` function with signature `safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)` and selector `[46, 178, 194, 214]`"]
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
        name = "safeBatchTransferFrom",
        abi = "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct SafeBatchTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub ids: ::std::vec::Vec<ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,uint256,bytes)` and selector `[242, 66, 67, 42]`"]
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,uint256,bytes)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[doc = "Container type for all input parameters for the `setOptionsFactory` function with signature `setOptionsFactory(address)` and selector `[71, 169, 93, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setOptionsFactory", abi = "setOptionsFactory(address)")]
    pub struct SetOptionsFactoryCall {
        pub options_factory: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
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
    #[doc = "Container type for all input parameters for the `uri` function with signature `uri(uint256)` and selector `[14, 137, 52, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "uri", abi = "uri(uint256)")]
    pub struct UriCall(pub ethers::core::types::U256);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CollateralTokenCalls {
        BalanceOf(BalanceOfCall),
        BalanceOfBatch(BalanceOfBatchCall),
        BurnCollateralToken(BurnCollateralTokenCall),
        CreateOptionCollateralToken(CreateOptionCollateralTokenCall),
        CreateSpreadCollateralToken(CreateSpreadCollateralTokenCall),
        GetCollateralTokenId(GetCollateralTokenIdCall),
        IdToInfo(IdToInfoCall),
        IsApprovedForAll(IsApprovedForAllCall),
        MetaSetApprovalForAll(MetaSetApprovalForAllCall),
        MintCollateralToken(MintCollateralTokenCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SafeBatchTransferFrom(SafeBatchTransferFromCall),
        SafeTransferFrom(SafeTransferFromCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetOptionsFactory(SetOptionsFactoryCall),
        SupportsInterface(SupportsInterfaceCall),
        TransferOwnership(TransferOwnershipCall),
        Uri(UriCall),
    }
    impl ethers::core::abi::AbiDecode for CollateralTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::BalanceOfBatch(decoded));
            }
            if let Ok(decoded) =
                <BurnCollateralTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::BurnCollateralToken(decoded));
            }
            if let Ok(decoded) =
                <CreateOptionCollateralTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CollateralTokenCalls::CreateOptionCollateralToken(decoded));
            }
            if let Ok(decoded) =
                <CreateSpreadCollateralTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CollateralTokenCalls::CreateSpreadCollateralToken(decoded));
            }
            if let Ok(decoded) =
                <GetCollateralTokenIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::GetCollateralTokenId(decoded));
            }
            if let Ok(decoded) =
                <IdToInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::IdToInfo(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) =
                <MetaSetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::MetaSetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <MintCollateralTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::MintCollateralToken(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::Nonces(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SafeBatchTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::SafeBatchTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SetOptionsFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::SetOptionsFactory(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CollateralTokenCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UriCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CollateralTokenCalls::Uri(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CollateralTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CollateralTokenCalls::BalanceOf(element) => element.encode(),
                CollateralTokenCalls::BalanceOfBatch(element) => element.encode(),
                CollateralTokenCalls::BurnCollateralToken(element) => element.encode(),
                CollateralTokenCalls::CreateOptionCollateralToken(element) => element.encode(),
                CollateralTokenCalls::CreateSpreadCollateralToken(element) => element.encode(),
                CollateralTokenCalls::GetCollateralTokenId(element) => element.encode(),
                CollateralTokenCalls::IdToInfo(element) => element.encode(),
                CollateralTokenCalls::IsApprovedForAll(element) => element.encode(),
                CollateralTokenCalls::MetaSetApprovalForAll(element) => element.encode(),
                CollateralTokenCalls::MintCollateralToken(element) => element.encode(),
                CollateralTokenCalls::Nonces(element) => element.encode(),
                CollateralTokenCalls::Owner(element) => element.encode(),
                CollateralTokenCalls::RenounceOwnership(element) => element.encode(),
                CollateralTokenCalls::SafeBatchTransferFrom(element) => element.encode(),
                CollateralTokenCalls::SafeTransferFrom(element) => element.encode(),
                CollateralTokenCalls::SetApprovalForAll(element) => element.encode(),
                CollateralTokenCalls::SetOptionsFactory(element) => element.encode(),
                CollateralTokenCalls::SupportsInterface(element) => element.encode(),
                CollateralTokenCalls::TransferOwnership(element) => element.encode(),
                CollateralTokenCalls::Uri(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CollateralTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CollateralTokenCalls::BalanceOf(element) => element.fmt(f),
                CollateralTokenCalls::BalanceOfBatch(element) => element.fmt(f),
                CollateralTokenCalls::BurnCollateralToken(element) => element.fmt(f),
                CollateralTokenCalls::CreateOptionCollateralToken(element) => element.fmt(f),
                CollateralTokenCalls::CreateSpreadCollateralToken(element) => element.fmt(f),
                CollateralTokenCalls::GetCollateralTokenId(element) => element.fmt(f),
                CollateralTokenCalls::IdToInfo(element) => element.fmt(f),
                CollateralTokenCalls::IsApprovedForAll(element) => element.fmt(f),
                CollateralTokenCalls::MetaSetApprovalForAll(element) => element.fmt(f),
                CollateralTokenCalls::MintCollateralToken(element) => element.fmt(f),
                CollateralTokenCalls::Nonces(element) => element.fmt(f),
                CollateralTokenCalls::Owner(element) => element.fmt(f),
                CollateralTokenCalls::RenounceOwnership(element) => element.fmt(f),
                CollateralTokenCalls::SafeBatchTransferFrom(element) => element.fmt(f),
                CollateralTokenCalls::SafeTransferFrom(element) => element.fmt(f),
                CollateralTokenCalls::SetApprovalForAll(element) => element.fmt(f),
                CollateralTokenCalls::SetOptionsFactory(element) => element.fmt(f),
                CollateralTokenCalls::SupportsInterface(element) => element.fmt(f),
                CollateralTokenCalls::TransferOwnership(element) => element.fmt(f),
                CollateralTokenCalls::Uri(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CollateralTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            CollateralTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfBatchCall> for CollateralTokenCalls {
        fn from(var: BalanceOfBatchCall) -> Self {
            CollateralTokenCalls::BalanceOfBatch(var)
        }
    }
    impl ::std::convert::From<BurnCollateralTokenCall> for CollateralTokenCalls {
        fn from(var: BurnCollateralTokenCall) -> Self {
            CollateralTokenCalls::BurnCollateralToken(var)
        }
    }
    impl ::std::convert::From<CreateOptionCollateralTokenCall> for CollateralTokenCalls {
        fn from(var: CreateOptionCollateralTokenCall) -> Self {
            CollateralTokenCalls::CreateOptionCollateralToken(var)
        }
    }
    impl ::std::convert::From<CreateSpreadCollateralTokenCall> for CollateralTokenCalls {
        fn from(var: CreateSpreadCollateralTokenCall) -> Self {
            CollateralTokenCalls::CreateSpreadCollateralToken(var)
        }
    }
    impl ::std::convert::From<GetCollateralTokenIdCall> for CollateralTokenCalls {
        fn from(var: GetCollateralTokenIdCall) -> Self {
            CollateralTokenCalls::GetCollateralTokenId(var)
        }
    }
    impl ::std::convert::From<IdToInfoCall> for CollateralTokenCalls {
        fn from(var: IdToInfoCall) -> Self {
            CollateralTokenCalls::IdToInfo(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for CollateralTokenCalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            CollateralTokenCalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<MetaSetApprovalForAllCall> for CollateralTokenCalls {
        fn from(var: MetaSetApprovalForAllCall) -> Self {
            CollateralTokenCalls::MetaSetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<MintCollateralTokenCall> for CollateralTokenCalls {
        fn from(var: MintCollateralTokenCall) -> Self {
            CollateralTokenCalls::MintCollateralToken(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for CollateralTokenCalls {
        fn from(var: NoncesCall) -> Self {
            CollateralTokenCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for CollateralTokenCalls {
        fn from(var: OwnerCall) -> Self {
            CollateralTokenCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for CollateralTokenCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            CollateralTokenCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SafeBatchTransferFromCall> for CollateralTokenCalls {
        fn from(var: SafeBatchTransferFromCall) -> Self {
            CollateralTokenCalls::SafeBatchTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for CollateralTokenCalls {
        fn from(var: SafeTransferFromCall) -> Self {
            CollateralTokenCalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for CollateralTokenCalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            CollateralTokenCalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SetOptionsFactoryCall> for CollateralTokenCalls {
        fn from(var: SetOptionsFactoryCall) -> Self {
            CollateralTokenCalls::SetOptionsFactory(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for CollateralTokenCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            CollateralTokenCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for CollateralTokenCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            CollateralTokenCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UriCall> for CollateralTokenCalls {
        fn from(var: UriCall) -> Self {
            CollateralTokenCalls::Uri(var)
        }
    }
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `[0, 253, 213, 142]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `[78, 18, 115, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfBatchReturn {
        pub balances: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `createOptionCollateralToken` function with signature `createOptionCollateralToken(address)` and selector `[28, 130, 165, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreateOptionCollateralTokenReturn {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `createSpreadCollateralToken` function with signature `createSpreadCollateralToken(address,address)` and selector `[225, 173, 146, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreateSpreadCollateralTokenReturn {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getCollateralTokenId` function with signature `getCollateralTokenId(address,address)` and selector `[110, 209, 255, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCollateralTokenIdReturn {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `idToInfo` function with signature `idToInfo(uint256)` and selector `[217, 74, 126, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IdToInfoReturn {
        pub q_token_address: ethers::core::types::Address,
        pub q_token_as_collateral: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    #[doc = "Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NoncesReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    #[doc = "Container type for all return fields from the `uri` function with signature `uri(uint256)` and selector `[14, 137, 52, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UriReturn {
        pub uri: String,
    }
}
