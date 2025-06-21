#![allow(dead_code)]
use serr::SerializeError;
use serr::ToSerializedError;

#[derive(Debug, thiserror::Error, SerializeError)]
#[serialize_error(name = "test")]
enum TestError {
    #[error("Simple variant")]
    SimpleVariant,
}

fn main() {
    let err = TestError::SimpleVariant;
    let serial = err.to_serialized();

    assert_eq!(serial.code, "test.simpleVariant");
    assert_eq!(serial.fields, None);
}
