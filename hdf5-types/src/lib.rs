#![recursion_limit = "1024"]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::transmute_bytes_to_str))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::missing_safety_doc))]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod array;
mod h5type;
mod string;

pub use self::array::{Array, VarLenArray};
pub use self::h5type::{
    CompoundField, CompoundType, EnumMember, EnumType, FloatSize, H5Type, IntSize, TypeDescriptor,
};
pub use self::string::{FixedAscii, FixedUnicode, StringError, VarLenAscii, VarLenUnicode};
