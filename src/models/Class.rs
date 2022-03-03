use crate::new_struct;
use crate::new_struct_with_impl;
#[macro_use]
use super::macros;
use hello_macro_derive::HelloMacro;

extern crate json;
use parse_display::{Display, FromStr};
use serde::de::{self, DeserializeOwned};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::{collections::HashMap, sync::RwLock};

use super::BaseEntity::{BaseEntity, FrogienKey, HelloMacro, InheritsProvider, NameProvider};

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
        return &self.base;
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
        todo!()
    }
}
