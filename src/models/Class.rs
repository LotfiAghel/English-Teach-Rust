use crate::new_struct;
use crate::new_struct_with_impl;

use hello_macro_derive::HelloMacro;

extern crate json;
use serde::{Deserialize, Serialize};



use super::BaseEntity::{BaseEntity, ForeignKey, HelloMacro, InheritsProvider, NameProvider};

use super::macros::getCollection0;


new_struct_with_impl! {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Debug, Clone)]
    #[derive(HelloMacro)]
    pub struct Class {
        #[serde(flatten)]
        base: BaseEntity,
        description: String,
        image_url: String, //sessions:ICollection<Session,Class,Class::sessions>
        sessions :oneToMany[Session,_orm_chache_sessions],


    }

}

impl InheritsProvider<BaseEntity> for Class {
    fn get_base(&self) -> &BaseEntity {
        &self.base
    }
}

new_struct_with_impl! {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Debug, Clone)]
    pub struct Session {
        #[serde(flatten)]
        base: BaseEntity,
        description: String,
        image_url: Option<String>,
    }
}

impl InheritsProvider<BaseEntity> for Session {
    fn get_base(&self) -> &BaseEntity {
        &self.base
    }
}
