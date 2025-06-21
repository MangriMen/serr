#![allow(dead_code)]
use serr::SerializeError;
use serr::ToSerializedError;

#[derive(Debug, thiserror::Error, SerializeError)]
enum TestError {
    #[error("Simple variant")]
    SimpleVariant,
}

fn main() {
    let err = TestError::SimpleVariant;
    let serial = err.to_serialized();

    assert_eq!(serial.code, "simpleVariant");
    assert_eq!(serial.fields, None);
}
