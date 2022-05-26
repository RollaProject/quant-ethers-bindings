pub use collateraltoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod collateraltoken_mod {
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
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_version\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"uri_\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"qTokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"qTokenAsCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralTokenCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferBatch\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferSingle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"value\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"URI\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"owners\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfBatch\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenOwner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralTokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnCollateralToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qTokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createOptionCollateralToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenAsCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createSpreadCollateralToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_qToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_qTokenAsCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getCollateralTokenId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"idToInfo\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"qTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"qTokenAsCollateral\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenOwner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"metaSetApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralTokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintCollateralToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeBatchTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"optionsFactory_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOptionsFactory\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"uri\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"uri_\",\"type\":\"string\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COLLATERALTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101408060405234620000dd5762002b53803803809162000021828562000124565b8339810190606081830312620000dd5780516001600160401b039290838111620000dd57816200005391840162000148565b916020810151848111620000dd57826200006f91830162000148565b916040820151948511620000dd5762000095946200008e920162000148565b916200037b565b604051612702908162000451823960805181612036015260a051816120eb015260c05181612007015260e05181612085015261010051816120ab015261012051816120620152f35b600080fd5b50634e487b7160e01b600052604160045260246000fd5b60c081019081106001600160401b038211176200011557604052565b6200011f620000e2565b604052565b601f909101601f19168101906001600160401b038211908210176200011557604052565b9080601f83011215620000dd578151906001600160401b038211620001d1575b6040519260209162000184601f8501601f191684018662000124565b838552828483010111620000dd5782906000905b83838310620001b857505011620001ae57505090565b6000918301015290565b8193508281939201015182828801015201839162000198565b620001db620000e2565b62000168565b90600182811c9216801562000213575b6020831014620001fd57565b634e487b7160e01b600052602260045260246000fd5b91607f1691620001f1565b601f81116200022b575050565b6000906005825260208220906020601f850160051c830194106200026c575b601f0160051c01915b8281106200026057505050565b81815560010162000253565b90925082906200024a565b80519091906001600160401b0381116200036b575b620002a4816200029e600554620001e1565b6200021e565b602080601f8311600114620002e35750819293600092620002d7575b50508160011b916000199060031b1c191617600555565b015190503880620002c0565b6005600052601f198316949091907f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db0926000905b8782106200035257505083600195961062000338575b505050811b01600555565b015160001960f88460031b161c191690553880806200032d565b8060018596829496860151815501950193019062000317565b62000375620000e2565b6200028c565b906200044e92916020815191012090602081519101208160e05280610100524660a05260405160208101917f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f93848452604083015260608201524660808201523060a082015260a08152620003f081620000f9565b5190206080523060c0526101205260028054336001600160a01b0319821681179092556040516001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a362000277565b56fe60806040526004361015610013575b600080fd5b60003560e01c8062fdd58e146101a257806301ffc9a7146101995780630e89341c146101905780631c82a559146101875780632eb2c2d61461017e5780633e0291791461017557806347a95d481461016c5780634e1273f4146101635780636ed1ff6d1461015a578063715018a6146101515780637ecebe00146101485780638da5cb5b1461013f578063a22cb46514610136578063b4e60a321461012d578063d94a7e5314610124578063e1ad92211461011b578063e37f5ac714610112578063e985e9c514610109578063f242432a146101005763f2fde38b146100f857600080fd5b61000e6114b5565b5061000e611250565b5061000e6111e6565b5061000e61103b565b5061000e610e7c565b5061000e610e10565b5061000e610d94565b5061000e610c9a565b5061000e610c47565b5061000e610bdd565b5061000e610b33565b5061000e610b11565b5061000e610989565b5061000e6108bc565b5061000e61080b565b5061000e61071f565b5061000e610536565b5061000e6103df565b5061000e61026b565b5061000e6101c9565b73ffffffffffffffffffffffffffffffffffffffff81160361000e57565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff60043561021a816101ab565b16600052600060205260406000206024356000526020526020604060002054604051908152f35b7fffffffff0000000000000000000000000000000000000000000000000000000081160361000e57565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760207fffffffff000000000000000000000000000000000000000000000000000000006004356102ca81610241565b167f01ffc9a7000000000000000000000000000000000000000000000000000000008114908115610332575b8115610308575b506040519015158152f35b7f0e89341c00000000000000000000000000000000000000000000000000000000915014386102fd565b7fd9b67a2600000000000000000000000000000000000000000000000000000000811491506102f6565b91908251928382526000905b8482106103b35750601f84602094957fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe093116103a6575b0116010190565b600085828601015261039f565b90602090818082850101519082860101520190610368565b9060206103dc92818152019061035c565b90565b503461000e576020807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576040519060009060055460019181831c9280831692831561052c575b82851084146104ff5784875260208701939081156104c7575060011461046d575b6104698661045d81880382611746565b604051918291826103cb565b0390f35b6005600090815294509192917f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db05b8386106104b6575050509101905061045d826104693861044d565b80548587015294820194810161049b565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00168452505001915061045d9050826104693861044d565b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526022600452fd5b93607f169361042c565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610469600435610575816101ab565b73ffffffffffffffffffffffffffffffffffffffff8060025416331480156106b3575b6105a19061160e565b6106656105ad83611b40565b92600183604051926105be846116c9565b16938483526020830160008152866000526003602052610621826040600020955116859073ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b511691019073ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b7f7b88fa12e0eaf025be34548ca7c53f68b4ac70c4d36664baf147c0d4226335f3604051806106a08582919060206040840193600081520152565b0390a26040519081529081906020820190565b5060065481163314610598565b9181601f8401121561000e5782359167ffffffffffffffff831161000e576020808501948460051b01011161000e57565b9181601f8401121561000e5782359167ffffffffffffffff831161000e576020838186019501011161000e57565b503461000e5760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043561075b816101ab565b602435610767816101ab565b67ffffffffffffffff9160443583811161000e576107899036906004016106c0565b9060643585811161000e576107a29036906004016106c0565b92909160843596871161000e576107c06107c89736906004016106f1565b969095612449565b005b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc606091011261000e57600435610800816101ab565b906024359060443590565b503461000e576108a761081d366107ca565b9173ffffffffffffffffffffffffffffffffffffffff90610843826002541633146115a9565b16907fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f6260009485948486528560205260408620848752602052604086208181548181106108af575b0390556040805194855260208501919091523393918291820190565b0390a4604051f35b6108b7612176565b61088b565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576004356108f8816101ab565b73ffffffffffffffffffffffffffffffffffffffff9061091d826002541633146115a9565b167fffffffffffffffffffffffff000000000000000000000000000000000000000060065416176006556000604051f35b6020908160408183019282815285518094520193019160005b828110610975575050505090565b835185529381019392810192600101610967565b503461000e576040807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5767ffffffffffffffff9060043582811161000e576109dc9036906004016106c0565b91909260243590811161000e576109f79036906004016106c0565b9390610a048585146122d0565b610a0d84612649565b93610a1a84519586611746565b8085527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0610a4782612649565b016020903682880137600091825b818110610a69578651806104698a8261094e565b8073ffffffffffffffffffffffffffffffffffffffff610a8c600193858a612365565b35610a96816101ab565b168552848452610abc888620610aad838d8a612365565b35600052602052604060002090565b54610ac7828b61266a565b5201610a55565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc604091011261000e57600435610b04816101ab565b906024356103dc816101ab565b503461000e576020610b2b610b2536610ace565b90611b89565b604051908152f35b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610bda576002547fffffffffffffffffffffffff000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff821691610bab3384146115a9565b1660025581604051917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08284a3f35b80fd5b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff600435610c2e816101ab565b1660005260046020526020604060002054604051908152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602073ffffffffffffffffffffffffffffffffffffffff60025416604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57600435610cd6816101ab565b6024358015159182820361000e57610d6473ffffffffffffffffffffffffffffffffffffffff92336000526001602052610d348360406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b9060ff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0083541691151516179055565b60405192835216907f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c3160203392a3005b503461000e576101007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57600435610dd1816101ab565b60243590610dde826101ab565b604435801515810361000e5760a43560ff8116810361000e576107c89360e4359360c43593608435926064359261193c565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57600435600052600360205260408060002073ffffffffffffffffffffffffffffffffffffffff6001818354169201541682519182526020820152f35b503461000e57610e8b36610ace565b73ffffffffffffffffffffffffffffffffffffffff80600254163314801561102e575b610eb79061160e565b610ec18284611b89565b918184169181168214610f8457827f7b88fa12e0eaf025be34548ca7c53f68b4ac70c4d36664baf147c0d4226335f391610f5461046996610f1f610f03611787565b73ffffffffffffffffffffffffffffffffffffffff9092168252565b73ffffffffffffffffffffffffffffffffffffffff83166020820152610f4f846000526003602052604060002090565b611796565b6040805173ffffffffffffffffffffffffffffffffffffffff9092168252602082019290925290819081016106a0565b60a46040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152604960248201527f436f6c6c61746572616c546f6b656e3a2043616e206f6e6c792063726561746560448201527f206120636f6c6c61746572616c20746f6b656e2077697468206469666665726560648201527f6e7420746f6b656e7300000000000000000000000000000000000000000000006084820152fd5b5060065481163314610eae565b503461000e5761112861104d366107ca565b92919073ffffffffffffffffffffffffffffffffffffffff93611075856002541633146115a9565b60405190611082826116f2565b6000958683526110c5846110b68773ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b90600052602052604060002090565b6110d08382546121bd565b905584169384876040517fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f62339180611116888b8360209093929193604081019481520152565b0390a43b61112d57505050151561226b565b604051f35b611190917fffffffff000000000000000000000000000000000000000000000000000000009394602092886040518096819582947ff23a6e61000000000000000000000000000000000000000000000000000000009b8c8552336004860161268c565b03925af19081156111d9575b85916111ab575b50161461226b565b6111cc915060203d81116111d2575b6111c48183611746565b8101906121d5565b386111a3565b503d6111ba565b6111e1611eaf565b61119c565b503461000e57602060ff61124473ffffffffffffffffffffffffffffffffffffffff61121136610ace565b91166000526001845260406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b54166040519015158152f35b503461000e5760a07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043561128c816101ab565b602435611298816101ab565b6084359160443560643567ffffffffffffffff851161000e576112c26107c89536906004016106f1565b9173ffffffffffffffffffffffffffffffffffffffff9586861696873314801561144d575b6112f090612111565b61131e866110b68973ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b6113298482546121a6565b9055611359866110b68473ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b6113648482546121bd565b9055811680976040517fc3d58168c5ae7397731d063d5bbf3d657854427343f4c083240f7aacaa2d0f623391806113a9888c8360209093929193604081019481520152565b0390a4600094903b6113c257505050505050151561226b565b9560209291857fffffffff000000000000000000000000000000000000000000000000000000009798611427604051978896879586947ff23a6e61000000000000000000000000000000000000000000000000000000009e8f87523360048801612229565b03925af1918215611440575b916111ab5750161461226b565b611448611eaf565b611433565b506112f06114ae6114a7336114828b73ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b9073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b5460ff1690565b90506112e7565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576004356114f1816101ab565b73ffffffffffffffffffffffffffffffffffffffff611515816002541633146115a9565b811615611525576107c890611bd1565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b156115b057565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b1561161557565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152603660248201527f436f6c6c61746572616c546f6b656e3a2063616c6c6572206973206e6f74206f60448201527f776e6572206f72204f7074696f6e73466163746f7279000000000000000000006064820152fd5b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040810190811067ffffffffffffffff8211176116e557604052565b6116ed611699565b604052565b6020810190811067ffffffffffffffff8211176116e557604052565b6060810190811067ffffffffffffffff8211176116e557604052565b60c0810190811067ffffffffffffffff8211176116e557604052565b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff8211176116e557604052565b60405190611794826116c9565b565b815181547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff9182161782556117949260019190602001511691019073ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055565b1561182d57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602160248201527f436f6c6c61746572616c546f6b656e3a206578706972656420646561646c696e60448201527f65000000000000000000000000000000000000000000000000000000000000006064820152fd5b156118b857565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f436f6c6c61746572616c546f6b656e3a20696e76616c6964207369676e61747560448201527f72650000000000000000000000000000000000000000000000000000000000006064820152fd5b949092939695919673ffffffffffffffffffffffffffffffffffffffff94858716978860005260046020526040600020548203611ae25787611a7b8a89611a74611482958f999a611ad29b7f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c319f611a6f908f9b611a678f91610d349f8f9190611a3b916119cb42831015611826565b60405195869460208601988991959493909260a09360c08401977f8733d126a676f1e83270eccfbe576f65af55d3ff784c4dc4884be48932f47c81855273ffffffffffffffffffffffffffffffffffffffff80921660208601521660408401521515606083015260808201520152565b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08101835282611746565b519020611f7f565b611c40565b16146118b1565b611aa58173ffffffffffffffffffffffffffffffffffffffff166000526004602052604060002090565b8054600101905573ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b60405194151585521692602090a3565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601e60248201527f436f6c6c61746572616c546f6b656e3a20696e76616c6964206e6f6e636500006044820152fd5b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000602082019260601b1682526000603482015260288152611b838161170e565b51902090565b6040519060208201927fffffffffffffffffffffffffffffffffffffffff000000000000000000000000809260601b16845260601b16603482015260288152611b838161170e565b6002549073ffffffffffffffffffffffffffffffffffffffff80911691827fffffffffffffffffffffffff0000000000000000000000000000000000000000821617600255167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e06000604051a3565b916103dc9391611c4f93611ebc565b919091611c90565b60051115611c6157565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b611c9981611c57565b80611ca15750565b611caa81611c57565b60018103611d11576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606490fd5b611d1a81611c57565b60028103611d81576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606490fd5b611d8a81611c57565b60038103611e17576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c60448201527f75650000000000000000000000000000000000000000000000000000000000006064820152608490fd5b80611e23600492611c57565b14611e2a57565b6040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c60448201527f75650000000000000000000000000000000000000000000000000000000000006064820152608490fd5b506040513d6000823e3d90fd5b9291907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311611f735760ff16601b81141580611f68575b611f5c579160809493916020936040519384528484015260408301526060820152600093849182805260015afa15611f4f575b815173ffffffffffffffffffffffffffffffffffffffff811615611f49579190565b50600190565b611f57611eaf565b611f27565b50505050600090600490565b50601c811415611ef4565b50505050600090600390565b611f87611ff0565b906040519060208201927f1901000000000000000000000000000000000000000000000000000000000000845260228301526042820152604281526080810181811067ffffffffffffffff821117611fe3575b60405251902090565b611feb611699565b611fda565b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000163014806120e8575b15612058577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f000000000000000000000000000000000000000000000000000000000000000082527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a08152611b838161172a565b507f0000000000000000000000000000000000000000000000000000000000000000461461202f565b1561211857565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600e60248201527f4e4f545f415554484f52495a45440000000000000000000000000000000000006044820152fd5b507f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b8181106121b1570390565b6121b9612176565b0390565b811981116121c9570190565b6121d1612176565b0190565b9081602091031261000e57516103dc81610241565b601f82602094937fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0938186528686013760008582860101520116010190565b91926103dc96949160a09473ffffffffffffffffffffffffffffffffffffffff80921685521660208401526040830152606082015281608082015201916121ea565b1561227257565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601060248201527f554e534146455f524543495049454e54000000000000000000000000000000006044820152fd5b156122d757565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600f60248201527f4c454e4754485f4d49534d4154434800000000000000000000000000000000006044820152fd5b507f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b9190811015612376575b60051b0190565b61237e612335565b61236f565b90918281527f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831161000e5760209260051b80928483013701016000815290565b92906123dd906103dc9593604086526040860191612383565b926020818503910152612383565b96949261243b9461242d926103dc9a989473ffffffffffffffffffffffffffffffffffffffff8092168b521660208a015260a060408a015260a0890191612383565b918683036060880152612383565b9260808185039101526121ea565b9095939291949661245b8884146122d0565b73ffffffffffffffffffffffffffffffffffffffff9687831697883314801561260d575b61248890612111565b8760005b8b87808310612575575050505081168098887f4a39dc06d4c0dbc64b70af90fd698a233a518aa5d07e595d983b8c0526c8f7fb6124d38d6040519182918c8c3397856123c4565b0390a4600095903b6124f257505050505050506117949150151561226b565b6040979596975198899586957fbc197c81000000000000000000000000000000000000000000000000000000009a8b885233966004890197612533986123eb565b0381845a94602095f1611794937fffffffff0000000000000000000000000000000000000000000000000000000092821561144057916111ab5750161461226b565b6125fb6125948461258c8161260395600199612365565b35948d612365565b35926125c4816110b68c73ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b6125cf8582546121a6565b90556110b68873ffffffffffffffffffffffffffffffffffffffff166000526000602052604060002090565b9182546121bd565b905501889061248c565b506124886126426114a7336114828873ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b905061247f565b60209067ffffffffffffffff81116126625760051b0190565b61237e611699565b602091815181101561267f575b60051b010190565b612687612335565b612677565b909260a09273ffffffffffffffffffffffffffffffffffffffff6103dc96951683526000602084015260408301526060820152816080820152019061035c56fea2646970667358221220f099cd28067f8899fd000f20f055add35871b2ed4de644b577d8b9c725c0226a64736f6c634300080e0033" . parse () . expect ("invalid bytecode")
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
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address,uint256)` and selector `[0, 253, 213, 142]`"]
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
    #[doc = "Container type for all input parameters for the `balanceOfBatch`function with signature `balanceOfBatch(address[],uint256[])` and selector `[78, 18, 115, 244]`"]
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
    #[doc = "Container type for all input parameters for the `burnCollateralToken`function with signature `burnCollateralToken(address,uint256,uint256)` and selector `[62, 2, 145, 121]`"]
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
    #[doc = "Container type for all input parameters for the `createOptionCollateralToken`function with signature `createOptionCollateralToken(address)` and selector `[28, 130, 165, 89]`"]
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
    #[doc = "Container type for all input parameters for the `createSpreadCollateralToken`function with signature `createSpreadCollateralToken(address,address)` and selector `[225, 173, 146, 33]`"]
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
    #[doc = "Container type for all input parameters for the `getCollateralTokenId`function with signature `getCollateralTokenId(address,address)` and selector `[110, 209, 255, 109]`"]
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
    #[doc = "Container type for all input parameters for the `idToInfo`function with signature `idToInfo(uint256)` and selector `[217, 74, 126, 83]`"]
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
    #[doc = "Container type for all input parameters for the `isApprovedForAll`function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
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
    #[doc = "Container type for all input parameters for the `metaSetApprovalForAll`function with signature `metaSetApprovalForAll(address,address,bool,uint256,uint256,uint8,bytes32,bytes32)` and selector `[180, 230, 10, 50]`"]
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
    #[doc = "Container type for all input parameters for the `mintCollateralToken`function with signature `mintCollateralToken(address,uint256,uint256)` and selector `[227, 127, 90, 199]`"]
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
    #[doc = "Container type for all input parameters for the `nonces`function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
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
    #[doc = "Container type for all input parameters for the `safeBatchTransferFrom`function with signature `safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)` and selector `[46, 178, 194, 214]`"]
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
    #[doc = "Container type for all input parameters for the `safeTransferFrom`function with signature `safeTransferFrom(address,address,uint256,uint256,bytes)` and selector `[242, 66, 67, 42]`"]
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
    #[doc = "Container type for all input parameters for the `setApprovalForAll`function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
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
    #[doc = "Container type for all input parameters for the `setOptionsFactory`function with signature `setOptionsFactory(address)` and selector `[71, 169, 93, 72]`"]
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
    #[doc = "Container type for all input parameters for the `supportsInterface`function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
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
    #[doc = "Container type for all input parameters for the `uri`function with signature `uri(uint256)` and selector `[14, 137, 52, 28]`"]
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
}
