#![feature(plugin, custom_derive)]
#![plugin(mockers_macros)]
#![cfg_attr(test, plugin(stainless))]

#[cfg(test)] extern crate mockers;
extern crate rl_sys;

mod pipe;

pub use pipe::AppFactory;
