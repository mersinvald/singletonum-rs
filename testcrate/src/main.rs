extern crate singletonum;
use singletonum::{Singleton, SingletonInit};

#[derive(Debug, PartialEq, Singleton)]
struct SampleSingleton {
    inner: String,
}

impl SingletonInit for SampleSingleton {
    type Init = String;
    fn init(init: String) -> Self {
        SampleSingleton { inner: init }
    }
}

#[cfg(test)]
mod tests {
    use super::{SampleSingleton, Singleton};

    #[test]
    fn initialize_and_get() {
        SampleSingleton::get_instance(String::from("hello, world!"));
        let instance = SampleSingleton::get_instance(String::from("hello, world!"));
        assert_eq!(
            instance,
            &SampleSingleton { inner: String::from("hello, world!") }
        );
    }
}

fn main() {}
