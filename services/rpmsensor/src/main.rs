#![feature(proc_macro_hygiene, decl_macro)]

#![allow(unused)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod config;

fn main() {
    config::rocket().launch();
}
