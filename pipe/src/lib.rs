#![feature(plugin, custom_derive)]
#![plugin(mockers_macros)]

#[cfg(test)] extern crate mockers;
extern crate rl_sys;

mod pipe;

pub use pipe::AppFactory;
