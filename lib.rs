#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod nft_psp34_sample {

    use ink_prelude::{
        vec::Vec,
        string::{
            String,
            ToString,
        },
    };

    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        contracts::psp34::extensions::mintable::*,
        contracts::psp34::extensions::burnable::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: Data,
        initial_id:Id,
        next_token_id: u32,
    }

    impl PSP34 for Contract {}
    impl PSP34Metadata for Contract {}
    impl PSP34Mintable for Contract {}
    impl PSP34Burnable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(id: u32, name: String, symbol: String, base_uri: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let name_key: Vec<u8> = "name".as_bytes().to_vec();
                let symbol_key: Vec<u8> = "symbol".as_bytes().to_vec();
                let base_uri_key: Vec<u8> = "base_uri".as_bytes().to_vec();
                instance._set_attribute(Id::U32(id).clone(), name_key, name.as_bytes().to_vec());
                instance._set_attribute(Id::U32(id).clone(), symbol_key, symbol.as_bytes().to_vec());
                instance._set_attribute(Id::U32(id).clone(), base_uri_key, base_uri.as_bytes().to_vec());
                instance.initial_id = Id::U32(id);
            })
        }

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), Id::U32(self.next_token_id))?;
            self.next_token_id += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, id: u32) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), Id::U32(id));
                //Ok(()),
                //Err(e) => return Err(e),
            self.next_token_id += 1;
            Ok(())
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn mint_for_sale(&mut self, account:AccountId, id: u32) -> Result<(), PSP34Error> {
            let transfered_value = self.env().transferred_value();
            if transfered_value < 1000000000000000000 {
                return Err(PSP34Error::Custom("Insufficient mint price.".to_string()));
            }
            self._mint_to(account, Id::U32(id));
            self.next_token_id += 1;
            Ok(())
        }

        //#[ink(message)]
        //pub fn burn_token(&mut self, account:AccountId, id: u32) -> Result<(), PSP34Error> {
        //    PSP34Burnable::_burn(account, Id::U32(id))
        //}

        #[ink(message)]
        pub fn token_uri(&self, id: String) -> String {
            let base_uri_key: Vec<u8> = "base_uri".as_bytes().to_vec();
            let base_uri = match self.get_attribute(self.initial_id.clone(), base_uri_key) {
                Some(value) => value,
                None => return "".to_string(),
            };
            //String::from_utf8(base_uri.clone()).unwrap() + &self._get_id_string(Id::U32(token_id)) + ".json"
            let extention: &str = &".json".to_string();
            let id_str: &str = &id.to_string();
            String::from_utf8(base_uri.clone()).unwrap() + id_str + extention
        }

        #[ink(message)]
        pub fn get_id_string(&self, id: Id) -> String {
            self._get_id_string(id)
        }

        #[inline]
        fn _get_id_string(&self, id: Id) -> String {
            match id {
                Id::U8(id) => {
                    let tmp: u8 = id;
                    tmp.to_string()
                }
                Id::U16(id) => {
                    let tmp: u16 = id;
                    tmp.to_string()
                }
                Id::U32(id) => {
                    let tmp: u32 = id;
                    tmp.to_string()
                }
                Id::U64(id) => {
                    let tmp: u64 = id; 
                    tmp.to_string()
                }
                Id::U128(id) => {
                    let tmp: u128 = id;
                    tmp.to_string()
                }
                Id::Bytes(value) => String::from_utf8(value.clone()).unwrap(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn init_works() {
            let id: u32 = 0;
            let nft = Contract::new(0, String::from("KEY"), String::from("VAL"), String::from("VAL2"));

            assert_eq!(
                nft.get_attribute(id.clone(), String::from("KEY")),
                Some(String::from("VAL"))
            );
        }
    }
}