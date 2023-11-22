use crate::{
    cairo_types::{Error, Result as CairoResult},
    CairoType,
};
use starknet::core::types::{BlockId, BlockTag, FunctionCall};
use std::marker::PhantomData;

#[derive(Debug)]
pub enum TransactionStatus {
    Succeeded,
    Pending,
    Reverted(String),
    Error(String),
}

#[derive(Debug)]
pub struct FCall<'p, P, T> {
    pub call_raw: FunctionCall,
    pub block_id: BlockId,
    provider: &'p P,
    rust_type: PhantomData<T>,
}

impl<'p, P, T> FCall<'p, P, T>
where
    P: starknet::providers::Provider + Sync,
    T: CairoType<RustType = T>,
{
    pub fn new(call_raw: FunctionCall, provider: &'p P) -> Self {
        Self {
            call_raw,
            block_id: BlockId::Tag(BlockTag::Pending),
            provider,
            rust_type: PhantomData,
        }
    }

    pub fn provider(self) -> &'p P {
        self.provider
    }

    pub fn block_id(mut self, block_id: BlockId) -> Self {
        self.block_id = block_id;
        self
    }

    pub async fn call(self) -> CairoResult<T> {
        let r = self
            .provider
            .call(self.call_raw, self.block_id)
            .await
            .map_err(|err| Error::Deserialize(format!("Deserialization error {}", err)))?;

        T::deserialize(&r, 0)
    }
}
