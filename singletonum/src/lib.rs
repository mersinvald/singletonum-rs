#[macro_use]
extern crate singletonum_derive;
extern crate once_cell;

pub use once_cell::sync::OnceCell;
pub use singletonum_derive::*;

pub trait Singleton: SingletonInit {
    fn get_instance(init: &Self::Init) -> &'static Self;
}

pub trait SingletonInit {
    type Init;
    fn init(init: &Self::Init) -> Self;
}
