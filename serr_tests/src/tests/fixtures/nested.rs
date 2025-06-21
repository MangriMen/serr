#![allow(dead_code)]
use serr::SerializeError;
use serr::ToSerializedError;

#[derive(Debug, thiserror::Error, SerializeError)]
enum TestErrorWithNested {
    #[error("Simple variant")]
    SimpleVariant,
    #[error(transparent)]
    #[serialize_error]
    NestedError(#[from] NestedError),
}

#[derive(Debug, thiserror::Error, SerializeError)]
enum NestedError {
    #[error("Simple variant")]
    SimpleVariant,
}

fn main() {
    let err = TestErrorWithNested::NestedError(NestedError::SimpleVariant);
    let serial = err.to_serialized();

    assert_eq!(serial.code, "nestedError.simpleVariant");
    assert_eq!(serial.fields, None);
}
