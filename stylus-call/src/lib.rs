#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_sol_types::sol;
use stylus_sdk::prelude::{entrypoint, external, sol_storage};

sol! {
    #[allow(missing_docs)]
    event CallBytes(bytes data);
}

sol_storage! {
    #[entrypoint]
    pub struct CallPoC {
        bool initialized;
    }
}

#[external]
impl CallPoC {
    pub fn easy(&self) -> u8 {
        11
    }
}
