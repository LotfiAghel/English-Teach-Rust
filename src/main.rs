//! Example: File Explorer
//! -------------------------
//!
//! This is a fun little desktop application that lets you explore the file system.
//!
//! This example is interesting because it's mixing filesystem operations and GUI, which is typically hard for UI to do.
#![feature(trace_macros)]
#[macro_use]

use hello_macro_derive::HelloMacro;

extern crate json;
extern crate lazy_static;
use curl::easy::Easy;
use dioxus::prelude::*;
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
mod models;
use models::BaseEntity::{BaseEntity,InheritsProvider,ForeignKey,HelloMacro, NameProvider};
use models::Class::Class;


macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

#[tokio::main]
async fn main() {
    BaseEntity::hello_macro();
    say_hello!();
    
    let mut r: EntityManager<Class> = EntityManager::new();
    let mut idd = 2262;
    let mut class: &mut Class = r.get_by_async_id(&idd).await;

    class.sessions().await;

    // simple_logger::init_with_level(log::Level::Debug).unwrap();
    dioxus::desktop::launch_cfg(App, |c| {
        c.with_window(|w| {
            w.with_resizable(true).with_inner_size(
                dioxus::desktop::wry::application::dpi::LogicalSize::new(1200.0, 800.0),
            )
        })
    });
}

static App: Component<()> = |cx| {
    let files = use_ref(&cx, || Files::new());

    rsx!(cx, div {
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" }
        style { [include_str!("./style.css")] }
        header {
            i { class: "material-icons icon-menu", "menu" }
            h1 { "Files: " [files.read().current()] }
            span { }
            i { class: "material-icons", onclick: move |_| files.write().go_up(), "logout" }
        }
        main {
            files.read().path_names.iter().enumerate().map(|(dir_id, path)| {
                let path_end = path.split('/').last().unwrap_or(path.as_str());
                let icon_type = if path_end.contains(".") {
                    "description"
                } else {
                    "folder"
                };
                rsx! (
                    div { class: "folder", key: "{path}",
                        i { class: "material-icons",
                            onclick: move |_| files.write().enter_dir(dir_id),
                            "{icon_type}"
                            p { class: "cooltip", "0 folders / 0 files" }
                        }
                        h1 { "{path_end}" }
                    }
                )
            })
            files.read().err.as_ref().map(|err| {
                rsx! (
                    div {
                        code { "{err}" }
                        button { onclick: move |_| files.write().clear_err(), "x" }
                    }
                )
            })
        }

    })
};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Copy, Clone)]
struct User {
    #[serde(flatten)]
    base: BaseEntity,
}



impl InheritsProvider<BaseEntity> for User {
    fn get_base(&self) -> &BaseEntity {
        return &self.base;
    }
}








struct EntityManager<T2: InheritsProvider<BaseEntity>> {
    data: HashMap<i32, T2>,
}

impl<T2: InheritsProvider<BaseEntity>> EntityManager<T2>
where
    T2: DeserializeOwned,
    T2: Serialize,
    T2:NameProvider
{
    pub fn new() -> Self {
        //let mut sender2=sender.clone();
        let mut x = Self {
            data: HashMap::new(),
        };

        return x;
    }

    fn get_by_id(&self, i: &i32) -> &T2 {
        return &self.data[i];
    }

    async fn get_by_async_id(&mut self, i: &i32) -> &mut T2 {
        if self.data.contains_key(i) {
            return self.data.get_mut(i).unwrap();
        }
        //let z=self.chert.write().unwrap();

        let pp = format!(
            "https://cs.karafsgym.com/v1/generic/{class_name}/{class_id}",
            class_name = T2::get_name(),
            class_id = i
        );
        let body = reqwest::get(pp.as_str())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{}", body);
        let deserialized_data = serde_json::from_str::<T2>(body.as_str()).unwrap();
        println!("{}", serde_json::to_string(&deserialized_data).unwrap());
        self.data.insert(deserialized_data.get_base().id,deserialized_data);
            //.entry(deserialized_data.get_base().id)
            //.insert_entry(deserialized_data);

        return self.data.get_mut(i).unwrap();
    }
}

struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    err: Option<String>,
}

impl Files {
    fn new() -> Self {
        let mut files = Self {
            path_stack: vec!["./".to_string()],
            path_names: vec![],
            err: None,
        };

        files.reload_path_list();

        files
    }

    fn reload_path_list(&mut self) {
        let cur_path = self.path_stack.last().unwrap();
        log::info!("Reloading path list for {:?}", cur_path);
        let paths = match std::fs::read_dir(cur_path) {
            Ok(e) => e,
            Err(err) => {
                let err = format!("An error occured: {:?}", err);
                self.err = Some(err);
                self.path_stack.pop();
                return;
            }
        };
        let collected = paths.collect::<Vec<_>>();
        log::info!("Path list reloaded {:#?}", collected);

        // clear the current state
        self.clear_err();
        self.path_names.clear();

        for path in collected {
            self.path_names
                .push(path.unwrap().path().display().to_string());
        }
        log::info!("path namees are {:#?}", self.path_names);
    }

    fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }

    fn enter_dir(&mut self, dir_id: usize) {
        let path = &self.path_names[dir_id];
        self.path_stack.push(path.clone());
        self.reload_path_list();
    }

    fn current(&self) -> &str {
        self.path_stack.last().unwrap()
    }
    fn clear_err(&mut self) {
        self.err = None;
    }
}
