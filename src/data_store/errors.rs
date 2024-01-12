use std::{
    num::{ParseIntError, TryFromIntError},
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum TypeConversionError {
    ParseIntError(ParseIntError),
    FromUtf8Error(FromUtf8Error),
    TryFromIntError(TryFromIntError),
    // Add other type cast error variants as needed
}

#[derive(Debug)]
pub enum TypeConversionImpossible {
    IncompatibleTypes,
    AsReference,
    AsMutable,
}

#[derive(Debug)]
pub enum ValueError {
    TypeConversionImpossible(TypeConversionImpossible),
    TypeConversionError(TypeConversionError),
}
