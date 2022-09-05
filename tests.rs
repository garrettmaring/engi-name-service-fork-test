#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_env::{
    Hash,
    AccountId,
    DefaultEnvironment as Environment
};

pub use crate::dns::*;

fn default_accounts(
                   ) -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
    ink_env::test::default_accounts::<Environment>()
}

fn set_next_caller(caller: AccountId) {
    ink_env::test::set_caller::<Environment>(caller);
}

#[ink::test]
fn register_works() {
    let default_accounts = default_accounts();
    let name = Hash::from([0x99; 32]);

    set_next_caller(default_accounts.alice);
    let mut contract = EngiNameService::new();

    assert_eq!(contract.register(name), Ok(()));
    assert_eq!(contract.register(name), Err(Error::NameAlreadyExists));
}

#[ink::test]
fn set_address_works() {
    let accounts = default_accounts();
    let name = Hash::from([0x99; 32]);

    set_next_caller(accounts.alice);

    let mut contract = EngiNameService::new();
    assert_eq!(contract.register(name), Ok(()));

    // Caller is not owner, `set_address` should fail.
    set_next_caller(accounts.bob);
    assert_eq!(
        contract.set_address(name, accounts.bob),
        Err(Error::CallerIsNotOwner)
        );

    // Caller is owner, set_address will be successful
    set_next_caller(accounts.alice);
    assert_eq!(contract.set_address(name, accounts.bob), Ok(()));
    assert_eq!(contract.get_address(name), accounts.bob);
}

#[ink::test]
fn transfer_works() {
    let accounts = default_accounts();
    let name = Hash::from([0x99; 32]);

    set_next_caller(accounts.alice);

    let mut contract = EngiNameService::new();
    assert_eq!(contract.register(name), Ok(()));

    // Test transfer of owner.
    assert_eq!(contract.transfer(name, accounts.bob), Ok(()));

    // Owner is bob, alice `set_address` should fail.
    assert_eq!(
        contract.set_address(name, accounts.bob),
        Err(Error::CallerIsNotOwner)
        );

    set_next_caller(accounts.bob);
    // Now owner is bob, `set_address` should be successful.
    assert_eq!(contract.set_address(name, accounts.bob), Ok(()));
    assert_eq!(contract.get_address(name), accounts.bob);
}
