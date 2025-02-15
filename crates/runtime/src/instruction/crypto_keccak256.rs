use crate::RuntimeContext;
use fluentbase_types::IJournaledTrie;
use rwasm::{core::Trap, Caller};

pub struct CryptoKeccak256;

impl CryptoKeccak256 {
    pub fn fn_handler<DB: IJournaledTrie>(
        mut caller: Caller<'_, RuntimeContext<DB>>,
        data_offset: u32,
        data_len: u32,
        output_offset: u32,
    ) -> Result<(), Trap> {
        let data = caller.read_memory(data_offset, data_len)?;
        caller.write_memory(output_offset, &Self::fn_impl(data))?;
        Ok(())
    }

    pub fn fn_impl(data: &[u8]) -> [u8; 32] {
        let mut result = [0u8; 32];
        keccak_hash::write_keccak(data, &mut result);
        result
    }
}
