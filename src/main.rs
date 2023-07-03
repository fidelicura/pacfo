#![allow(dead_code)]

mod package;
mod args;

use crate::{
    package::Package,
    args::App,
};



fn main() {
    App::new().args
        .into_iter()
        .for_each(|arg| {
            let pkg = Package::from(arg);
            println!("{}", pkg);
        })
}
