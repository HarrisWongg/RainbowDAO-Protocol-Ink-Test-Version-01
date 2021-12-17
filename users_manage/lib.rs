#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod users_manage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };
    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct User {
        id:u128,
        nickname:String,
        profile:String,
        code:u128,
        address:AccountId,
        referer:AccountId,
        childs:Vec<AccountId>
    }


    #[ink(storage)]
    pub struct UsersManage {
        user_info:StorageHashMap<AccountId,User>,
        // user_referer:StorageHashMap<AccountId,AccountId>,
        code_user:StorageHashMap<u128, AccountId>,
        length:u128
    }

    impl UsersManage {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                user_info:StorageHashMap::new(),
                code_user:StorageHashMap::new(),
                length:0,
            }
        }
        #[ink(message)]
        pub fn join(&mut self,invitation_code:u128,name:String,user_profile:String) -> bool {
            assert_eq!(self.length + 1 > self.length, true);
            let caller = Self::env().caller();
            //let user = self.user_info.get(&caller).unwrap().clone();
            assert_eq!(self.exists_user(caller),false);
            // let code = self.create_code();
            let code =  self.length + 1;

            self.code_user.insert(code,caller);
            let referer = if invitation_code == 0 { AccountId::default()} else { self.get_user_by_code(invitation_code) };
            let nickname = if name.is_empty() { String::default()} else {name };
            let profile = if user_profile.is_empty() { String::default()} else {user_profile };
            self.user_info.insert(caller, User{id:self.length + 1,nickname,profile,code,address:caller,referer,childs:Vec::new()});
            self.length += 1;
            if referer != AccountId::default() {
                self.insert_user_child(referer,caller);
            }
            true
        }
        #[ink(message)]
        pub fn get_user_referer(&self,user:AccountId) -> AccountId {
            let user_info : User =  self.user_info.get(&user).unwrap().clone();
            return user_info.referer;
        }
        #[ink(message)]
        pub fn exists_user(&self,user:AccountId) -> bool {
            let user_info = self.user_info.get(&user).unwrap().clone();
            return user_info.id != 0 ;
        }

        #[ink(message)]
        pub fn get_user_by_code(&self,invitation_code:u128) -> AccountId {
            self.code_user.get(&invitation_code).unwrap().clone()
        }
        #[ink(message)]
        pub fn list_user(&self) -> Vec<User> {
            let mut user_vec = Vec::new();
            let mut iter = self.user_info.values();
            let mut user = iter.next();
            while user.is_some() {
                user_vec.push(user.unwrap().clone());
                user = iter.next();
            }
            user_vec
        }
        #[ink(message)]
        pub fn insert_user_child(&mut self,user:AccountId,child:AccountId) -> bool {
            let mut user_info = self.user_info.get_mut(&user).unwrap().clone();
            user_info.childs.push(child);
            true
        }
    }
}