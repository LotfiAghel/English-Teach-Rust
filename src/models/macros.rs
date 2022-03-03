//! Example: File Explorer
//! -------------------------
//!
//! This is a fun little desktop application that lets you explore the file system.
//!
//! This example is interesting because it's mixing filesystem operations and GUI, which is typically hard for UI to do.
use hello_macro_derive::HelloMacro;

extern crate json;
use curl::easy::Easy;
use futures::executor::block_on;
use parse_display::{Display, FromStr};
use serde::de::{self, DeserializeOwned};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::io::{stdout, Write};
use std::iter::Cloned;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{collections::HashMap, sync::RwLock};
use std::{env, result};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::models::BaseEntity::FrogienKey;

use super::BaseEntity::{BaseEntity, InheritsProvider,NameProvider};

pub async fn getCollection0<T1, T2>(item: &T1, cname: String) -> Vec<FrogienKey<T2>>
where
    T1: InheritsProvider<BaseEntity>,
    T2: InheritsProvider<BaseEntity>,
    T2: DeserializeOwned,
    T1:NameProvider,
{
    let pp = format!(
        "https://cs.karafsgym.com/v1/generic/{class_name}/{class_id}/{feild_name}",
        class_name = T1::get_name(),
        class_id = item.get_base().id,
        feild_name = cname
    );
    let body = reqwest::get(pp.as_str())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", body);
    let v: Value = serde_json::from_str(body.as_str()).unwrap();
    let mut res = Vec::<FrogienKey<T2>>::new();
    match v {
        Value::Null => todo!(),
        Value::Bool(_) => todo!(),
        Value::Number(_) => todo!(),
        Value::String(_) => todo!(),
        Value::Array(_) => {
            let x = v.as_array().unwrap();
            for i in x.into_iter() {
                let z = serde_json::from_value::<T2>(i.clone());
                match z {
                    Ok(_) => {
                        let zz = z.unwrap();
                        res.push(FrogienKey {
                            id: zz.get_base().id,
                            value: Some(Box::new(zz)),
                        })
                    }
                    Err(_) => todo!(),
                }
            }
        }
        Value::Object(_) => todo!(),
    }

    return res;
}

#[macro_export]
macro_rules! new_struct {


    // throw on another field (not the last one)
    ( ($parrent_name:tt) delmtr0 ( $(#[$attr:meta])* $id:ident: $ty:ty, $($next:tt)* ) delmtr1 {$($output:tt)*}) => {
        new_struct!( ($parrent_name) delmtr0 ($($next)*) delmtr1 {$($output)*  ( $(#[$attr])* $id: $ty)});
    };

    // throw on the last field
    ( ($parrent_name:tt) delmtr0 ($(#[$attr:meta])* $id:ident: $ty:ty) delmtr1 {$($output:tt)*}) => {
        new_struct!( ($parrent_name) delmtr0 () delmtr1 {$($output)* ($(#[$attr])* $id: $ty)});
    };

  

    // throw on the last field
    ( ($parrent_name:tt) delmtr0 ($id:ident :oneToMany[$ty:ty,$chache_name:ident], $($next:tt)*) delmtr1 {$($output:tt)*} ) => {
        new_struct!( ($parrent_name) delmtr0 ($($next)*) delmtr1 {$($output)* ($chache_name: Option<Vec<FrogienKey<$ty>>>)});
        impl $parrent_name{
            pub async fn $id(&mut self) -> &Vec<FrogienKey<$ty>> {
                match self.$chache_name {
                    Some(_) => self.$chache_name.as_ref().unwrap(),
                    None => {
                        //getCollection0::<Self,Session>(self,String::from("sessions")).await
                        self.$chache_name =
                            Some(getCollection0::<Self, $ty>(self, String::from(stringify!($id))).await);
                        self.$chache_name.as_ref().unwrap()
                    }
                }
            }
        }
    };
     // throw on the last field
    ( ($parrent_name:tt) delmtr0 ($id:ident :oneToMany[$ty:ty,$chache_name:ident]) delmtr1 {$($output:tt)*}) => {
        new_struct!( ($parrent_name) delmtr0 () delmtr1 {$($output)* ($chache_name: Option<Vec<Box<$ty>>>)});
        impl $parrent_name{
            fn $id(){println!("thaths right {}","stringify!(#id)")}
        }
    };


    // input is empty: time to output
    ( ($parrent_name:tt) delmtr0 () delmtr1 {$(#[$attr:meta])* $vis:vis struct $name:ident $((  $(#[$attr2:meta])* $id:ident: $ty:ty))*}) => {
        $(#[$attr])* $vis struct $name { $( $(#[$attr2])* $id: $ty),* }
    };




}

#[macro_export]
macro_rules! new_struct_with_impl {
        ($(#[$attr:meta])* $vis:vis struct $name:ident { $($input:tt)* } ) => {
            new_struct!( ($name) delmtr0 ($($input)*) delmtr1 {$(#[$attr])* $vis struct $name});
            //           ^^^^^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
            //               input       output
            impl NameProvider for $name{
                #[inline]
                fn get_name() -> &'static str {
                    return stringify!($name);
                }
            }
        }
}
