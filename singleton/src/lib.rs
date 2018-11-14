#[macro_use]
extern crate singleton_derive;
extern crate once_cell;

pub use singleton_derive::*;
pub use once_cell::sync::OnceCell;

pub trait Singleton: SingletonInit {
    fn get_instance(init: Self::Init) -> &'static Self;
}

pub trait SingletonInit {
    type Init;
    fn init(init: Self::Init) -> Self;
}
