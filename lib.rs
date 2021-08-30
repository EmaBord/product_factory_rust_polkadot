#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod product {

    use ink_storage::{
        collections::{
            Vec as StorageVec,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };

    #[derive(Copy, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct Product{
        state: u8,
        code: u16,
        owner:AccountId, 
        delegate_to:Option<AccountId>,
    }

    impl Product {
            pub fn new(
                state: u8,
                code: u16,
                owner: AccountId,
            ) -> Product{
                Product { 
                    state: state,
                    code: code,
                    owner:owner,
                    delegate_to:None
                } 
            }
        }

    impl Product {
            pub fn get_owner(&mut self) -> AccountId{
                self.owner    
            } 
    }

    impl Product {
            pub fn get_code(&mut self) -> u16{
                self.code
            } 
    }

    impl Product {
            pub fn get(& self) -> Product{
                Product{
                    code:self.code,
                    state:self.state,
                    owner:self.owner,
                    delegate_to:self.delegate_to
                }    
            } 
    }

    impl Product {
            pub fn get_delegate(&mut self) -> Option<AccountId>{
                self.delegate_to    
            } 
    }

    impl Product {
            pub fn get_state(&mut self) -> u8{
                self.state 
            } 
    }

    impl Product {
            pub fn delegate_to(&mut self, delegate: AccountId){
                self.state = 1;
                self.delegate_to = Some(delegate);
            } 
    }

    impl Product {
            pub fn accept(&mut self, delegate: AccountId){
                self.state = 0;
                self.owner = delegate;
                self.delegate_to = None;
            } 
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        PidNotExists,
        InvalidOwner,
        InvalidDelegate,
        InvalidState,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct ProductFactory{
        products: StorageVec<Product>,
    }

    impl ProductFactory {
        #[ink(constructor)]
        pub fn new()->  Self{
            Self{
                products:StorageVec::<Product>::new(),
            }        

        }
        
    
        #[ink(message)]
        pub fn create_product(&mut self, code: u16){
            let p = Product::new(
                0,
                code,
                Self::env().caller(),
            );
            self.products.push(p);

        }

        #[ink(message)]
        pub fn get_last(&mut self) ->  Product{
            self.products[self.products.len()-1]
        }


        #[ink(message)]
        pub fn delegate_product(&mut self, pid: u32, delegate_to: AccountId) -> Result<()>{
            if pid >= self.products.len(){
                return Err(Error::PidNotExists)
            }

            let  p = &mut self.products[pid];
            if p.get_owner() != Self::env().caller(){
                return Err(Error::InvalidOwner)
            }
            if p.get_state() != 0{
                return Err(Error::InvalidState)
            }
            p.delegate_to(delegate_to);
            Ok(())        
            

        }

        #[ink(message)]
        pub fn accept_product(&mut self, pid: u32) -> Result<()>{
            if pid >= self.products.len(){
                return Err(Error::PidNotExists)
            }

            let  p = &mut self.products[pid];
            if p.get_delegate() != Some(Self::env().caller()){
                return Err(Error::InvalidDelegate)
            }
            if p.get_state() != 1{
                return Err(Error::InvalidState)
            }
            p.accept(Self::env().caller());
            Ok(())        
            

        }

            
    }
    
/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
/// module and test functions are marked with a `#[test]` attribute.
/// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        use ink_env::{
            call,
            test,
        };

        #[ink::test]
        fn create_product_test() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut product_factory = ProductFactory::new();
            assert_eq!(product_factory.products.len(), 0);
            product_factory.create_product(1);

            assert_eq!(product_factory.get_last().owner, accounts.alice);
            assert_eq!(product_factory.get_last().state, 0);
        }

        #[ink::test]
        fn delegate_product_test() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut product_factory = ProductFactory::new();
            assert_eq!(product_factory.products.len(), 0);
            product_factory.create_product(1);

            assert_eq!(product_factory.get_last().owner, accounts.alice);
            assert_eq!(product_factory.get_last().state, 0);

            product_factory.delegate_product(0,accounts.bob);
            assert_eq!(product_factory.get_last().owner, accounts.alice);
            assert_eq!(product_factory.get_last().state, 1);
            assert_eq!(product_factory.get_last().get_delegate(), Some(accounts.bob));

            assert_eq!(
                product_factory.delegate_product(1,accounts.bob), 
                Err(Error::PidNotExists)
            );

            assert_eq!(
                product_factory.delegate_product(0,accounts.bob), 
                Err(Error::InvalidState)
            );

            set_sender(accounts.bob);
            assert_eq!(
                product_factory.delegate_product(0,accounts.bob), 
                Err(Error::InvalidOwner)
            );


        }

        #[ink::test]
        fn accept_product_test() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut product_factory = ProductFactory::new();
            assert_eq!(product_factory.products.len(), 0);
            product_factory.create_product(1);

            assert_eq!(product_factory.get_last().owner, accounts.alice);
            assert_eq!(product_factory.get_last().state, 0);

            product_factory.delegate_product(0,accounts.bob);
            assert_eq!(product_factory.get_last().owner, accounts.alice);
            assert_eq!(product_factory.get_last().state, 1);
            assert_eq!(product_factory.get_last().get_delegate(), Some(accounts.bob));
            
            set_sender(accounts.bob);
            product_factory.accept_product(0);

            assert_eq!(product_factory.get_last().owner, accounts.bob);
            assert_eq!(product_factory.get_last().state, 0);
            assert_eq!(product_factory.get_last().get_delegate(), None);


            assert_eq!(
                product_factory.accept_product(0),
                Err(Error::InvalidDelegate));

        }

        fn set_sender(sender: AccountId) {
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or_else(|_| [0x0; 32].into());
            test::push_execution_context::<Environment>(
                sender,
                callee,
                1000000,
                1000000,
                test::CallData::new(call::Selector::new([0x00; 4])), // dummy
            );
        }

        
    }
}
