#![cfg_attr(not(feature = "std"), no_std)]

use ink::utils::*;
use ink_lang as ink;
use ink_prelude::string::String;
use ink_storage::traits::SpreadAllocate;
use ink_storage::Mapping;
use more_asserts as ma;
use scale_info::*;
#[ink::contract]
mod ecc20 {
    pub use super::*;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Ecc20 {
        /// Stores a single `bool` value on the storage.
        total_value: Balance,
        balances: Mapping<AccountId, Balance>,
        name: String,
        symbol: String,
    }

    // ----------------------------------------------Event----------------------------------------------
    #[ink(event)]
    pub struct Transferred {
        from: Option<AccountId>,
        to: Option<AccountId>,
        amount: Balance,
    }

    impl Ecc20 {
        // ----------------------------------------------Constructor----------------------------------------------
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(_name: String, _symbol: String, init_value: Balance) -> Self {
            // Self {
            //     total_value: init_value
            // }
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.balances.insert(&caller, &init_value);
                contract.name = String::from(_name);
                contract.symbol = String::from(_symbol);
                contract.total_value = init_value;
                Self::env().emit_event(Transferred {
                    from: None,
                    to: Some(caller),
                    amount: init_value,
                })
            })
        }

        // fn new_init(&mut self, )

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            // Self::new(Default::default())
            initialize_contract(|_| {})
        }

        // ----------------------------------------------Action----------------------------------------------
        // Return balance of AccountId
        #[ink(message)]
        pub fn balanceOf(&self, account: AccountId) -> Balance {
            let account_balance = self.balance_of_impl(&account);
            account_balance
        }
        #[inline]
        fn balance_of_impl(&self, account: &AccountId) -> Balance {
            self.balances.get(account).unwrap_or_default()
        }

        #[ink(message)]
        pub fn totalSupply(&self) -> Result<u128, ()> {
            Ok(self.total_value)
        }

        #[ink(message)]
        pub fn getName(&self) -> Result<String, ()> {
            Ok(self.name.clone())
        }

        #[ink(message)]
        pub fn transfer(&mut self, recipient: AccountId, amount: Balance) -> Result<(), ()> {
            let caller = Self::env().caller();

            let mut own_balance = self.balanceOf(caller);
            ma::assert_gt!(own_balance, amount, "Not enough balance");

            own_balance = own_balance - amount;
            self.balances.insert(caller, &own_balance);
            let mut recipient_balance = self.balanceOf(recipient);
            recipient_balance += amount;
            self.balances.insert(recipient, &recipient_balance);

            self.env().emit_event(Transferred {
                from: Some(caller),
                to: Some(recipient),
                amount: amount,
            });

            Ok(())
        }
    }

    // ----------------------------------------------ErrorCode----------------------------------------------
    #[derive(scale::Encode, scale::Decode, TypeInfo)]
    pub enum ContractErrorCode {
        NotEnoughBalance,
    }
    // / Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // / module and test functions are marked with a `#[test]` attribute.
    // / The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let ecc20 = Ecc20::default();
    //         assert_eq!(ecc20.get(), false);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut ecc20 = Ecc20::new(false);
    //         assert_eq!(ecc20.get(), false);
    //         ecc20.flip();
    //         assert_eq!(ecc20.get(), true);
    //     }
    // }
}
