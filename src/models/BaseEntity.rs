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

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Copy, Clone, Display, HelloMacro)]
pub struct BaseEntity {
    pub id: i32,
}

pub trait InheritsProvider<T> {
    fn get_base(&self) -> &T;
}
pub trait NameProvider {
    fn get_name() -> &'static str;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForeignKey<T: InheritsProvider<BaseEntity>> {
    pub id: i32,
    pub value: Option<Box<T>>,
}
