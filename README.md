### Singleton Derive 
[![Crates.io](https://img.shields.io/crates/v/singletonum.svg)](https://crates.io/crates/singletonum)

[Documentation](https://docs.rs/crate/singletonum)

This crate provides a convenient API to make a singleton from any `struct` with just a `#[derive(Singleton)]`

### Example
```rust
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
```

### Contribution
This crate is not yet complete, suggestions for improvements are welcome in issues or via authors email.
PRs are welcome also.
