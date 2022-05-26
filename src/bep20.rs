pub use bep20_mod::*;
#[allow(clippy::too_many_arguments)]
mod bep20_mod {
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
    #[doc = "BEP20 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BEP20_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialSupply\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BEP20_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052346200009a5762001be5803803806200001d81620000b6565b92833981016080828203126200009a5781516001600160401b03908181116200009a57826200004e918501620000eb565b9160208401519182116200009a5762000069918401620000eb565b60408301519060ff821682036200009a5760606200008a9401519262000372565b6040516116c49081620005218239f35b600080fd5b50634e487b7160e01b600052604160045260246000fd5b6040519190601f01601f191682016001600160401b03811183821017620000dc57604052565b620000e66200009f565b604052565b81601f820112156200009a578051906001600160401b0382116200016f575b60209062000121601f8401601f19168301620000b6565b938385528284830101116200009a5782906000905b8383831062000156575050116200014c57505090565b6000918301015290565b8193508281939201015182828801015201839162000136565b620001796200009f565b6200010a565b90600182811c92168015620001b1575b60208310146200019b57565b634e487b7160e01b600052602260045260246000fd5b91607f16916200018f565b601f8111620001c9575050565b6000906005825260208220906020601f850160051c830194106200020a575b601f0160051c01915b828110620001fe57505050565b818155600101620001f1565b9092508290620001e8565b601f811162000222575050565b6000906006825260208220906020601f850160051c8301941062000263575b601f0160051c01915b8281106200025757505050565b8181556001016200024a565b909250829062000241565b80519091906001600160401b03811162000362575b6200029b81620002956005546200017f565b620001bc565b602080601f8311600114620002da5750819293600092620002ce575b50508160011b916000199060031b1c191617600555565b015190503880620002b7565b6005600052601f198316949091907f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db0926000905b878210620003495750508360019596106200032f575b505050811b01600555565b015160001960f88460031b161c1916905538808062000324565b806001859682949686015181550195019301906200030e565b6200036c6200009f565b62000283565b91906000923360018060a01b031985541617845533847f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e081604051a38051906001600160401b03821162000510575b620003d982620003d36006546200017f565b62000215565b60209081601f84116001146200047a575091806200042b9594926200041b9488926200046e575b50508160011b916000199060031b1c1916176006556200026e565b60ff1660ff196004541617600455565b33600081815260016020908152604091829020859055905193845290927fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9190a3565b01519050388062000400565b60066000529190601f1984167ff652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d3f9388905b828210620004f75750509260019285926200041b966200042b99989610620004dd575b505050811b016006556200026e565b015160001960f88460031b161c19169055388080620004ce565b80600186978294978701518155019601940190620004ab565b6200051a6200009f565b620003c156fe60806040526004361015610013575b600080fd5b60003560e01c806306fdde0314610191578063095ea7b31461018857806318160ddd1461017f57806323b872dd14610176578063313ce5671461016d57806332424aa31461016d578063395093511461016457806342966c681461015b57806370a0823114610152578063715018a614610149578063893d20e8146101405780638da5cb5b1461014057806395d89b4114610137578063a0712d681461012e578063a457c2d714610125578063a9059cbb1461011c578063b09f126614610113578063d28d88521461010a578063dd62ed3e146101015763f2fde38b146100f957600080fd5b61000e610ef7565b5061000e610e61565b5061000e610d7f565b5061000e610c9d565b5061000e610bcc565b5061000e610af0565b5061000e6109b2565b5061000e6108d0565b5061000e61087d565b5061000e6107d8565b5061000e610772565b5061000e610612565b5061000e61058d565b5061000e61054d565b5061000e610440565b5061000e610403565b5061000e6103b3565b5061000e610210565b919091602080825283519081818401526000945b8286106101fa575050601f817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe09260409596116101ed575b0116010190565b60008582860101526101e6565b85810182015184870160400152948101946101ae565b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261036a5760405190806006549060019180831c92808216928315610360575b60209283861085146103335785885260208801949081156102f957506001146102a0575b61029c8761029081890382610c5c565b6040519182918261019a565b0390f35b600660005294509192917ff652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d3f5b8386106102e857505050910190506102908261029c3880610280565b8054858701529482019481016102cc565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016855250505001915061029090508261029c3880610280565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526022600452fd5b93607f169361025c565b80fd5b6004359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b6024359073ffffffffffffffffffffffffffffffffffffffff8216820361000e57565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576103f86103ee61036d565b60243590336114e5565b602060405160018152f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020600354604051908152f35b503461000e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576103f861047b61036d565b610545610486610390565b6104946044358092856112ae565b73ffffffffffffffffffffffffffffffffffffffff831660005260026020526104e13360406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b5490604051916104f083610c11565b602883527f42455032303a207472616e7366657220616d6f756e742065786365656473206160208401527f6c6c6f77616e636500000000000000000000000000000000000000000000000060408401526110e8565b9033906114e5565b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602060ff60045416604051908152f35b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576103f86105c861036d565b33600052600260205261060b6024356106058360406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b54611143565b90336114e5565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5760043533156106ee5760009061068b61067b3373ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b5482610685611430565b916110e8565b336000908152600160205260409020556106af6106aa8260035461148f565b600355565b60405190815233907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9080602081015b0390a360405160018152602090f35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602160248201527f42455032303a206275726e2066726f6d20746865207a65726f2061646472657360448201527f73000000000000000000000000000000000000000000000000000000000000006064820152fd5b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e5773ffffffffffffffffffffffffffffffffffffffff6107bf61036d565b1660005260016020526020604060002054604051908152f35b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261036a5780547fffffffffffffffffffffffff000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff82169161084f338414611053565b83604051937f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08286a3168255f35b503461000e5760007ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57602073ffffffffffffffffffffffffffffffffffffffff60005416604051908152f35b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261036a5760405190806005549060019180831c928082169283156109a8575b60209283861085146103335785885260208801949081156102f9575060011461094f5761029c8761029081890382610c5c565b600560005294509192917f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db05b83861061099757505050910190506102908261029c3880610280565b80548587015294820194810161097b565b93607f169361091c565b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57600435610a0873ffffffffffffffffffffffffffffffffffffffff600054163314611053565b3315610a9257610a1d6106aa82600354611143565b610a4b816106053373ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b3360009081526001602052604090205560007fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef604051806106df3395829190602083019252565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601f60248201527f42455032303a206d696e7420746f20746865207a65726f2061646472657373006044820152fd5b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576103f8610b2b61036d565b33600052600260205261060b610b658260406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b5460405190610b7382610c11565b602582527f42455032303a2064656372656173656420616c6c6f77616e63652062656c6f7760208301527f207a65726f0000000000000000000000000000000000000000000000000000006040830152602435906110e8565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576103f8610c0761036d565b60243590336112ae565b6060810190811067ffffffffffffffff821117610c2d57604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610c2d57604052565b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261036a5760405190806005549060019180831c92808216928315610d75575b60209283861085146103335785885260208801949081156102f95750600114610d1c5761029c8761029081890382610c5c565b600560005294509192917f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db05b838610610d6457505050910190506102908261029c3880610280565b805485870152948201948101610d48565b93607f1693610ce9565b503461000e576000807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261036a5760405190806006549060019180831c92808216928315610e57575b60209283861085146103335785885260208801949081156102f95750600114610dfe5761029c8761029081890382610c5c565b600660005294509192917ff652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d3f5b838610610e4657505050910190506102908261029c3880610280565b805485870152948201948101610e2a565b93607f1693610dcb565b503461000e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e576020610eee610e9e61036d565b73ffffffffffffffffffffffffffffffffffffffff610ebb610390565b91166000526002835260406000209073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b54604051908152f35b503461000e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261000e57610f2f61036d565b73ffffffffffffffffffffffffffffffffffffffff806000541690610f55338314611053565b82168015610fcf57600092610fcd91604051937f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08686a373ffffffffffffffffffffffffffffffffffffffff167fffffffffffffffffffffffff00000000000000000000000000000000000000006000541617600055565bf35b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f64647265737300000000000000000000000000000000000000000000000000006064820152fd5b1561105a57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b507f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b909181831161110957508181106110fd570390565b6111056110b8565b0390565b61113f906040519182917f08c379a00000000000000000000000000000000000000000000000000000000083526004830161019a565b0390fd5b90801982116111b7575b81019081106111595790565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601b60248201527f536166654d6174683a206164646974696f6e206f766572666c6f7700000000006044820152fd5b6111bf6110b8565b61114d565b156111cb57565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f42455032303a207472616e7366657220746f20746865207a65726f206164647260448201527f65737300000000000000000000000000000000000000000000000000000000006064820152fd5b6040519061125c82610c11565b602682527f616c616e636500000000000000000000000000000000000000000000000000006040837f42455032303a207472616e7366657220616d6f756e742065786365656473206260208201520152565b919073ffffffffffffffffffffffffffffffffffffffff8084169283156113ac57826113a7927fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9416956113038715156111c4565b61136461133d6113338373ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b548561068561124f565b9173ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b5561139661133d836106058473ffffffffffffffffffffffffffffffffffffffff166000526001602052604060002090565b556040519081529081906020820190565b0390a3565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602560248201527f42455032303a207472616e736665722066726f6d20746865207a65726f20616460448201527f64726573730000000000000000000000000000000000000000000000000000006064820152fd5b6040519061143d82610c11565b602282527f63650000000000000000000000000000000000000000000000000000000000006040837f42455032303a206275726e20616d6f756e7420657863656564732062616c616e60208201520152565b9060405190604082019282841067ffffffffffffffff851117610c2d576114e293604052601e83527f536166654d6174683a207375627472616374696f6e206f766572666c6f77000060208401526110e8565b90565b9073ffffffffffffffffffffffffffffffffffffffff9182811692831561160b57821693841561158757806113967f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925946115626113a79573ffffffffffffffffffffffffffffffffffffffff166000526002602052604060002090565b9073ffffffffffffffffffffffffffffffffffffffff16600052602052604060002090565b60846040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f42455032303a20617070726f766520746f20746865207a65726f20616464726560448201527f73730000000000000000000000000000000000000000000000000000000000006064820152fd5b60846040517f08c379a0000000000000000000000000000000000000000000000000000000008152602060048201526024808201527f42455032303a20617070726f76652066726f6d20746865207a65726f2061646460448201527f72657373000000000000000000000000000000000000000000000000000000006064820152fdfea26469706673582212201d829d8e81c8ef5f115364044023a3e2166801ffe013274dcbad3cba45afa30764736f6c634300080e0033" . parse () . expect ("invalid bytecode")
        });
    pub struct BEP20<M>(ethers::contract::Contract<M>);
    impl<M> Clone for BEP20<M> {
        fn clone(&self) -> Self {
            BEP20(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for BEP20<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BEP20<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BEP20))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> BEP20<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BEP20_ABI.clone(), client).into()
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
                BEP20_ABI.clone(),
                BEP20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_decimals` (0x32424aa3) function"]
        pub fn _decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([50, 66, 74, 163], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_name` (0xd28d8852) function"]
        pub fn _name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([210, 141, 136, 82], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_symbol` (0xb09f1266) function"]
        pub fn _symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([176, 159, 18, 102], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x42966c68) function"]
        pub fn burn(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([66, 150, 108, 104], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOwner` (0x893d20e8) function"]
        pub fn get_owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([137, 61, 32, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0xa0712d68) function"]
        pub fn mint(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([160, 113, 45, 104], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            sender: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
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
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, BEP20Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for BEP20<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BEP20Events {
        ApprovalFilter(ApprovalFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for BEP20Events {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(BEP20Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(BEP20Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(BEP20Events::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BEP20Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BEP20Events::ApprovalFilter(element) => element.fmt(f),
                BEP20Events::OwnershipTransferredFilter(element) => element.fmt(f),
                BEP20Events::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `_decimals`function with signature `_decimals()` and selector `[50, 66, 74, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_decimals", abi = "_decimals()")]
    pub struct _DecimalsCall;
    #[doc = "Container type for all input parameters for the `_name`function with signature `_name()` and selector `[210, 141, 136, 82]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_name", abi = "_name()")]
    pub struct _NameCall;
    #[doc = "Container type for all input parameters for the `_symbol`function with signature `_symbol()` and selector `[176, 159, 18, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_symbol", abi = "_symbol()")]
    pub struct _SymbolCall;
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(uint256)` and selector `[66, 150, 108, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `decreaseAllowance`function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getOwner`function with signature `getOwner()` and selector `[137, 61, 32, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getOwner", abi = "getOwner()")]
    pub struct GetOwnerCall;
    #[doc = "Container type for all input parameters for the `increaseAllowance`function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(uint256)` and selector `[160, 113, 45, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(uint256)")]
    pub struct MintCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub sender: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BEP20Calls {
        _Decimals(_DecimalsCall),
        _Name(_NameCall),
        _Symbol(_SymbolCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        GetOwner(GetOwnerCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Mint(MintCall),
        Name(NameCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for BEP20Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <_DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::_Decimals(decoded));
            }
            if let Ok(decoded) = <_NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::_Name(decoded));
            }
            if let Ok(decoded) =
                <_SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::_Symbol(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BEP20Calls::Burn(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <GetOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::GetOwner(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BEP20Calls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(BEP20Calls::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BEP20Calls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BEP20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                BEP20Calls::_Decimals(element) => element.encode(),
                BEP20Calls::_Name(element) => element.encode(),
                BEP20Calls::_Symbol(element) => element.encode(),
                BEP20Calls::Allowance(element) => element.encode(),
                BEP20Calls::Approve(element) => element.encode(),
                BEP20Calls::BalanceOf(element) => element.encode(),
                BEP20Calls::Burn(element) => element.encode(),
                BEP20Calls::Decimals(element) => element.encode(),
                BEP20Calls::DecreaseAllowance(element) => element.encode(),
                BEP20Calls::GetOwner(element) => element.encode(),
                BEP20Calls::IncreaseAllowance(element) => element.encode(),
                BEP20Calls::Mint(element) => element.encode(),
                BEP20Calls::Name(element) => element.encode(),
                BEP20Calls::Owner(element) => element.encode(),
                BEP20Calls::RenounceOwnership(element) => element.encode(),
                BEP20Calls::Symbol(element) => element.encode(),
                BEP20Calls::TotalSupply(element) => element.encode(),
                BEP20Calls::Transfer(element) => element.encode(),
                BEP20Calls::TransferFrom(element) => element.encode(),
                BEP20Calls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BEP20Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BEP20Calls::_Decimals(element) => element.fmt(f),
                BEP20Calls::_Name(element) => element.fmt(f),
                BEP20Calls::_Symbol(element) => element.fmt(f),
                BEP20Calls::Allowance(element) => element.fmt(f),
                BEP20Calls::Approve(element) => element.fmt(f),
                BEP20Calls::BalanceOf(element) => element.fmt(f),
                BEP20Calls::Burn(element) => element.fmt(f),
                BEP20Calls::Decimals(element) => element.fmt(f),
                BEP20Calls::DecreaseAllowance(element) => element.fmt(f),
                BEP20Calls::GetOwner(element) => element.fmt(f),
                BEP20Calls::IncreaseAllowance(element) => element.fmt(f),
                BEP20Calls::Mint(element) => element.fmt(f),
                BEP20Calls::Name(element) => element.fmt(f),
                BEP20Calls::Owner(element) => element.fmt(f),
                BEP20Calls::RenounceOwnership(element) => element.fmt(f),
                BEP20Calls::Symbol(element) => element.fmt(f),
                BEP20Calls::TotalSupply(element) => element.fmt(f),
                BEP20Calls::Transfer(element) => element.fmt(f),
                BEP20Calls::TransferFrom(element) => element.fmt(f),
                BEP20Calls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<_DecimalsCall> for BEP20Calls {
        fn from(var: _DecimalsCall) -> Self {
            BEP20Calls::_Decimals(var)
        }
    }
    impl ::std::convert::From<_NameCall> for BEP20Calls {
        fn from(var: _NameCall) -> Self {
            BEP20Calls::_Name(var)
        }
    }
    impl ::std::convert::From<_SymbolCall> for BEP20Calls {
        fn from(var: _SymbolCall) -> Self {
            BEP20Calls::_Symbol(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for BEP20Calls {
        fn from(var: AllowanceCall) -> Self {
            BEP20Calls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for BEP20Calls {
        fn from(var: ApproveCall) -> Self {
            BEP20Calls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for BEP20Calls {
        fn from(var: BalanceOfCall) -> Self {
            BEP20Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for BEP20Calls {
        fn from(var: BurnCall) -> Self {
            BEP20Calls::Burn(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for BEP20Calls {
        fn from(var: DecimalsCall) -> Self {
            BEP20Calls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for BEP20Calls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            BEP20Calls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<GetOwnerCall> for BEP20Calls {
        fn from(var: GetOwnerCall) -> Self {
            BEP20Calls::GetOwner(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for BEP20Calls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            BEP20Calls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<MintCall> for BEP20Calls {
        fn from(var: MintCall) -> Self {
            BEP20Calls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for BEP20Calls {
        fn from(var: NameCall) -> Self {
            BEP20Calls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for BEP20Calls {
        fn from(var: OwnerCall) -> Self {
            BEP20Calls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for BEP20Calls {
        fn from(var: RenounceOwnershipCall) -> Self {
            BEP20Calls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for BEP20Calls {
        fn from(var: SymbolCall) -> Self {
            BEP20Calls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for BEP20Calls {
        fn from(var: TotalSupplyCall) -> Self {
            BEP20Calls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for BEP20Calls {
        fn from(var: TransferCall) -> Self {
            BEP20Calls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for BEP20Calls {
        fn from(var: TransferFromCall) -> Self {
            BEP20Calls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for BEP20Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            BEP20Calls::TransferOwnership(var)
        }
    }
}
