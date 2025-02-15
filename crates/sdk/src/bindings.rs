#[link(wasm_import_module = "fluentbase_v1alpha")]
extern "C" {
    /// Functions that provide access to crypto elements, right now we support following:
    /// - Keccak256
    /// - Poseidon (two modes, message hash and two elements hash)
    /// - Ecrecover
    pub fn _crypto_keccak256(data_offset: *const u8, data_len: u32, output32_offset: *mut u8);
    pub fn _crypto_poseidon(data_offset: *const u8, data_len: u32, output32_offset: *mut u8);
    pub fn _crypto_poseidon2(
        fa32_offset: *const u8,
        fb32_offset: *const u8,
        fd32_offset: *const u8,
        output32_offset: *mut u8,
    );
    pub fn _crypto_ecrecover(
        digest32_offset: *const u8,
        sig64_offset: *const u8,
        output65_offset: *mut u8,
        rec_id: u32,
    );

    /// Basic system methods that are available for every app (shared and sovereign)
    pub fn _sys_halt(code: i32) -> !;
    pub fn _sys_write(offset: *const u8, length: u32);
    pub fn _sys_input_size() -> u32;
    pub fn _sys_read(target: *mut u8, offset: u32, length: u32);
    pub fn _sys_output_size() -> u32;
    pub fn _sys_read_output(target: *mut u8, offset: u32, length: u32);
    pub fn _sys_forward_output(offset: u32, len: u32);
    pub fn _sys_state() -> u32;
    pub fn _sys_exec_hash(
        code_hash32_offset: *const u8,
        input_offset: *const u8,
        input_len: u32,
        return_offset: *mut u8,
        return_len: u32,
        fuel_offset: *mut u32,
        state: u32,
    ) -> i32;
    pub fn _sys_fuel(delta: u64) -> u64;

    /// Journaled ZK Trie methods to work with blockchain state
    pub fn _jzkt_open(root32_ptr: *const u8);
    pub fn _jzkt_checkpoint() -> u64;
    pub fn _jzkt_get(
        key32_ptr: *const u8,
        field: u32,
        output32_ptr: *mut u8,
        committed: bool,
    ) -> bool;
    pub fn _jzkt_update(
        key32_ptr: *const u8,
        flags: u32,
        vals32_offset: *const [u8; 32],
        vals32_len: u32,
    );
    pub fn _jzkt_update_preimage(
        key32_ptr: *const u8,
        field: u32,
        preimage_ptr: *const u8,
        preimage_len: u32,
    ) -> bool;
    pub fn _jzkt_remove(key32_ptr: *const u8);
    pub fn _jzkt_compute_root(output32_ptr: *mut u8);
    pub fn _jzkt_emit_log(
        address20_ptr: *const u8,
        topics32s_ptr: *const [u8; 32],
        topics32s_len: u32,
        data_ptr: *const u8,
        data_len: u32,
    );
    pub fn _jzkt_commit(root32_ptr: *mut u8);
    pub fn _jzkt_rollback(checkpoint: u64);
    pub fn _jzkt_preimage_size(hash32_ptr: *const u8) -> u32;
    pub fn _jzkt_preimage_copy(hash32_ptr: *const u8, preimage_ptr: *mut u8);

    pub fn _wasm_to_rwasm_size(input_ptr: *const u8, input_len: u32) -> i32;
    pub fn _wasm_to_rwasm(
        input_ptr: *const u8,
        input_len: u32,
        output_ptr: *mut u8,
        output_len: u32,
    ) -> i32;
    pub fn _debug_log(msg_ptr: *const u8, msg_len: u32);
}
