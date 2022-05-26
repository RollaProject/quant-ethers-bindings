#[doc = "`ActionArgs(uint8,address,address,address,uint256,uint256,bytes)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ActionArgs {
    pub action_type: u8,
    pub q_token: ethers::core::types::Address,
    pub secondary_address: ethers::core::types::Address,
    pub receiver: ethers::core::types::Address,
    pub amount: ethers::core::types::U256,
    pub secondary_uint: ethers::core::types::U256,
    pub data: ethers::core::types::Bytes,
}
#[doc = "`PriceWithDecimals(uint256,uint8)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct PriceWithDecimals {
    pub price: ethers::core::types::U256,
    pub decimals: u8,
}
#[doc = "`MetaAction(uint256,uint256,address,(uint8,address,address,address,uint256,uint256,bytes)[])`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct MetaAction {
    pub nonce: ethers::core::types::U256,
    pub deadline: ethers::core::types::U256,
    pub from: ethers::core::types::Address,
    pub actions: ::std::vec::Vec<ActionArgs>,
}
