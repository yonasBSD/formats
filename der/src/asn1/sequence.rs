//! The [`Sequence`] trait simplifies writing decoders/encoders which map ASN.1
//! `SEQUENCE`s to Rust structs.

use crate::{
    BytesRef, DecodeValue, EncodeValue, Error, FixedTag, Header, Length, Reader, Result, Tag,
    Writer,
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Marker trait for ASN.1 `SEQUENCE`s.
///
/// This is mainly used for custom derive.
pub trait Sequence<'a>: DecodeValue<'a> + EncodeValue {}

impl<'a, S> FixedTag for S
where
    S: Sequence<'a>,
{
    const TAG: Tag = Tag::Sequence;
}

#[cfg(feature = "alloc")]
impl<'a, T> Sequence<'a> for Box<T> where T: Sequence<'a> {}

/// The [`SequenceRef`] type provides raw access to the octets which comprise a
/// DER-encoded `SEQUENCE`.
///
/// This is a zero-copy reference type which borrows from the input data.
pub struct SequenceRef<'a> {
    /// Body of the `SEQUENCE`.
    body: &'a BytesRef,
}

impl<'a> SequenceRef<'a> {
    /// Borrow the inner byte slice.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.body.as_slice()
    }
}

impl AsRef<[u8]> for SequenceRef<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> DecodeValue<'a> for SequenceRef<'a> {
    type Error = Error;

    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        Ok(Self {
            body: <&'a BytesRef>::decode_value(reader, header)?,
        })
    }
}

impl EncodeValue for SequenceRef<'_> {
    fn value_len(&self) -> Result<Length> {
        Ok(self.body.len())
    }

    fn encode_value(&self, writer: &mut impl Writer) -> Result<()> {
        self.body.encode_value(writer)
    }
}

impl<'a> Sequence<'a> for SequenceRef<'a> {}
