#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod callee {

    #[ink(storage)]
    pub struct Callee {}

    impl Callee {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn contract_error(&mut self) -> Result<(), ()> {
            Err(())
        }

        #[ink(message)]
        pub fn environment_error(&mut self) {
            panic!("ContractTrapped")
        }
    }
}
