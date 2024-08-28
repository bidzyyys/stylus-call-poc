#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloy_primitives::{Address, FixedBytes};
use alloy_sol_types::sol;
use stylus_sdk::{
    abi::Bytes,
    call::Call,
    console, msg,
    prelude::{entrypoint, external, sol_interface, sol_storage, SolidityError},
};

sol! {
    #[allow(missing_docs)]
    event CallBytes(bytes data);
}

sol! {
    #[derive(Debug)]
    #[allow(missing_docs)]
    error ReceiverError();
}

#[derive(SolidityError, Debug)]
pub enum Error {
    RecError(ReceiverError),
}

sol_interface! {
    interface SolidityReceiver {
        #[allow(missing_docs)]
        function handle(
            address operator,
            address from,
            bytes calldata data
        ) external returns (bytes4);
    }
}

sol_storage! {
    #[entrypoint]
    pub struct CallPoC {
        uint8 value;
    }
}

#[external]
impl CallPoC {
    pub fn dummy(
        &mut self,
        receiver: Address,
        to: Address,
        data: Bytes,
    ) -> Result<FixedBytes<4>, Error> {
        let receiver = SolidityReceiver::new(receiver);
        let call = Call::new_in(self);
        let from = msg::sender();
        let data = data.to_vec().into();
        console!("Calldata : {:x?}", data);
        let result = receiver.handle(call, from, to, data);
        match result {
            Ok(retval) => {
                console!("It works: {:?}", retval);
                Ok(retval)
            }
            Err(e) => {
                console!("Error: {:?}", e);
                Err(Error::RecError(ReceiverError {}))
            }
        }
    }
}
