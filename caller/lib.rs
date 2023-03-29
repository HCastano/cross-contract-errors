#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod caller {
    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };

    use ink::env::Result as EnvResult;
    use ink::MessageResult;
    type ContractResult = core::result::Result<(), ()>;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Caller {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Caller {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn contract_error(&mut self, address: AccountId) {
            let result = build_call::<DefaultEnvironment>()
                .call(address)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "contract_error"
                ))))
                .returns::<Result<(), ()>>()
                .try_invoke();

            match result {
                EnvResult::Ok(MessageResult::Ok(ContractResult::Err(_))) => {
                    self.value = !self.value;
                }
                _ => unimplemented!(),
            }
        }

        #[ink(message)]
        pub fn lang_error(&mut self, address: AccountId) {
            let result = build_call::<DefaultEnvironment>()
                .call(address)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "not_a_real_selector"
                ))))
                .returns::<()>()
                .try_invoke();

            // Note: You can match on more specific `LangError`s here.
            //
            // See [`ink::LangError`] for more.
            match result {
                EnvResult::Ok(MessageResult::Err(_)) => {
                    self.value = !self.value;
                }
                _ => unimplemented!(),
            }
        }

        #[ink(message)]
        pub fn environment_error(&mut self, address: AccountId) {
            let result = build_call::<DefaultEnvironment>()
                .call(address)
                .exec_input(ExecutionInput::new(Selector::new(ink::selector_bytes!(
                    "environment_error"
                ))))
                .returns::<()>()
                .try_invoke();

            // Note: You can match on more specific `EnvError`s here.
            //
            // See [`ink::env::Error`] for more.
            match result {
                EnvResult::Err(_) => {
                    self.value = !self.value;
                }
                _ => unimplemented!()
            }
        }
    }
}
