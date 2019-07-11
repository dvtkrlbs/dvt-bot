#![feature(result_map_or_else)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

pub mod core;
pub mod macros;
pub mod modules;

use serenity::framework::StandardFramework;

pub struct DvtBot {}
