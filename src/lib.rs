#[allow(clippy::all)]

mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

pub use protos::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_demo() {
        let mut get = demo::RequestGet::new();
        assert_eq!("", get.get_key())
    }
}
