
use std::sync::Arc;
use electrs::{
    config::Config,
};
#[macro_use]
extern crate log;

fn main(){
    let config = Arc::new(Config::from_args());
    debug!("config: {:?}", config);
}